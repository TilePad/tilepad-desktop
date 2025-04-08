use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ws::WebSocket, ConnectInfo, WebSocketUpgrade},
    response::Response,
    Extension,
};

use crate::device::{session::DeviceSession, Devices};

/// GET /devices/ws
///
/// Accept a new device websocket connection upgrade
pub async fn ws(
    Extension(devices): Extension<Arc<Devices>>,
    Extension(connect_info): Extension<ConnectInfo<SocketAddr>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_device_socket(devices, connect_info.0, socket))
}

/// Handle the connection of a new device socket
pub async fn handle_device_socket(
    devices: Arc<Devices>,
    socket_addr: SocketAddr,
    socket: WebSocket,
) {
    DeviceSession::start(devices, socket_addr, socket);
}
