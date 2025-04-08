use std::{net::SocketAddr, sync::Arc};

use axum::extract::ws::WebSocket;
use parking_lot::RwLock;
use tauri::async_runtime::spawn;
use tracing::error;
use uuid::Uuid;

use crate::{
    database::entity::device::DeviceId,
    utils::ws::{WebSocketFuture, WsTx},
};

use super::{
    protocol::{ClientDeviceMessage, ServerDeviceMessage},
    Devices,
};

pub type DeviceSessionId = Uuid;
pub type DeviceSessionRef = Arc<DeviceSession>;

pub struct DeviceSession {
    /// Unique ID of the session
    id: DeviceSessionId,

    /// Address of the device session socket
    socket_addr: SocketAddr,

    /// Session state
    state: RwLock<DeviceSessionState>,

    /// Channel to send messages to the session
    tx: WsTx<ServerDeviceMessage>,

    devices: Arc<Devices>,
}

#[derive(Default)]
pub struct DeviceSessionState {
    /// Device ID if authenticated as a device
    device_id: Option<DeviceId>,
}

impl DeviceSession {
    pub fn new(
        devices: Arc<Devices>,
        socket_addr: SocketAddr,
        socket: WebSocket,
    ) -> (DeviceSessionId, DeviceSessionRef) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) =
            WebSocketFuture::<ServerDeviceMessage, ClientDeviceMessage>::new(socket);
        spawn(async move {
            if let Err(cause) = ws_future.await {
                error!(?cause, "error running device websocket future");
            }
        });

        let session = Arc::new(DeviceSession {
            id,
            socket_addr,
            state: Default::default(),
            tx: ws_tx,
            devices,
        });

        spawn({
            let session = session.clone();

            async move {
                let mut ws_rx = ws_rx;

                // Process messages from the session
                while let Some(msg) = ws_rx.recv().await {
                    session.handle_message(msg).await;
                }

                let device_id = session.get_device_id();

                // Remove the session thats no longer running
                session.devices.remove_session(session.id, device_id);
            }
        });

        (id, session)
    }

    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }

    pub fn get_device_id(&self) -> Option<DeviceId> {
        self.state.read().device_id
    }

    pub fn set_device_id(&self, device_id: Option<DeviceId>) {
        self.state.write().device_id = device_id;
    }

    pub fn send_message(&self, msg: ServerDeviceMessage) -> bool {
        self.tx.send(msg).is_ok()
    }

    /// Handle messages from the socket
    pub async fn handle_message(&self, message: ClientDeviceMessage) {
        match self.get_device_id() {
            Some(device_id) => self.handle_message_authenticated(device_id, message).await,
            None => self.handle_message_unauthenticated(message).await,
        };
    }

    /// Handle messages when unauthenticated
    pub async fn handle_message_unauthenticated(&self, message: ClientDeviceMessage) {
        match message {
            ClientDeviceMessage::RequestApproval { name } => {
                self.devices
                    .add_device_request(self.id, self.socket_addr, name);
            }

            ClientDeviceMessage::Authenticate { access_token } => {
                if let Err(cause) = self
                    .devices
                    .attempt_authenticate_device(self.id, access_token)
                    .await
                {
                    tracing::error!(?cause, "failed to authenticate device");
                }
            }

            message => {
                tracing::warn!(?message, "got unexpected message from unauthorized device");
            }
        }
    }

    /// Handle message when authenticated as `device_id`
    pub async fn handle_message_authenticated(
        &self,
        device_id: DeviceId,
        message: ClientDeviceMessage,
    ) {
        match message {
            ClientDeviceMessage::RequestTiles => {
                // Get the current folder the device is using
                let (folder, tiles) = match self.devices.request_device_tiles(device_id).await {
                    Ok(value) => value,
                    Err(cause) => {
                        tracing::error!(?cause, "failed to request device tiles");
                        return;
                    }
                };

                // Send the tiles to the device
                self.send_message(ServerDeviceMessage::Tiles { tiles, folder });
            }

            ClientDeviceMessage::TileClicked { tile_id } => {
                let devices = self.devices.clone();

                _ = tokio::spawn(async move {
                    if let Err(cause) = devices.device_execute_tile(device_id, tile_id).await {
                        tracing::error!(?cause, "failed to execute tile");
                    }
                });
            }

            message => {
                tracing::warn!(?message, "got unexpected message from authorized device");
            }
        }
    }
}
