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

#[tauri::command]
pub async fn folders_get_folders(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<Vec<FolderModel>> {
    let profiles = FolderModel::all(db.inner(), profile_id).await?;
    Ok(profiles)
}

#[tauri::command]
pub async fn folders_get_folder(
    db: State<'_, DbPool>,
    folder_id: FolderId,
) -> CmdResult<FolderModel> {
    let folder = FolderModel::get_by_id(db.inner(), folder_id)
        .await?
        .context("unknown folder")?;
    Ok(folder)
}

#[tauri::command]
pub async fn folders_delete_folder(db: State<'_, DbPool>, folder_id: FolderId) -> CmdResult<()> {
    FolderModel::delete(db.inner(), folder_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn folders_update_folder(
    db: State<'_, DbPool>,
    folder_id: FolderId,
    update: UpdateFolder,
) -> CmdResult<()> {
    let folder = FolderModel::get_by_id(db.inner(), folder_id)
        .await?
        .context("unknown folder")?;
    folder.update(&db, update).await?;

    Ok(())
}
#[tauri::command]
pub async fn folders_create_folder(
    db: State<'_, DbPool>,
    create: CreateFolder,
) -> CmdResult<FolderModel> {
    let folder = FolderModel::create(&db, create).await?;
    Ok(folder)
}
