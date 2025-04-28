use tauri::State;

use crate::database::{
    DbPool,
    entity::settings::{SettingsConfig, SettingsModel},
};

use super::CmdResult;

/// Get the current settings
#[tauri::command]
pub async fn settings_get_settings(db: State<'_, DbPool>) -> CmdResult<SettingsConfig> {
    let model = SettingsModel::get_or_default(db.inner()).await?;
    Ok(model.config)
}

/// Update the current settings
#[tauri::command]
pub async fn settings_set_settings(
    db: State<'_, DbPool>,
    settings: SettingsConfig,
) -> CmdResult<SettingsConfig> {
    let model = SettingsModel::get_or_default(db.inner()).await?;
    let model = model.update(db.inner(), settings).await?;
    Ok(model.config)
}
