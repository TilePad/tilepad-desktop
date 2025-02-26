use std::{
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use anyhow::anyhow;
use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use parking_lot::RwLock;
use tauri::async_runtime::spawn;
use tokio::sync::mpsc;
use tracing::error;
use uuid::Uuid;

use crate::database::entity::device::DeviceId;

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

    tx: DeviceSessionSender,

    devices: Devices,
}

impl DeviceSession {
    pub fn new(
        devices: Devices,
        socket_addr: SocketAddr,
        socket: WebSocket,
    ) -> (DeviceSessionId, DeviceSessionRef) {
        let id = Uuid::new_v4();

        let (tx, rx) = mpsc::unbounded_channel();

        let session = Arc::new(DeviceSession {
            id,
            socket_addr,
            state: Default::default(),
            tx: DeviceSessionSender { tx },
            devices,
        });

        // Spawn the future to handle the socket
        spawn(DeviceSocketFuture {
            rx,
            session: session.clone(),
            socket,
            write_state: WriteState::Receive,
        });

        (id, session)
    }

    pub fn is_closed(&self) -> bool {
        self.tx.tx.is_closed()
    }

    pub fn get_device_id(&self) -> Option<DeviceId> {
        self.state.read().device_id
    }

    pub fn set_device_id(&self, device_id: Option<DeviceId>) {
        self.state.write().device_id = device_id;
    }

    pub fn send_approved(&self, device_id: DeviceId, access_token: String) -> anyhow::Result<()> {
        self.tx.tx.send(ServerDeviceMessage::Approved {
            device_id,
            access_token,
        })?;
        Ok(())
    }

    pub fn send_authenticated(&self) -> anyhow::Result<()> {
        self.tx.tx.send(ServerDeviceMessage::Authenticated)?;
        Ok(())
    }

    pub fn send_invalid_access_token(&self) -> anyhow::Result<()> {
        self.tx.tx.send(ServerDeviceMessage::InvalidAccessToken)?;
        Ok(())
    }

    pub fn send_declined(&self) -> anyhow::Result<()> {
        self.tx.tx.send(ServerDeviceMessage::Declined)?;
        Ok(())
    }

    pub fn send_revoked(&self) -> anyhow::Result<()> {
        self.tx.tx.send(ServerDeviceMessage::Revoked)?;
        Ok(())
    }

    pub fn handle_message(&self, message: ClientDeviceMessage) {
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

            ClientDeviceMessage::RequestProfile => {
                // TODO: Request the device profile data, respond with new profile data
            }

            ClientDeviceMessage::TileClicked { tile_id } => {
                // TODO: Forward on click
            }
        }
    }
}

#[derive(Default)]
pub struct DeviceSessionState {
    /// Device ID if authenticated as a device
    device_id: Option<DeviceId>,
}

/// Sender to send messages from the server to the device
#[derive(Clone)]
pub struct DeviceSessionSender {
    tx: mpsc::UnboundedSender<ServerDeviceMessage>,
}

pub struct DeviceSocketFuture {
    session: DeviceSessionRef,

    socket: WebSocket,

    rx: mpsc::UnboundedReceiver<ServerDeviceMessage>,

    write_state: WriteState,
}

impl Drop for DeviceSocketFuture {
    fn drop(&mut self) {
        self.session.devices.remove_session(self.session.id);
    }
}

enum WriteState {
    /// Waiting for an outbound message
    Receive,

    /// Write a message
    Write(Option<Message>),

    /// Flush a message
    Flush,
}

/// Possible outcomes from polling the read state
enum PollReadOutcome {
    /// Encountered an error
    Error(anyhow::Error),

    /// Encountered a no more messages state
    Closed,

    /// Encountered a continue state (Read ping/pong message)
    Continue,

    /// Read an actual device message
    Message(ClientDeviceMessage),
}

enum PollWriteOutcome {
    /// Encountered an error
    Error(anyhow::Error),

    /// Connection is closed
    Closed,

    /// Continue to next write
    Continue,
}

impl DeviceSocketFuture {
    fn poll_read_message(&mut self, cx: &mut Context<'_>) -> Poll<PollReadOutcome> {
        let msg = match ready!(self.socket.poll_next_unpin(cx)) {
            Some(Ok(msg)) => msg,

            // Reading has errored
            Some(Err(err)) => return Poll::Ready(PollReadOutcome::Error(anyhow::Error::new(err))),

            // Socket has no more messages, socket has closed
            None => return Poll::Ready(PollReadOutcome::Closed),
        };

        let message = match msg {
            Message::Text(utf8_bytes) => utf8_bytes,

            // Ping and pong are handled internally
            Message::Ping(_) | Message::Pong(_) => return Poll::Ready(PollReadOutcome::Continue),

            // Expecting a text based protocol
            Message::Binary(_) => {
                return Poll::Ready(PollReadOutcome::Error(anyhow!("unexpected binary message")))
            }

            // Socket is closed
            Message::Close(_) => return Poll::Ready(PollReadOutcome::Closed),
        };

        let msg: ClientDeviceMessage = match serde_json::from_str(message.as_str()) {
            Ok(value) => value,

            // Failed to deserialize message
            Err(err) => return Poll::Ready(PollReadOutcome::Error(anyhow::Error::new(err))),
        };

        Poll::Ready(PollReadOutcome::Message(msg))
    }

    fn poll_write_message(&mut self, cx: &mut Context<'_>) -> Poll<PollWriteOutcome> {
        match &mut self.write_state {
            WriteState::Receive => {
                // Receive a message
                let result = match ready!(self.rx.poll_recv(cx)) {
                    Some(value) => value,
                    None => return Poll::Ready(PollWriteOutcome::Closed),
                };

                // Encode the message
                let encoded = match serde_json::to_string(&result) {
                    Ok(value) => value,
                    Err(err) => {
                        return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)))
                    }
                };

                // Move to next state
                let message = Message::text(encoded);
                self.write_state = WriteState::Write(Some(message));

                Poll::Ready(PollWriteOutcome::Continue)
            }
            WriteState::Write(message) => {
                if let Err(err) = ready!(self.socket.poll_ready_unpin(cx)) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                let packet = message
                    .take()
                    .expect("unexpected write state without a packet");

                if let Err(err) = self.socket.start_send_unpin(packet) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                self.write_state = WriteState::Flush;

                Poll::Ready(PollWriteOutcome::Continue)
            }
            WriteState::Flush => {
                if let Err(err) = ready!(self.socket.poll_flush_unpin(cx)) {
                    return Poll::Ready(PollWriteOutcome::Error(anyhow::Error::new(err)));
                }

                self.write_state = WriteState::Receive;

                Poll::Ready(PollWriteOutcome::Continue)
            }
        }
    }
}

impl Future for DeviceSocketFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        while let Poll::Ready(outcome) = this.poll_read_message(cx) {
            match outcome {
                // Continue to the next message
                PollReadOutcome::Continue => continue,

                PollReadOutcome::Error(cause) => {
                    error!(?cause, "device socket encountered error while reading");

                    // Stop the socket since we are in an error state
                    return Poll::Ready(());
                }
                PollReadOutcome::Message(message) => {
                    this.session.handle_message(message);
                }

                // Socket has closed, finish the future
                PollReadOutcome::Closed => return Poll::Ready(()),
            }
        }

        while let Poll::Ready(outcome) = this.poll_write_message(cx) {
            match outcome {
                PollWriteOutcome::Continue => continue,
                PollWriteOutcome::Error(cause) => {
                    error!(?cause, "device socket encountered error while writing");

                    // Stop the socket since we are in an error state
                    return Poll::Ready(());
                }
                PollWriteOutcome::Closed => return Poll::Ready(()),
            }
        }

        Poll::Pending
    }
}
