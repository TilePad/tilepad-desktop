use anyhow::Context;
use tauri::State;

use crate::{
    commands::CmdResult,
    database::{
        entity::profile::{ProfileId, ProfileModel, UpdateProfile},
        DbPool,
    },
};

#[tauri::command]
pub async fn profiles_get_profiles(db: State<'_, DbPool>) -> CmdResult<Vec<ProfileModel>> {
    let profiles = ProfileModel::all(db.inner()).await?;
    Ok(profiles)
}

#[tauri::command]
pub async fn profiles_get_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<ProfileModel> {
    let profile = ProfileModel::get_by_id(db.inner(), profile_id)
        .await?
        .context("unknown profile")?;
    Ok(profile)
}

#[tauri::command]
pub async fn profiles_delete_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<()> {
    ProfileModel::delete(db.inner(), profile_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn profiles_update_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
    update: UpdateProfile,
) -> CmdResult<()> {
    let db = db.inner();
    let profile = ProfileModel::get_by_id(db, profile_id)
        .await?
        .context("unknown profile")?;

    profile.update(db, update).await?;
    Ok(())
}
