use anyhow::Context;
use axum::{
    body::Body,
    extract::{ws::WebSocket, Path, WebSocketUpgrade},
    response::Response,
    Extension,
};
use mime_guess::mime::TEXT_HTML;
use reqwest::{header::CONTENT_TYPE, StatusCode};

use crate::{
    plugin::{manifest::PluginId, socket::PluginSession, PluginRegistry},
    server::models::error::DynHttpError,
};

/// GET /plugins/ws
///
/// Accept a new plugin websocket connection upgrade
pub async fn ws(Extension(plugins): Extension<PluginRegistry>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(move |socket| handle_plugin_socket(plugins, socket))
}

/// Handle the connection of a new plugin socket
pub async fn handle_plugin_socket(plugins: PluginRegistry, socket: WebSocket) {
    let (session_id, session_ref) = PluginSession::new(plugins.clone(), socket);
    plugins.insert_session(session_id, session_ref);
}

static INSPECTOR_SCRIPT: &str = include_str!("../resources/propertyInspectorScript.js");
static INSPECTOR_STYLES: &str = include_str!("../resources/propertyInspectorStyles.css");

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
