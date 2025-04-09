use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, anyhow};
use axum::{
    Extension,
    body::Body,
    extract::{ConnectInfo, Path, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};
use mime_guess::mime::TEXT_HTML;
use reqwest::{StatusCode, header::CONTENT_TYPE};

use crate::{
    plugin::{Plugins, manifest::PluginId, session::PluginSession},
    server::models::error::DynHttpError,
};

/// GET /plugins/ws
///
/// Accept a new plugin websocket connection upgrade
pub async fn ws(
    Extension(plugins): Extension<Arc<Plugins>>,
    Extension(connect_info): Extension<ConnectInfo<SocketAddr>>,
    ws: WebSocketUpgrade,
) -> Result<Response, DynHttpError> {
    if !connect_info.ip().is_loopback() {
        return Err(
            anyhow!("plugin sessions cannot be started from hosts other than loopback").into(),
        );
    }

    tracing::debug!(?connect_info, "plugin session starting");

    Ok(ws.on_upgrade(move |socket| handle_plugin_socket(plugins, socket)))
}

/// Handle the connection of a new plugin socket
pub async fn handle_plugin_socket(plugins: Arc<Plugins>, socket: WebSocket) {
    PluginSession::start(plugins, socket);
}

static INSPECTOR_SCRIPT: &str = include_str!("../resources/propertyInspectorScript.js");
static INSPECTOR_STYLES: &str = include_str!("../resources/propertyInspectorStyles.css");

/// GET /plugins/{plugin_id}/assets/{file_path*}
pub async fn get_plugin_file(
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

            let file_text = inject_property_inspector_script(&file_text);

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

fn inject_property_inspector_script(value: &str) -> String {
    value.replace(
        "<head>",
        &format!(
            "<head><script>{}</script><style>{}</style>",
            INSPECTOR_SCRIPT, INSPECTOR_STYLES
        ),
    )
}
