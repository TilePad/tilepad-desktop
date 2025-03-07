use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        entity::{
            device::{DeviceModel, UpdateDevice},
            folder::{CreateFolder, FolderId, FolderModel, UpdateFolder},
            profile::ProfileId,
        },
        DbPool,
    },
    device::Devices,
};

/// Get all folders for the specified profile
#[tauri::command]
pub async fn folders_get_folders(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<Vec<FolderModel>> {
    let profiles = FolderModel::all(db.inner(), profile_id).await?;
    Ok(profiles)
}

/// Get a specific folder
#[tauri::command]
pub async fn folders_get_folder(
    db: State<'_, DbPool>,
    folder_id: FolderId,
) -> CmdResult<Option<FolderModel>> {
    let folder = FolderModel::get_by_id(db.inner(), folder_id).await?;
    Ok(folder)
}

/// Create a new folder
#[tauri::command]
pub async fn folders_create_folder(
    db: State<'_, DbPool>,
    create: CreateFolder,
) -> CmdResult<FolderModel> {
    let folder = FolderModel::create(&db, create).await?;
    Ok(folder)
}

/// Update a specific folder
#[tauri::command]
pub async fn folders_update_folder(
    db: State<'_, DbPool>,
    devices: State<'_, Devices>,
    folder_id: FolderId,
    update: UpdateFolder,
) -> CmdResult<FolderModel> {
    let folder = FolderModel::get_by_id(db.inner(), folder_id)
        .await?
        .context("unknown folder")?;
    let folder = folder.update(&db, update).await?;

    tokio::spawn({
        let folder_id = folder.id;
        let devices = devices.inner().clone();

        async move {
            let devices = devices;
            _ = devices.update_devices_tiles(folder_id).await;
        }
    });

    Ok(folder)
}

/// Delete a folder
#[tauri::command]
pub async fn folders_delete_folder(
    db: State<'_, DbPool>,
    devices: State<'_, Devices>,
    folder_id: FolderId,
) -> CmdResult<()> {
    let db = db.inner();

    // Get the folder to delete
    let folder = FolderModel::get_by_id(db, folder_id)
        .await?
        .context("folder not found")?;

    if folder.default {
        return Err(anyhow::anyhow!("cannot delete default folder").into());
    }

    // Get the default folder for that profile
    let default_folder = FolderModel::get_default(db, folder.profile_id)
        .await?
        .context("default folder is missing")?;

    // First all devices using the folder must be updated to use the default folder
    let folder_devices = DeviceModel::all_by_folder(db, folder_id).await?;
    for device in folder_devices {
        device
            .update(
                db,
                UpdateDevice {
                    folder_id: Some(default_folder.id),
                    ..Default::default()
                },
            )
            .await?;
    }

    // Update the actual device sessions
    devices.update_devices_tiles(default_folder.id).await?;

    // Delete the folder itself
    FolderModel::delete(db, folder_id).await?;

    Ok(())
}
