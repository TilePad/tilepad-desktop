use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use crate::{
    events::{AppEvent, PluginAppEvent},
    utils::ws::{WebSocketFuture, WsRx, WsTx},
};
use anyhow::anyhow;
use axum::extract::ws::{Message, WebSocket};
use parking_lot::RwLock;
use serde_json::{Map, Value};
use tauri::async_runtime::spawn;
use tauri_plugin_opener::open_url;
use tracing::{debug, error};
use uuid::Uuid;

use super::{
    manifest::PluginId,
    protocol::{ClientPluginMessage, ServerPluginMessage},
    Plugins,
};

pub type PluginSessionId = Uuid;

pub type PluginSessionRef = Arc<PluginSession>;

pub struct PluginSession {
    /// Unique ID of the session
    id: PluginSessionId,
    /// Session state
    state: RwLock<PluginSessionState>,
    /// Access to the plugins registry
    plugins: Arc<Plugins>,
    /// Sender to send messages to the session socket
    tx: WsTx<ServerPluginMessage>,
}

#[derive(Default)]
struct PluginSessionState {
    /// Device ID if authenticated as a device
    plugin_id: Option<PluginId>,
}

impl PluginSession {
    pub fn start(plugins: Arc<Plugins>, socket: WebSocket) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) =
            WebSocketFuture::<ServerPluginMessage, ClientPluginMessage>::new(socket);

        spawn(async move {
            if let Err(cause) = ws_future.await {
                error!(?cause, "error running device websocket future");
            }
        });

        let session = Arc::new(PluginSession {
            id,
            state: Default::default(),
            plugins,
            tx: ws_tx,
        });

        spawn(async move {
            // Add the session
            session.plugins.insert_session(id, session.clone());

            let mut ws_rx = ws_rx;

            // Process messages from the session
            while let Some(msg) = ws_rx.recv().await {
                session.handle_message(msg);
            }

            // Remove the session thats no longer running
            session.plugins.remove_session(session.id);

            if let Some(plugin_id) = session.get_plugin_id() {
                session.plugins.remove_plugin_session(plugin_id);
            }
        });
    }

    pub fn get_plugin_id(&self) -> Option<PluginId> {
        self.state.read().plugin_id.clone()
    }

    pub fn set_plugin_id(&self, plugin_id: Option<PluginId>) {
        self.state.write().plugin_id = plugin_id;
    }

    pub fn send_message(&self, msg: ServerPluginMessage) -> bool {
        self.tx.send(msg).is_ok()
    }

    pub fn handle_message(self: &Arc<Self>, message: ClientPluginMessage) {
        match message {
            ClientPluginMessage::RegisterPlugin { plugin_id } => {
                // Handle unknown plugin
                if self.plugins.get_plugin(&plugin_id).is_none() {
                    debug!(?plugin_id, "plugin registered with unknown id");
                    return;
                }

                self.plugins.set_plugin_session(plugin_id.clone(), self.id);
                self.set_plugin_id(Some(plugin_id.clone()));

                self.send_message(ServerPluginMessage::Registered { plugin_id });
            }
            ClientPluginMessage::GetProperties => {
                let plugin_id = match self.get_plugin_id() {
                    Some(value) => value,
                    None => {
                        debug!("plugin requested properties before registering");
                        return;
                    }
                };

                let session = self.clone();
                tokio::spawn(async move {
                    let properties = match session.plugins.get_plugin_properties(plugin_id).await {
                        Ok(value) => value,
                        Err(cause) => {
                            error!(?cause, "failed to load plugin properties");
                            return;
                        }
                    };

                    session.send_message(ServerPluginMessage::Properties { properties });
                });
            }
            ClientPluginMessage::SendToInspector { ctx, message } => {
                self.plugins.send_to_inspector(ctx, message);
            }
            ClientPluginMessage::SetProperties { properties } => {
                let plugin_id = match self.get_plugin_id() {
                    Some(value) => value,
                    None => return,
                };

                let session = self.clone();
                tokio::spawn(async move {
                    if let Err(cause) = session
                        .plugins
                        .set_plugin_properties(plugin_id, properties)
                        .await
                    {
                        error!(?cause, "failed to save plugin properties");
                    }
                });
            }
            ClientPluginMessage::OpenUrl { url } => {
                if let Err(cause) = open_url(url, None::<&str>) {
                    error!(?cause, "failed to open url");
                }
            }
        }
    }
}
