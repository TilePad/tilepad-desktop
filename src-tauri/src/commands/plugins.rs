use tauri::State;

use crate::{
    commands::CmdResult,
    database::DbPool,
    events::{AppEvent, AppEventSender, PluginAppEvent, PluginMessageContext},
    plugin::PluginRegistry,
};

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
