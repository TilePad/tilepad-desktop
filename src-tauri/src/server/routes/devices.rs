use std::{net::SocketAddr, sync::Arc};

use axum::{
    Extension,
    extract::{ConnectInfo, WebSocketUpgrade},
    response::Response,
};

use crate::device::{Devices, session::DeviceSession};

/// GET /devices/ws
///
/// Accept a new device websocket connection upgrade
pub async fn ws(
    Extension(devices): Extension<Arc<Devices>>,
    Extension(ConnectInfo(socket_addr)): Extension<ConnectInfo<SocketAddr>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| async move { DeviceSession::start(devices, socket_addr, socket) })
}
