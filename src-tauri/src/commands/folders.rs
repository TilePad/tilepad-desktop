use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        entity::{
            folder::{CreateFolder, FolderId, FolderModel, UpdateFolder},
            profile::ProfileId,
        },
        DbPool,
    },
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
    folder_id: FolderId,
    update: UpdateFolder,
) -> CmdResult<FolderModel> {
    let folder = FolderModel::get_by_id(db.inner(), folder_id)
        .await?
        .context("unknown folder")?;
    let folder = folder.update(&db, update).await?;

    Ok(folder)
}

/// Delete a folder
#[tauri::command]
pub async fn folders_delete_folder(db: State<'_, DbPool>, folder_id: FolderId) -> CmdResult<()> {
    FolderModel::delete(db.inner(), folder_id).await?;
    Ok(())
}
