use crate::{
    fonts::Fonts,
    server::models::error::{DynHttpError, HttpError},
};
use axum::{
    Extension,
    extract::{Path, Query},
    http::{HeaderValue, StatusCode, header::CONTENT_TYPE},
    response::IntoResponse,
};
use reqwest::header::CONTENT_DISPOSITION;
use serde::Deserialize;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FontError {
    #[error("font not found")]
    UnknownFont,

    #[error("invalid font path")]
    InvalidFontPath,

    #[error("io error occurred")]
    Io(#[from] std::io::Error),
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct FontQuery {
    pub bold: bool,
    pub italic: bool,
}

/// GET /fonts/{font_family}
///
/// Get a font file
pub async fn get_font_file(
    Extension(fonts): Extension<Arc<Fonts>>,
    Path(font_family): Path<String>,
    Query(query): Query<FontQuery>,
) -> Result<impl IntoResponse, DynHttpError> {
    let font_path = fonts
        .query_font_path(&font_family, query.bold, query.italic)
        .ok_or(FontError::UnknownFont)?;

    let mime = mime_guess::from_path(&font_path);
    let mime = mime.first_or_octet_stream();

    let file_name = font_path.file_name().ok_or(FontError::InvalidFontPath)?;
    let file_disposition = format!("attachment; filename=\"{}\"", file_name.to_string_lossy());

    let file_bytes = tokio::fs::read(font_path).await.map_err(FontError::Io)?;
    let mime_header = HeaderValue::try_from(mime.essence_str()).map_err(axum::http::Error::from)?;
    let content_disposition_header =
        HeaderValue::try_from(file_disposition).map_err(axum::http::Error::from)?;

    Ok((
        StatusCode::OK,
        [
            (CONTENT_TYPE, mime_header),
            (CONTENT_DISPOSITION, content_disposition_header),
        ],
        file_bytes,
    ))
}

impl HttpError for FontError {
    fn log(&self) {
        match self {
            // Font not found errors are ignored
            FontError::UnknownFont => {}
            FontError::InvalidFontPath => {
                tracing::warn!("attempted to access invalid file path")
            }
            FontError::Io(cause) => {
                tracing::error!(?cause, "io error occurred when serving file")
            }
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            FontError::UnknownFont => StatusCode::NOT_FOUND,
            FontError::InvalidFontPath => StatusCode::BAD_REQUEST,
            FontError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
