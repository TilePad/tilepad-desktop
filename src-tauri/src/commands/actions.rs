use tauri::State;

use crate::plugin::{
    action::{ActionCategory, ActionWithCategory},
    manifest::{ActionId, PluginId},
    Plugins,
};

/// Get a list of all available actions from the plugin registry
#[tauri::command]
pub fn actions_get_actions(plugins: State<'_, Plugins>) -> Vec<ActionCategory> {
    plugins.get_action_collection()
}

/// Get a specific action
#[tauri::command]
pub fn actions_get_action(
    plugins: State<'_, Plugins>,
    plugin_id: PluginId,
    action_id: ActionId,
) -> Option<ActionWithCategory> {
    plugins.get_action(&plugin_id, &action_id)
}
