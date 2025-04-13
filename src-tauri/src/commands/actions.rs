use std::sync::Arc;

use tauri::State;

use crate::plugin::{
    Plugins,
    action::{ActionCategory, ActionWithCategory},
    manifest::{ActionId, PluginId},
};

/// Get a list of all available actions from the plugin registry
#[tauri::command]
pub fn actions_get_actions(plugins: State<'_, Arc<Plugins>>) -> Vec<ActionCategory> {
    plugins.get_action_collection()
}

/// Get a specific action
#[tauri::command]
pub fn actions_get_action(
    plugins: State<'_, Arc<Plugins>>,
    plugin_id: PluginId,
    action_id: ActionId,
) -> Option<ActionWithCategory> {
    plugins.get_action(&plugin_id, &action_id)
}
