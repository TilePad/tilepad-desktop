use tauri::State;

use crate::icons::{IconPack, Icons};

/// Get a list of all available actions from the plugin registry
#[tauri::command]
pub fn icons_get_icon_packs(icons: State<'_, Icons>) -> Vec<IconPack> {
    icons.get_icon_packs()
}
