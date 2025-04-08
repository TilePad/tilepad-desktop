use crate::{
    commands::CmdResult,
    icons::{
        install::{install_icon_pack_zip, remove_icon_pack_files},
        loader::{load_icon_pack_from_path, read_icon_pack_manifest_zip},
        IconPack, Icons,
    },
};
use anyhow::Context;
use std::sync::Arc;
use tauri::State;
use tilepad_manifest::icons::IconPackId;

/// Get a list of all available actions from the icon pack registry
#[tauri::command]
pub fn icons_get_icon_packs(icons: State<'_, Arc<Icons>>) -> Vec<Arc<IconPack>> {
    icons.get_icon_packs()
}

/// Install an icon pack into the registry
#[tauri::command]
pub async fn icons_install_icon_pack(icons: State<'_, Arc<Icons>>, data: Vec<u8>) -> CmdResult<()> {
    let user_icons = icons.packs_path();

    // Read the icon pack manifest from within the zip file
    let manifest = read_icon_pack_manifest_zip(&data).await?;

    // Determine icon pack install directory
    let pack_id = manifest.icons.id;
    let path = user_icons.join(&pack_id.0);

    // Unload the icon pack
    icons.unload_pack(&pack_id);

    // Cleanup old files
    remove_icon_pack_files(&path).await?;

    // Install the icon pack zip file
    install_icon_pack_zip(&data, &path).await?;

    // Load the icon pack
    let pack = load_icon_pack_from_path(&path)
        .await
        .context("failed to load icon pack")?;

    icons.load_pack(pack);

    Ok(())
}

/// Remove an icon pack from the registry
#[tauri::command]
pub async fn icons_uninstall_icon_pack(
    icons: State<'_, Arc<Icons>>,
    pack_id: IconPackId,
) -> CmdResult<()> {
    let user_icons = icons.packs_path();

    // Determine icon pack install directory
    let path = user_icons.join(&pack_id.0);

    // Unload the pack
    icons.unload_pack(&pack_id);

    // Cleanup old files
    remove_icon_pack_files(&path).await?;

    Ok(())
}

#[tauri::command]
pub async fn icons_upload_user_icon(
    icons: State<'_, Arc<Icons>>,
    name: String,
    data: Vec<u8>,
) -> CmdResult<String> {
    let file_name = icons.upload_user_icon(name, data).await?;
    Ok(file_name)
}
