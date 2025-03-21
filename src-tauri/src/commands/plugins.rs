use crate::{
    commands::CmdResult,
    database::DbPool,
    events::{AppEvent, AppEventSender, PluginAppEvent, PluginMessageContext},
    plugin::{manifest::PluginId, PluginRegistry},
};
use tauri::State;

#[tauri::command]
pub async fn plugins_send_plugin_message(
    app_tx: State<'_, AppEventSender>,
    plugins: State<'_, PluginRegistry>,
    db: State<'_, DbPool>,
    context: PluginMessageContext,
    message: serde_json::Value,
) -> CmdResult<()> {
    plugins
        .handle_send_message(app_tx.inner(), db.inner(), context, message)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn plugins_open_inspector(
    app_tx: State<'_, AppEventSender>,
    context: PluginMessageContext,
) -> CmdResult<()> {
    app_tx.send(AppEvent::Plugin(PluginAppEvent::OpenInspector { context }))?;
    Ok(())
}

#[tauri::command]
pub async fn plugins_close_inspector(
    app_tx: State<'_, AppEventSender>,
    context: PluginMessageContext,
) -> CmdResult<()> {
    app_tx.send(AppEvent::Plugin(PluginAppEvent::CloseInspector { context }))?;
    Ok(())
}

#[tauri::command]
pub async fn plugins_get_plugin_properties(
    plugins: State<'_, PluginRegistry>,
    plugin_id: PluginId,
) -> CmdResult<serde_json::Value> {
    let result = plugins.get_plugin_properties(plugin_id).await?;
    Ok(result)
}

#[tauri::command]
pub async fn plugins_set_plugin_properties(
    plugins: State<'_, PluginRegistry>,
    plugin_id: PluginId,
    properties: serde_json::Value,
) -> CmdResult<()> {
    plugins.set_plugin_properties(plugin_id, properties).await?;
    Ok(())
}
