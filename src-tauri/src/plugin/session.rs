use crate::{
    database::entity::tile::UpdateKind,
    tile::Tiles,
    utils::{
        error::try_cast_error,
        ws::{WebSocketFuture, WsTx},
    },
};
use axum::extract::ws::WebSocket;
use parking_lot::RwLock;
use std::{io::ErrorKind, sync::Arc};
use tauri::async_runtime::{spawn, spawn_blocking};
use tauri_plugin_opener::open_url;
use tracing::{debug, error};
use uuid::Uuid;

use super::{
    Plugins,
    manifest::PluginId,
    protocol::{ClientPluginMessage, ServerPluginMessage},
};

pub type PluginSessionId = Uuid;

pub type PluginSessionRef = Arc<PluginSession>;

pub struct PluginSession {
    /// Unique ID of the session
    id: PluginSessionId,
    /// Session state
    state: RwLock<PluginSessionState>,
    /// Sender to send messages to the session socket
    tx: WsTx<ServerPluginMessage>,
    /// Access to the plugins registry the session is apart of
    plugins: Arc<Plugins>,
    /// Access to work with tiles
    tiles: Arc<Tiles>,
}

#[derive(Default)]
struct PluginSessionState {
    /// Device ID if authenticated as a device
    plugin_id: Option<PluginId>,
}

impl PluginSession {
    pub fn start(plugins: Arc<Plugins>, tiles: Arc<Tiles>, socket: WebSocket) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) =
            WebSocketFuture::<ServerPluginMessage, ClientPluginMessage>::new(socket);

        spawn(async move {
            if let Err(cause) = ws_future.await {
                // Handle plugin connection lost as just a warning
                if let Some(cause_io) = try_cast_error::<std::io::Error>(&cause) {
                    if cause_io.kind() == ErrorKind::ConnectionReset {
                        tracing::warn!(?cause_io, "plugin connection closed");
                        return;
                    }
                }

                error!(?cause, "error running plugin websocket future");
            }
        });

        let session = Arc::new(PluginSession {
            id,
            state: Default::default(),
            plugins,
            tiles,
            tx: ws_tx,
        });

        spawn(async move {
            // Add the session
            session.plugins.insert_session(id, session.clone());

            let mut ws_rx = ws_rx;

            // Process messages from the session
            while let Some(msg) = ws_rx.recv().await {
                session.handle_message(msg).await;
            }

            let plugin_id = session.get_plugin_id();

            // Remove the session thats no longer running
            session.plugins.remove_session(session.id, plugin_id);
        });
    }

    /// Get the current plugin ID
    pub fn get_plugin_id(&self) -> Option<PluginId> {
        self.state.read().plugin_id.clone()
    }

    /// Send a message to the plugin session
    pub fn send_message(&self, msg: ServerPluginMessage) -> bool {
        self.tx.send(msg).is_ok()
    }

    /// Handle messages from the socket
    pub async fn handle_message(&self, message: ClientPluginMessage) {
        match self.get_plugin_id() {
            Some(plugin_id) => self.handle_message_authenticated(plugin_id, message).await,
            None => self.handle_message_unauthenticated(message).await,
        };
    }

    /// Handle messages when unauthenticated
    pub async fn handle_message_unauthenticated(&self, message: ClientPluginMessage) {
        match message {
            ClientPluginMessage::RegisterPlugin { plugin_id } => {
                // Handle unknown plugin
                if self.plugins.get_plugin(&plugin_id).is_none() {
                    debug!(?plugin_id, "plugin registered with unknown id");
                    return;
                }

                self.plugins.set_plugin_session(plugin_id.clone(), self.id);

                // Set the current plugin ID
                {
                    self.state.write().plugin_id = Some(plugin_id.clone());
                }

                self.send_message(ServerPluginMessage::Registered { plugin_id });
            }
            message => {
                tracing::warn!(?message, "got unexpected message from unauthorized plugin");
            }
        }
    }

    /// Handle message when authenticated as `plugin_id`
    pub async fn handle_message_authenticated(
        &self,
        plugin_id: PluginId,
        message: ClientPluginMessage,
    ) {
        match message {
            ClientPluginMessage::GetProperties => {
                let properties = match self.plugins.get_plugin_properties(plugin_id).await {
                    Ok(value) => value,
                    Err(cause) => {
                        tracing::error!(?cause, "failed to load plugin properties");
                        return;
                    }
                };

                self.send_message(ServerPluginMessage::Properties { properties });
            }

            ClientPluginMessage::SetProperties {
                properties,
                partial,
            } => {
                if let Err(cause) = self
                    .plugins
                    .set_plugin_properties(plugin_id, properties, partial)
                    .await
                {
                    tracing::error!(?cause, "failed to save plugin properties");
                }
            }

            ClientPluginMessage::SendToInspector { ctx, message } => {
                self.plugins.send_to_inspector(ctx, message);
            }

            ClientPluginMessage::OpenUrl { url } => {
                _ = spawn_blocking(move || {
                    if let Err(cause) = open_url(url, None::<&str>) {
                        tracing::error!(?cause, "failed to open url");
                    }
                })
                .await;
            }

            ClientPluginMessage::GetTileProperties { tile_id } => {
                let properties = match self
                    .tiles
                    .get_tile_properties(tile_id, Some(plugin_id))
                    .await
                {
                    Ok(value) => value,
                    Err(cause) => {
                        tracing::error!(?cause, "failed to get tile properties");
                        return;
                    }
                };

                self.send_message(ServerPluginMessage::TileProperties {
                    tile_id,
                    properties,
                });
            }

            ClientPluginMessage::SetTileProperties {
                tile_id,
                properties,
                partial,
            } => {
                if let Err(cause) = self
                    .tiles
                    .update_tile_properties(tile_id, Some(plugin_id), properties, partial)
                    .await
                {
                    tracing::error!(?cause, "failed to save tile properties");
                }
            }

            ClientPluginMessage::SetTileIcon { tile_id, icon } => {
                if let Err(cause) = self
                    .tiles
                    .update_tile_icon(tile_id, Some(plugin_id), icon, UpdateKind::Program)
                    .await
                {
                    tracing::error!(?cause, "failed to save tile icon");
                }
            }

            ClientPluginMessage::SetTileLabel { tile_id, label } => {
                if let Err(cause) = self
                    .tiles
                    .update_tile_label(tile_id, Some(plugin_id), label, UpdateKind::Program)
                    .await
                {
                    tracing::error!(?cause, "failed to save tile label");
                }
            }

            message => {
                tracing::warn!(?message, "got unexpected message from authorized plugin");
            }
        }
    }
}
