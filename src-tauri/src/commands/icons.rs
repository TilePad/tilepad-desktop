use std::{path::Path, sync::Arc};

use anyhow::Context;
use tauri::{AppHandle, Manager, State};
use tilepad_manifest::icons::IconPackId;
use uuid::Uuid;

use crate::{
    commands::CmdResult,
    icons::{
        install::{install_icon_pack_zip, remove_icon_pack_files},
        loader::{load_icon_pack_from_path, read_icon_pack_manifest_zip},
        IconPack, Icons,
    },
};

/// Get a list of all available actions from the icon pack registry
#[tauri::command]
pub fn icons_get_icon_packs(icons: State<'_, Icons>) -> Vec<Arc<IconPack>> {
    icons.get_icon_packs()
}

#[tauri::command]
pub async fn icons_install_icon_pack(
    app: AppHandle,
    icons: State<'_, Icons>,
    data: Vec<u8>,
) -> CmdResult<()> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let user_icons = app_data_path.join("icons");

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

#[tauri::command]
pub async fn icons_uninstall_icon_pack(
    app: AppHandle,
    icons: State<'_, Icons>,
    pack_id: IconPackId,
) -> CmdResult<()> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let user_icons = app_data_path.join("icons");

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
    app: AppHandle,
    name: String,
    data: Vec<u8>,
) -> CmdResult<String> {
    let app_data_path = app
        .path()
        .app_data_dir()
        .context("failed to get app data dir")?;
    let uploaded_icons = app_data_path.join("uploaded_icons");

    if !uploaded_icons.exists() {
        tokio::fs::create_dir_all(&uploaded_icons).await?;
    }

    let file_path_name = Path::new(&name);
    let extension = file_path_name
        .extension()
        .context("missing file extension")?
        .to_string_lossy();
    let file_id = Uuid::new_v4();
    let file_name = format!("{}.{}", file_id, extension);
    let file_path = uploaded_icons.join(&file_name);

    tokio::fs::write(&file_path, data)
        .await
        .context("save file")?;

    Ok(file_name)
}
