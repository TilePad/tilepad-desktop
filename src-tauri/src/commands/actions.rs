use tauri::State;

use crate::plugin::{
    action::{Action, ActionCategory},
    manifest::{ActionId, PluginId},
    PluginRegistry,
};

/// Get a list of all available actions from the plugin registry
#[tauri::command]
pub fn actions_get_actions(plugins: State<'_, PluginRegistry>) -> Vec<ActionCategory> {
    plugins.get_action_collection()
}

/// Get a specific action
#[tauri::command]
pub fn actions_get_action(
    plugins: State<'_, PluginRegistry>,
    plugin_id: PluginId,
    action_id: ActionId,
) -> Option<Action> {
    plugins.get_action(&plugin_id, &action_id)
}
