use std::sync::Arc;

use axum::{Extension, http::StatusCode};
use thiserror::Error;

use crate::{
    database::{DbErr, DbPool, entity::settings::SettingsModel},
    plugin::Plugins,
    server::{
        extractors::enforce_local_socket::EnforceLocalSocket,
        models::error::{DynHttpError, HttpError},
    },
};

#[derive(Debug, Error)]
pub enum DevError {
    #[error("failed to load settings")]
    GetSettings(#[from] DbErr),
    #[error("failed to load reload plugins")]
    ReloadPlugins(#[from] anyhow::Error),
    #[error("development mode not enabled")]
    NotDevelopment,
}

/// POST /dev/reload_plugins
///
/// Reloads all the plugins that are currently loaded
pub async fn reload_plugins(
    _: EnforceLocalSocket,
    Extension(db): Extension<DbPool>,
    Extension(plugins): Extension<Arc<Plugins>>,
) -> Result<StatusCode, DynHttpError> {
    let settings = SettingsModel::get_or_default(&db)
        .await
        .map_err(DevError::GetSettings)?;

    if !settings.config.developer_mode {
        return Err(DevError::NotDevelopment.into());
    }

    plugins.unload_all().await;
    plugins.load_defaults().await;

    Ok(StatusCode::OK)
}

impl HttpError for DevError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
