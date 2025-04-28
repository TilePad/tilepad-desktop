use axum::{Extension, Json, http::StatusCode};
use thiserror::Error;

use crate::{
    database::{DbErr, DbPool, entity::settings::SettingsModel},
    server::models::{
        error::{HttpError, HttpResult},
        server::ServerDetails,
    },
};

const IDENTIFIER: &str = "TILEPAD_CONTROLLER_SERVER";

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("failed to load settings")]
    GetSettings(#[from] DbErr),
}

/// GET /server/details
///
/// Get simple details about the server, used to check if a server
/// is alive by device clients
pub async fn details(Extension(db): Extension<DbPool>) -> HttpResult<ServerDetails> {
    let settings = SettingsModel::get_or_default(&db)
        .await
        .map_err(ServerError::GetSettings)?;
    let name = settings.config.device_name;

    Ok(Json(ServerDetails {
        identifier: IDENTIFIER,
        hostname: name,
    }))
}

impl HttpError for ServerError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
