use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{ready, Context, Poll},
};

use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use parking_lot::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::database::{entity::device::DeviceId, DbPool};

use super::{protocol::ServerDeviceMessage, Devices};

pub type DeviceSessionId = Uuid;
pub type DeviceSessionRef = Arc<DeviceSession>;

pub struct DeviceSession {
    /// Unique ID of the session
    id: DeviceSessionId,
    /// Session state
    state: RwLock<DeviceSessionState>,

    devices: Devices,
}

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

    read_state: ReadState,
    write_state: WriteState,
}

enum WriteState {
    /// Waiting for an outbound message
    Receive,

    /// Write a message
    Write(Message),

    /// Flush a message
    Flush,
}

enum ReadState {
    /// Waiting for an incoming message
    Receive,
}

impl DeviceSocketFuture {
    fn poll_read_message(&mut self, cx: &mut Context<'_>) -> Poll<bool> {
        let msg = match ready!(self.socket.poll_next_unpin(cx)) {
            Some(msg) => msg,
            None => return Poll::Ready(false),
        };

        Poll::Pending
    }
}

impl Future for DeviceSocketFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        Poll::Pending
    }
}
