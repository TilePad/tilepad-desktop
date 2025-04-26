use std::{borrow::Cow, net::SocketAddr, sync::Arc};

use anyhow::Context;
use axum::{
    Extension,
    body::Body,
    extract::{ConnectInfo, Path, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use mime_guess::mime::TEXT_HTML;
use reqwest::{StatusCode, header::CONTENT_TYPE};
use tokio::join;

use crate::{
    plugin::{Plugins, manifest::PluginId, session::PluginSession},
    server::{extractors::enforce_local_socket::EnforceLocalSocket, models::error::DynHttpError},
    tile::Tiles,
};

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

/// In release mode bake in the inspector script
#[cfg(not(debug_assertions))]
async fn get_inspector_script() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../../inspector/dist/inspector.js"))
}

/// When debugging, load the inspector script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_inspector_script() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let inspector_script_path = manifest_dir.join("../inspector/dist/inspector.js");

    Cow::Owned(
        tokio::fs::read_to_string(inspector_script_path)
            .await
            .unwrap(),
    )
}

/// In release mode bake in the inspector script
#[cfg(not(debug_assertions))]
async fn get_inspector_styles() -> Cow<'static, str> {
    Cow::Borrowed(include_str!("../../../../inspector/dist/inspector.css"))
}

/// When debugging, load the inspector script directly from the file system
/// this allows updating it at runtime
#[cfg(debug_assertions)]
async fn get_inspector_styles() -> Cow<'static, str> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable missing");

    let manifest_dir = std::path::Path::new(&manifest_dir);
    let inspector_script_path = manifest_dir.join("../inspector/dist/inspector.css");

    Cow::Owned(
        tokio::fs::read_to_string(inspector_script_path)
            .await
            .unwrap(),
    )
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
    let file_path = plugin.path.join(path);

    // TODO: Assert file path is within plugin path

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(vec![].into())
            .context("failed to make response")?);
    }

    let mime = mime_guess::from_path(&file_path);

    if let Some(ext) = file_path.extension() {
        // Inject inspector script into HTML files
        if ext.eq_ignore_ascii_case("html") {
            let file_text = tokio::fs::read_to_string(&file_path)
                .await
                .context("failed to read file content")?;

            let (inspector_script, inspector_styles) =
                join!(get_inspector_script(), get_inspector_styles());

            let file_text =
                inject_property_inspector_script(&file_text, &inspector_script, &inspector_styles);

            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header(CONTENT_TYPE, TEXT_HTML.essence_str())
                .body(file_text.into())
                .context("failed to make response")?);
        }
    }

    let file_bytes = tokio::fs::read(&file_path)
        .await
        .context("failed to read content file")?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.first_or_octet_stream().essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}

fn inject_property_inspector_script(
    value: &str,
    inspector_script: &str,
    inspector_styles: &str,
) -> String {
    value.replace(
        "<head>",
        &format!("<head><script>{inspector_script}</script><style>{inspector_styles}</style>",),
    )
}
