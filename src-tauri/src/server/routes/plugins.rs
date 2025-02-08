use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
};

/// Accept a websocket connection
pub async fn ws(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_plugin_socket)
}

pub async fn handle_plugin_socket(mut socket: WebSocket) {}
