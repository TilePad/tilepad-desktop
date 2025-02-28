use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        entity::{
            folder::{FolderId, FolderModel},
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
pub async fn folders_set_default(db: State<'_, DbPool>, folder_id: FolderId) -> CmdResult<()> {
    let mut folder = FolderModel::get_by_id(db.inner(), folder_id)
        .await?
        .context("unknown folder")?;
    folder.set_default(&db).await?;

    Ok(())
}
