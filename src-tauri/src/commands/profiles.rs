use std::sync::Arc;

use crate::{
    commands::CmdResult,
    database::{
        DbPool,
        entity::{
            device::{DeviceModel, UpdateDevice},
            folder::FolderModel,
            profile::{CreateProfile, ProfileId, ProfileModel, UpdateProfile},
        },
    },
    device::Devices,
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
    devices: State<'_, Arc<Devices>>,
    profile_id: ProfileId,
) -> CmdResult<()> {
    let db = db.inner();

    // Obtain the default profile and folder
    let default_profile = ProfileModel::get_default_profile(db)
        .await?
        .context("default profile is missing")?;

    let default_folder = FolderModel::get_default(db, default_profile.id)
        .await?
        .context("default folder is missing")?;

    // First all devices using the profile must be updated to use the default profile
    let profile_devices = DeviceModel::all_by_profile(db, profile_id).await?;
    for device in profile_devices {
        device
            .update(
                db,
                UpdateDevice {
                    profile_id: Some(default_profile.id),
                    folder_id: Some(default_folder.id),
                    ..Default::default()
                },
            )
            .await?;
    }

    // Update the actual device sessions
    devices.update_devices_tiles(default_folder.id).await?;

    // Delete the profile itself
    ProfileModel::delete(db, profile_id).await?;

    Ok(())
}
