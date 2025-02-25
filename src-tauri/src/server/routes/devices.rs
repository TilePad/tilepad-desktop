use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade},
    response::Response,
    Extension,
};

use crate::device::{socket::DeviceSession, Devices};

/// GET /devices/ws
///
/// Accept a new device websocket connection upgrade
pub async fn ws(Extension(devices): Extension<Devices>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| handle_device_socket(devices, socket))
}

/// Handle the connection of a new device socket
pub async fn handle_device_socket(devices: Devices, socket: WebSocket) {
    let (session_id, session_ref) = DeviceSession::new(devices.clone(), socket);
    devices.insert_session(session_id, session_ref);
}
