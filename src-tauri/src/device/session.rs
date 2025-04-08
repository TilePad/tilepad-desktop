use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use anyhow::anyhow;
use axum::extract::ws::{Message, WebSocket};
use parking_lot::RwLock;
use tauri::async_runtime::spawn;
use tracing::error;
use uuid::Uuid;

use crate::{
    database::entity::device::DeviceId,
    utils::ws::{WebSocketFuture, WsRx, WsTx},
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

    tx: WsTx,

    devices: Devices,
}

impl DeviceSession {
    pub fn new(
        devices: Devices,
        socket_addr: SocketAddr,
        socket: WebSocket,
    ) -> (DeviceSessionId, DeviceSessionRef) {
        let id = Uuid::new_v4();

        // Create and spawn a future for the websocket
        let (ws_future, ws_rx, ws_tx) = WebSocketFuture::new(socket);
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

        // Create and spawn a future to process session messages
        let session_future = DeviceSessionFuture {
            session: session.clone(),
            rx: ws_rx,
        };

        spawn({
            let session = session.clone();

            async move {
                // Run the session to completion
                if let Err(cause) = session_future.await {
                    error!(?cause, "error running device session future");
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

    pub fn send_message(&self, msg: ServerDeviceMessage) -> anyhow::Result<()> {
        let msg = serde_json::to_string(&msg)?;
        let message = Message::text(msg);
        self.tx.send(message)?;
        Ok(())
    }

    pub fn handle_message(self: &Arc<Self>, message: ClientDeviceMessage) {
        match message {
            ClientDeviceMessage::RequestApproval { name } => {
                self.devices
                    .add_device_request(self.id, self.socket_addr, name);
            }

            ClientDeviceMessage::Authenticate { access_token } => {
                let session_id = self.id;
                let devices = self.devices.clone();

                _ = tokio::spawn(async move {
                    if let Err(cause) = devices
                        .attempt_authenticate_device(session_id, access_token)
                        .await
                    {
                        tracing::error!(?cause, "failed to authenticate device");
                    }
                });
            }

            ClientDeviceMessage::RequestTiles => {
                let session = self.clone();
                let device_id = match self.get_device_id() {
                    Some(value) => value,
                    None => {
                        tracing::error!("unauthenticated device requested tiles");
                        return;
                    }
                };
                let devices = self.devices.clone();

                _ = tokio::spawn(async move {
                    let (folder, tiles) = match devices.request_device_tiles(device_id).await {
                        Ok(value) => value,
                        Err(cause) => {
                            tracing::error!(?cause, "failed to request device tiles");
                            return;
                        }
                    };

                    if let Err(cause) =
                        session.send_message(ServerDeviceMessage::Tiles { tiles, folder })
                    {
                        tracing::error!(?cause, "failed to send device tiles");
                    }
                });
            }

            ClientDeviceMessage::TileClicked { tile_id } => {
                let device_id = match self.get_device_id() {
                    Some(value) => value,
                    None => {
                        tracing::error!("unauthenticated device requested tiles");
                        return;
                    }
                };
                let devices = self.devices.clone();

                _ = tokio::spawn(async move {
                    if let Err(cause) = devices.device_execute_tile(device_id, tile_id).await {
                        tracing::error!(?cause, "failed to execute tile");
                    }
                });
            }
        }
    }
}

#[derive(Default)]
pub struct DeviceSessionState {
    /// Device ID if authenticated as a device
    device_id: Option<DeviceId>,
}

/// Futures that processes messages for a device session
pub struct DeviceSessionFuture {
    session: DeviceSessionRef,
    rx: WsRx,
}

impl Future for DeviceSessionFuture {
    type Output = anyhow::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        while let Some(msg) = ready!(this.rx.poll_recv(cx)) {
            let message = match msg {
                Message::Text(utf8_bytes) => utf8_bytes,

                // Ping and pong are handled internally
                Message::Ping(_) | Message::Pong(_) => continue,

                // Expecting a text based protocol
                Message::Binary(_) => {
                    return Poll::Ready(Err(anyhow!("unexpected binary message")))
                }

                // Socket is closed
                Message::Close(_) => return Poll::Ready(Ok(())),
            };

            let msg: ClientDeviceMessage = serde_json::from_str(message.as_str())?;
            this.session.handle_message(msg);
        }

        // No more messages, session is terminated
        Poll::Ready(Ok(()))
    }
}
