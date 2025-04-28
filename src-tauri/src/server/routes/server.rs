use axum::{Extension, Json};

use crate::{
    database::{DbPool, entity::settings::SettingsModel},
    server::models::{error::HttpResult, server::ServerDetails},
    utils::device::get_device_name,
};

const IDENTIFIER: &str = "TILEPAD_CONTROLLER_SERVER";

/// GET /server/details
///
/// Get simple details about the server, used to check if a server
/// is alive by device clients
pub async fn details(Extension(db): Extension<DbPool>) -> HttpResult<ServerDetails> {
    let settings = SettingsModel::get_or_default(&db).await?;
    let name = settings.config.device_name;

    Json(ServerDetails {
        identifier: IDENTIFIER,
        hostname: name,
    })
}
