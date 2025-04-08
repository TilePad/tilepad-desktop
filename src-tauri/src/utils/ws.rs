use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;
use tokio::sync::mpsc;

/// Abstraction for easily implementing a JSON protocol
/// on top of a websocket connection, manages the underlying
/// reading and writing within a self contained future
///
/// Provides a `inbound_tx` and `outbound_tx`` channel for sending and
/// receiving messages.
pub struct WebSocketFuture<MsgOut, MsgIn> {
    /// Socket we are acting upon
    socket: WebSocket,
    /// Channel for processing received messages
    inbound_tx: Option<mpsc::UnboundedSender<MsgIn>>,
    /// Channel for outbound messages
    outbound_rx: mpsc::UnboundedReceiver<MsgOut>,
    /// Currently accepted outbound item, ready to be written
    buffered_item: Option<WsMessage>,
}

pub type WsTx<M> = mpsc::UnboundedSender<M>;
pub type WsRx<M> = mpsc::UnboundedReceiver<M>;

impl<MsgOut, MsgIn> WebSocketFuture<MsgOut, MsgIn>
where
    MsgOut: Serialize,
    MsgIn: DeserializeOwned,
{
    pub fn new(socket: WebSocket) -> (WebSocketFuture<MsgOut, MsgIn>, WsRx<MsgIn>, WsTx<MsgOut>) {
        let (inbound_tx, inbound_rx) = mpsc::unbounded_channel();
        let (outbound_tx, outbound_rx) = mpsc::unbounded_channel();

        let future = WebSocketFuture {
            socket,
            inbound_tx: Some(inbound_tx),
            outbound_rx,
            buffered_item: None,
        };

        (future, inbound_rx, outbound_tx)
    }
}

#[derive(Debug, Error)]
pub enum WsError {
    /// Got an axum generic error
    #[error(transparent)]
    Axum(#[from] axum::Error),

    /// Got a binary message for a text protocol
    #[error("unexpected binary message")]
    UnexpectedBinaryMessage,
}

impl<MsgOut, MsgIn> Future for WebSocketFuture<MsgOut, MsgIn>
where
    MsgOut: Serialize,
    MsgIn: DeserializeOwned,
{
    type Output = Result<(), WsError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        // Read messages from the socket
        while let Some(inbound_tx) = &mut this.inbound_tx {
            let msg = match this.socket.poll_next_unpin(cx) {
                Poll::Ready(Some(result)) => result?,

                // Socket is already closed, cannot ready anything more
                Poll::Ready(None) => return Poll::Ready(Ok(())),

                // Nothing yet, move onto the write polling
                Poll::Pending => break,
            };

            // Handle message types
            let msg = match msg {
                WsMessage::Text(utf8_bytes) => utf8_bytes,
                // Expecting a text based protocol
                WsMessage::Binary(_) => return Poll::Ready(Err(WsError::UnexpectedBinaryMessage)),

                // Ping and pong are handled internally
                WsMessage::Ping(_) | WsMessage::Pong(_) => continue,

                // Socket is closed
                WsMessage::Close(_) => return Poll::Ready(Ok(())),
            };

            // Deserialize message
            let msg: MsgIn = match serde_json::from_str(msg.as_str()) {
                Ok(value) => value,
                Err(cause) => {
                    tracing::warn!(?cause, "got invalid or unknown message from socket");
                    continue;
                }
            };

            if inbound_tx.send(msg).is_err() {
                // Receiver for messages has dropped, stop reading messages
                this.inbound_tx.take();
                break;
            }
        }

        // Write messages to the socket
        loop {
            if this.buffered_item.is_some() {
                // Wait until the socket is ready
                ready!(this.socket.poll_ready_unpin(cx))?;

                // Take the buffered item
                let packet = this
                    .buffered_item
                    .take()
                    .expect("unexpected write state without a packet");

                // Write the buffered item
                this.socket.start_send_unpin(packet)?;
            }

            match this.outbound_rx.poll_recv(cx) {
                // Message ready, set the buffered item
                Poll::Ready(Some(item)) => {
                    // Serialize outbound message
                    let msg = match serde_json::to_string(&item) {
                        Ok(value) => value,
                        Err(cause) => {
                            tracing::error!(?cause, "failed to serialize outbound message");
                            return Poll::Ready(Ok(()));
                        }
                    };

                    this.buffered_item = Some(WsMessage::text(msg));
                }
                // All message senders have dropped, close the socket
                Poll::Ready(None) => {
                    ready!(this.socket.poll_close_unpin(cx))?;
                    return Poll::Ready(Ok(()));
                }
                Poll::Pending => {
                    // Failed to flush the socket
                    ready!(this.socket.poll_flush_unpin(cx))?;
                    return Poll::Pending;
                }
            }
        }
    }
}
