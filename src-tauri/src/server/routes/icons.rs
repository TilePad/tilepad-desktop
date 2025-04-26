use std::sync::Arc;

use anyhow::Context;
use axum::{Extension, body::Body, extract::Path, response::Response};
use reqwest::{StatusCode, header::CONTENT_TYPE};
use tilepad_manifest::icons::IconPackId;

use crate::{
    icons::Icons,
    server::{http_content::read_serve_file, models::error::DynHttpError},
};

/// GET /icons/{pack_id}/assets/{file_path*}
pub async fn get_icon_file(
    Path((pack_id, path)): Path<(IconPackId, String)>,
    Extension(icons): Extension<Arc<Icons>>,
) -> Result<Response<Body>, DynHttpError> {
    let icon_pack_path = icons.get_pack_path(&pack_id).context("unknown plugin")?;
    let file_path = icon_pack_path.join(path);
    let (file_bytes, mime) = read_serve_file(&icon_pack_path, &file_path).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}

/// GET /uploaded-icons/{file_path*}
pub async fn get_uploaded_icon_file(
    Path(path): Path<String>,
    Extension(icons): Extension<Arc<Icons>>,
) -> Result<Response<Body>, DynHttpError> {
    let icons_path = icons.uploaded_path();
    let file_path = icons_path.join(path);
    let (file_bytes, mime) = read_serve_file(icons_path, &file_path).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(CONTENT_TYPE, mime.essence_str())
        .body(file_bytes.into())
        .context("failed to make response")?)
}
