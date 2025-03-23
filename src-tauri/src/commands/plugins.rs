use crate::{
    commands::CmdResult,
    database::DbPool,
    events::{AppEventSender, InspectorContext},
    plugin::{manifest::PluginId, PluginWithState, Plugins},
};
use tauri::State;

#[tauri::command]
pub async fn plugins_send_plugin_message(
    app_tx: State<'_, AppEventSender>,
    plugins: State<'_, Plugins>,
    db: State<'_, DbPool>,
    context: InspectorContext,
    message: serde_json::Value,
) -> CmdResult<()> {
    plugins
        .handle_send_message(app_tx.inner(), db.inner(), context, message)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn plugins_open_inspector(
    plugins: State<'_, Plugins>,
    context: InspectorContext,
) -> CmdResult<()> {
    plugins.open_inspector(context).await?;
    Ok(())
}

#[tauri::command]
pub async fn plugins_close_inspector(
    plugins: State<'_, Plugins>,
    context: InspectorContext,
) -> CmdResult<()> {
    plugins.close_inspector(context).await?;
    Ok(())
}

#[tauri::command]
pub async fn plugins_get_plugin_properties(
    plugins: State<'_, Plugins>,
    plugin_id: PluginId,
) -> CmdResult<serde_json::Value> {
    let result = plugins.get_plugin_properties(plugin_id).await?;
    Ok(result)
}

#[tauri::command]
pub async fn plugins_set_plugin_properties(
    plugins: State<'_, Plugins>,
    plugin_id: PluginId,
    properties: serde_json::Value,
) -> CmdResult<()> {
    plugins.set_plugin_properties(plugin_id, properties).await?;
    Ok(())
}

#[tauri::command]
pub fn plugins_get_plugins(plugins: State<'_, Plugins>) -> Vec<PluginWithState> {
    plugins.get_plugins_with_state()
}
