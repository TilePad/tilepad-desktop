use anyhow::Context;
use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, WebSocketUpgrade},
    response::Response,
    Extension,
};
use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::{
    plugin::{manifest::PluginId, PluginRegistry},
    server::models::error::DynHttpError,
};

/// GET /plugins/ws
///
/// Accept a websocket connection
pub async fn ws(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_plugin_socket)
}

pub async fn handle_plugin_socket(_socket: WebSocket) {}

/// GET /plugins/{plugin_id}/assets/{file_path*}
pub async fn get_plugin_file(
    Path((plugin_id, path)): Path<(PluginId, String)>,
    Extension(plugins): Extension<PluginRegistry>,
) -> Result<Response<Body>, DynHttpError> {
    let plugin_path = plugins
        .get_plugin_path(&plugin_id)
        .context("unknown plugin")?;

    let file_path = plugin_path.join(path);

    // TODO: Assert file path is within plugin path

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(vec![].into())
            .context("failed to make response")?);
    }

    let mime = mime_guess::from_path(&file_path);

    let file_bytes = tokio::fs::read(file_path)
        .await
        .context("failed to read content file")?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.first_or_octet_stream().essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}
