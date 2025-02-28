use std::{
    future::Future,
    pin::Pin,
    task::{ready, Context, Poll},
};

use axum::extract::ws::{Message as WsMessage, WebSocket};
use futures::{SinkExt, StreamExt};
use tokio::sync::mpsc;

pub struct WebSocketFuture {
    /// Socket we are acting upon
    socket: WebSocket,
    /// Channel for processing received messages
    inbound_tx: Option<mpsc::UnboundedSender<WsMessage>>,
    /// Channel for outbound messages
    outbound_rx: mpsc::UnboundedReceiver<WsMessage>,
    /// Currently accepted outbound item, ready to be written
    buffered_item: Option<WsMessage>,
}

pub type WsTx = mpsc::UnboundedSender<WsMessage>;
pub type WsRx = mpsc::UnboundedReceiver<WsMessage>;

impl WebSocketFuture {
    pub fn new(socket: WebSocket) -> (WebSocketFuture, WsRx, WsTx) {
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

impl Future for WebSocketFuture {
    type Output = Result<(), axum::Error>;

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
                    this.buffered_item = Some(item);
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
