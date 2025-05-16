use crate::{
    plugin::{Plugins, session::PluginSession},
    server::{
        extractors::enforce_local_socket::EnforceLocalSocket,
        http_content::read_serve_file,
        models::error::{DynHttpError, HttpError},
    },
    tile::Tiles,
    utils::{display::inject_display_current, inspector::inject_property_inspector_current},
};
use axum::{
    Extension,
    extract::{ConnectInfo, Path, WebSocketUpgrade},
    http::HeaderValue,
    response::{IntoResponse, Response},
};
use mime_guess::mime::{self};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tilepad_manifest::plugin::PluginId;

#[derive(Debug, Error)]
pub enum PluginError {
    #[error("plugin not found")]
    PluginNotFound,

    #[error("unsupported html file, encoding must be utf8 compatible")]
    UnsupportedHtmlEncoding,
}

/// GET /plugins/ws
///
/// Accept a new plugin websocket connection for a local running
/// plugin application
pub async fn ws(
    _: EnforceLocalSocket,
    Extension(plugins): Extension<Arc<Plugins>>,
    Extension(tiles): Extension<Arc<Tiles>>,
    Extension(connect_info): Extension<ConnectInfo<SocketAddr>>,
    ws: WebSocketUpgrade,
) -> Response {
    tracing::debug!(?connect_info, "plugin session starting");

    ws.on_upgrade(move |socket| async { PluginSession::start(plugins, tiles, socket) })
}

/// GET /plugins/{plugin_id}/assets/{file_path*}
///
/// Get a file from a specific plugin
pub async fn get_plugin_file(
    Path((plugin_id, path)): Path<(PluginId, String)>,
    Extension(plugins): Extension<Arc<Plugins>>,
) -> Result<impl IntoResponse, DynHttpError> {
    let plugin = plugins
        .get_plugin(&plugin_id)
        .ok_or(PluginError::PluginNotFound)?;

    let plugin_path = &plugin.path;
    let file_path = plugin_path.join(path);

    let (mut file_bytes, mime) = read_serve_file(plugin_path, &file_path).await?;
    let mime_header = HeaderValue::try_from(mime.essence_str()).map_err(axum::http::Error::from)?;

    // Display HTML files use the .display.html extension
    let is_display = file_path.file_name().is_some_and(|value| {
        value
            .to_str()
            .is_some_and(|name| name.ends_with("display.html"))
    });

    // Inject display script and styles into display files
    if is_display {
        let file_text =
            String::from_utf8(file_bytes).map_err(|_| PluginError::UnsupportedHtmlEncoding)?;
        let file_text = inject_display_current(&file_text).await;

        file_bytes = file_text.into_bytes();
    }
    // Inject inspector script and styles into HTML files
    else if mime == mime::TEXT_HTML {
        let file_text =
            String::from_utf8(file_bytes).map_err(|_| PluginError::UnsupportedHtmlEncoding)?;
        let file_text = inject_property_inspector_current(&file_text).await;

        file_bytes = file_text.into_bytes();
    }

    Ok((StatusCode::OK, [(CONTENT_TYPE, mime_header)], file_bytes))
}

impl HttpError for PluginError {
    fn log(&self) {}

    fn status(&self) -> StatusCode {
        match self {
            PluginError::PluginNotFound => StatusCode::NOT_FOUND,
            PluginError::UnsupportedHtmlEncoding => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
