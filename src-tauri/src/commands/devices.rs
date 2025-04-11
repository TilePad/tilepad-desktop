use std::sync::Arc;

use tauri::State;

use crate::{
    database::{
        DbPool,
        entity::{
            device::{DeviceId, DeviceModel},
            folder::FolderId,
            profile::ProfileId,
        },
    },
    device::{ConnectedDevice, DeviceRequest, DeviceRequestId, Devices},
};

use super::CmdResult;

/// Gets a list of current device approval requests
#[tauri::command]
pub fn devices_get_requests(devices: State<'_, Arc<Devices>>) -> Vec<DeviceRequest> {
    devices.get_device_requests()
}

/// Deny a specific device request
#[tauri::command]
pub async fn devices_get_devices(db: State<'_, DbPool>) -> CmdResult<Vec<DeviceModel>> {
    let devices = DeviceModel::all(db.inner()).await?;
    Ok(devices)
}

/// Deny a specific device request
#[tauri::command]
pub fn devices_get_connected_devices(devices: State<'_, Arc<Devices>>) -> Vec<ConnectedDevice> {
    devices.get_connected_devices()
}

/// Approve a specific device request
#[tauri::command]
pub async fn devices_approve_request(
    request_id: DeviceRequestId,
    devices: State<'_, Arc<Devices>>,
) -> CmdResult<()> {
    devices.approve_device_request(request_id).await?;
    Ok(())
}

/// Deny a specific device request
#[tauri::command]
pub async fn devices_decline_request(
    request_id: DeviceRequestId,
    devices: State<'_, Arc<Devices>>,
) -> CmdResult<()> {
    devices.decline_device_request(request_id)?;
    Ok(())
}

/// Deny a specific device request
#[tauri::command]
pub async fn devices_revoke_device(
    device_id: DeviceId,
    devices: State<'_, Arc<Devices>>,
) -> CmdResult<()> {
    devices.revoke_device(device_id).await?;
    Ok(())
}

/// Set the current profile for a device
#[tauri::command]
pub async fn devices_set_device_profile(
    device_id: DeviceId,
    profile_id: ProfileId,
    devices: State<'_, Arc<Devices>>,
) -> CmdResult<()> {
    devices.update_device_profile(device_id, profile_id).await?;
    Ok(())
}

/// Set the current folder for a device
#[tauri::command]
pub async fn devices_set_device_folder(
    device_id: DeviceId,
    folder_id: FolderId,
    devices: State<'_, Arc<Devices>>,
) -> CmdResult<()> {
    devices.update_device_folder(device_id, folder_id).await?;
    Ok(())
}
