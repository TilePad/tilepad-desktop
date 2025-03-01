use tauri::State;

use crate::plugin::{action::ActionCategory, PluginRegistry};

/// Get a list of all available actions from the plugin registry
#[tauri::command]
pub fn actions_get_actions(plugins: State<'_, PluginRegistry>) -> Vec<ActionCategory> {
    plugins.get_action_collection()
}
