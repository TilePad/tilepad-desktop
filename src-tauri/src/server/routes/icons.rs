use std::sync::Arc;

use anyhow::Context;
use axum::{Extension, body::Body, extract::Path, response::Response};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use tauri::{AppHandle, Manager};
use tilepad_manifest::icons::IconPackId;

use crate::{icons::Icons, server::models::error::DynHttpError};

/// GET /icons/{pack_id}/assets/{file_path*}
pub async fn get_icon_file(
    Path((pack_id, path)): Path<(IconPackId, String)>,
    Extension(icons): Extension<Arc<Icons>>,
) -> Result<Response<Body>, DynHttpError> {
    let icon_path = icons.get_pack_path(&pack_id).context("unknown plugin")?;
    let file_path = icon_path.join(path);

    // TODO: Assert file path is within pack path

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(vec![].into())
            .context("failed to make response")?);
    }

    let mime = mime_guess::from_path(&file_path);

    let file_bytes = tokio::fs::read(&file_path)
        .await
        .context("failed to read content file")?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.first_or_octet_stream().essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}

/// GET /uploaded-icons/{file_path*}
pub async fn get_uploaded_icon_file(
    Path(path): Path<String>,
    Extension(app): Extension<AppHandle>,
) -> Result<Response<Body>, DynHttpError> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let uploaded_icons = app_data_path.join("uploaded_icons");
    let file_path = uploaded_icons.join(path);

    // TODO: Assert file path is within uploaded icons path

    if !file_path.exists() {
        return Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(vec![].into())
            .context("failed to make response")?);
    }

    let mime = mime_guess::from_path(&file_path);

    let file_bytes = tokio::fs::read(&file_path)
        .await
        .context("failed to read content file")?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.first_or_octet_stream().essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}
