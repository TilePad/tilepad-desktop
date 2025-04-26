use mime_guess::Mime;
use reqwest::StatusCode;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::utils::file::is_within;

use super::models::error::HttpError;

#[derive(Debug, Error)]
pub enum ServeFileError {
    #[error("file not found")]
    FileNotFound,
    #[error("file path is invalid")]
    InvalidFilePath(PathBuf),
    #[error("io error occurred")]
    Io(#[from] std::io::Error),
}

/// Reads a file that is to be served, validates the file path
/// and determines the file mime type
pub async fn read_serve_file(
    base_path: &Path,
    file_path: &Path,
) -> Result<(Vec<u8>, Mime), ServeFileError> {
    // File does not exist
    if !file_path.exists() {
        return Err(ServeFileError::FileNotFound);
    }

    // Assert that the file is within the allowed base path
    if !is_within(base_path, file_path).await? {
        return Err(ServeFileError::InvalidFilePath(file_path.to_path_buf()));
    }

    let file_bytes = tokio::fs::read(file_path).await?;

    let mime = mime_guess::from_path(file_path);
    let mime = mime.first_or_octet_stream();

    Ok((file_bytes, mime))
}

impl HttpError for ServeFileError {
    fn log(&self) {
        match self {
            // File not found errors are ignored
            ServeFileError::FileNotFound => {}
            ServeFileError::InvalidFilePath(path) => {
                tracing::warn!(?path, "attempted to access invalid file path")
            }
            ServeFileError::Io(cause) => {
                tracing::error!(?cause, "io error occurred when serving file")
            }
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            ServeFileError::FileNotFound => StatusCode::NOT_FOUND,
            ServeFileError::InvalidFilePath(_) => StatusCode::BAD_REQUEST,
            ServeFileError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
