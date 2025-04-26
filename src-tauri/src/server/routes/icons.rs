use crate::{
    icons::Icons,
    server::{
        http_content::read_serve_file,
        models::error::{DynHttpError, HttpError},
    },
};
use axum::{
    Extension,
    extract::Path,
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
};
use std::sync::Arc;
use thiserror::Error;
use tilepad_manifest::icons::IconPackId;

#[derive(Debug, Error)]
pub enum IconError {
    #[error("unknown icon pack")]
    UnknownIconPack,
}

/// GET /icons/{pack_id}/assets/{file_path*}
///
/// Get an icon pack file
pub async fn get_icon_file(
    Path((pack_id, path)): Path<(IconPackId, String)>,
    Extension(icons): Extension<Arc<Icons>>,
) -> Result<impl IntoResponse, DynHttpError> {
    let icon_pack_path = icons
        .get_pack_path(&pack_id)
        .ok_or(IconError::UnknownIconPack)?;
    let file_path = icon_pack_path.join(path);
    let (file_bytes, mime) = read_serve_file(&icon_pack_path, &file_path).await?;
    let mime_header = HeaderValue::try_from(mime.essence_str()).map_err(axum::http::Error::from)?;

    Ok((StatusCode::OK, [(CONTENT_TYPE, mime_header)], file_bytes))
}

/// GET /uploaded-icons/{file_path*}
///
/// Get a user uploaded icon file
pub async fn get_uploaded_icon_file(
    Path(path): Path<String>,
    Extension(icons): Extension<Arc<Icons>>,
) -> Result<impl IntoResponse, DynHttpError> {
    let icons_path = icons.uploaded_path();
    let file_path = icons_path.join(path);
    let (file_bytes, mime) = read_serve_file(icons_path, &file_path).await?;
    let mime_header = HeaderValue::try_from(mime.essence_str()).map_err(axum::http::Error::from)?;

    Ok((StatusCode::OK, [(CONTENT_TYPE, mime_header)], file_bytes))
}

impl HttpError for IconError {
    fn log(&self) {}

    fn status(&self) -> StatusCode {
        match self {
            IconError::UnknownIconPack => StatusCode::NOT_FOUND,
        }
    }
}
