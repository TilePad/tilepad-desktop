use crate::{
    plugin::{Plugins, manifest::PluginId, session::PluginSession},
    server::{
        extractors::enforce_local_socket::EnforceLocalSocket, http_content::read_serve_file,
        models::error::DynHttpError,
    },
    tile::Tiles,
    utils::inspector::inject_property_inspector_current,
};
use anyhow::Context;
use axum::{
    Extension,
    body::Body,
    extract::{ConnectInfo, Path, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use mime_guess::mime::{self};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use std::{net::SocketAddr, sync::Arc};

/// GET /plugins/ws
///
/// Accept a new plugin websocket connection upgrade
pub async fn ws(
    _: EnforceLocalSocket,
    Extension(plugins): Extension<Arc<Plugins>>,
    Extension(tiles): Extension<Arc<Tiles>>,
    Extension(connect_info): Extension<ConnectInfo<SocketAddr>>,
    ws: WebSocketUpgrade,
) -> Response {
    tracing::debug!(?connect_info, "plugin session starting");

    ws.on_upgrade(move |socket| handle_plugin_socket(plugins, tiles, socket))
}

/// Handle the connection of a new plugin socket
async fn handle_plugin_socket(plugins: Arc<Plugins>, tiles: Arc<Tiles>, socket: WebSocket) {
    PluginSession::start(plugins, tiles, socket);
}

/// GET /plugins/{plugin_id}/assets/{file_path*}
pub async fn get_plugin_file(
    _: EnforceLocalSocket,
    Path((plugin_id, path)): Path<(PluginId, String)>,
    Extension(plugins): Extension<Arc<Plugins>>,
) -> Result<Response<Body>, DynHttpError> {
    let plugin = match plugins.get_plugin(&plugin_id) {
        Some(value) => value,
        None => {
            return Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(vec![].into())
                .context("failed to make response")?);
        }
    };

    let plugin_path = &plugin.path;
    let file_path = plugin_path.join(path);

    let (mut file_bytes, mime) = read_serve_file(plugin_path, &file_path).await?;

    // Inject inspector script and styles into HTML files
    if mime == mime::TEXT_HTML {
        let file_text = String::from_utf8(file_bytes)
            .context("unsupported html file, encoding must be utf8 compatible")?;
        let file_text = inject_property_inspector_current(&file_text).await;

        file_bytes = file_text.into_bytes();
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}
