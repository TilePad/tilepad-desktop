use crate::{
    commands::CmdResult,
    database::{
        entity::profile::{CreateProfile, ProfileId, ProfileModel, UpdateProfile},
        DbPool,
    },
};
use anyhow::Context;
use tauri::State;

/// Get a list of all profiles
#[tauri::command]
pub async fn profiles_get_profiles(db: State<'_, DbPool>) -> CmdResult<Vec<ProfileModel>> {
    let profiles = ProfileModel::all(db.inner()).await?;
    Ok(profiles)
}

/// Get a specific profile
#[tauri::command]
pub async fn profiles_get_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<Option<ProfileModel>> {
    let profile = ProfileModel::get_by_id(db.inner(), profile_id).await?;
    Ok(profile)
}

/// Create a new profile
#[tauri::command]
pub async fn profiles_create_profile(
    db: State<'_, DbPool>,
    create: CreateProfile,
) -> CmdResult<ProfileModel> {
    let db = db.inner();
    let profile = ProfileModel::create(db, create).await?;
    Ok(profile)
}

/// Update a specific profile
#[tauri::command]
pub async fn profiles_update_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
    update: UpdateProfile,
) -> CmdResult<ProfileModel> {
    let db = db.inner();
    let profile = ProfileModel::get_by_id(db, profile_id)
        .await?
        .context("unknown profile")?;

    let profile = profile.update(db, update).await?;
    Ok(profile)
}

/// Delete a specific profile
#[tauri::command]
pub async fn profiles_delete_profile(
    db: State<'_, DbPool>,
    profile_id: ProfileId,
) -> CmdResult<()> {
    ProfileModel::delete(db.inner(), profile_id).await?;
    Ok(())
}
