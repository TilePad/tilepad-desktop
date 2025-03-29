use anyhow::Context;
use axum::{body::Body, extract::Path, response::Response, Extension};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use tilepad_manifest::icons::IconPackId;

use crate::{icons::Icons, server::models::error::DynHttpError};

/// GET /icons/{pack_id}/assets/{file_path*}
pub async fn get_icon_file(
    Path((pack_id, path)): Path<(IconPackId, String)>,
    Extension(icons): Extension<Icons>,
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
