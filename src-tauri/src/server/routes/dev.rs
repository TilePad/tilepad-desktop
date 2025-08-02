use std::sync::Arc;

use axum::{Extension, extract::Path, http::StatusCode};
use thiserror::Error;
use tilepad_manifest::plugin::PluginId;

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
    #[error("plugin not found")]
    PluginNotFound,
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

/// POST /dev/plugin/{id}/stop
///
/// Stop a specific plugin by ID
pub async fn stop_plugin(
    _: EnforceLocalSocket,
    Extension(db): Extension<DbPool>,
    Extension(plugins): Extension<Arc<Plugins>>,
    Path(plugin_id): Path<PluginId>,
) -> Result<StatusCode, DynHttpError> {
    let settings = SettingsModel::get_or_default(&db)
        .await
        .map_err(DevError::GetSettings)?;

    if !settings.config.developer_mode {
        return Err(DevError::NotDevelopment.into());
    }

    plugins.stop_task(&plugin_id).await;

    Ok(StatusCode::OK)
}

/// POST /dev/plugin/{id}/restart
///
/// Restart a specific plugin by ID
pub async fn restart_plugin(
    _: EnforceLocalSocket,
    Extension(db): Extension<DbPool>,
    Extension(plugins): Extension<Arc<Plugins>>,
    Path(plugin_id): Path<PluginId>,
) -> Result<StatusCode, DynHttpError> {
    let settings = SettingsModel::get_or_default(&db)
        .await
        .map_err(DevError::GetSettings)?;

    if !settings.config.developer_mode {
        return Err(DevError::NotDevelopment.into());
    }

    let plugin = plugins
        .get_plugin(&plugin_id)
        .ok_or(DevError::PluginNotFound)?;

    plugins
        .restart_task(plugin_id, plugin.path.to_path_buf(), &plugin.manifest)
        .await;

    Ok(StatusCode::OK)
}

impl HttpError for DevError {
    fn status(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
