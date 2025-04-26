use std::sync::Arc;

use tauri::State;

use crate::fonts::Fonts;

use super::CmdResult;

/// Get a list of fonts available on the system
#[tauri::command]
pub async fn fonts_fonts(fonts: State<'_, Arc<Fonts>>) -> CmdResult<Vec<String>> {
    let all_fonts = fonts.inner().all_families();
    Ok(all_fonts)
}
