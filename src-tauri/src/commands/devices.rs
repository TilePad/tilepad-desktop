use tauri::State;

use crate::{
    database::{
        entity::device::{DeviceId, DeviceModel},
        DbPool,
    },
    device::{ConnectedDevice, DeviceRequest, DeviceRequestId, Devices},
};

use super::CmdResult;

/// Gets a list of current device approval requests
#[tauri::command]
pub fn devices_get_requests(devices: State<'_, Devices>) -> Vec<DeviceRequest> {
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
pub fn devices_get_connected_devices(devices: State<'_, Devices>) -> Vec<ConnectedDevice> {
    devices.get_connected_devices()
}

/// Approve a specific device request
#[tauri::command]
pub async fn devices_approve_request(
    request_id: DeviceRequestId,
    devices: State<'_, Devices>,
) -> CmdResult<()> {
    devices.approve_device_request(request_id).await?;
    Ok(())
}

/// Deny a specific device request
#[tauri::command]
pub async fn devices_decline_request(
    request_id: DeviceRequestId,
    devices: State<'_, Devices>,
) -> CmdResult<()> {
    devices.decline_device_request(request_id)?;
    Ok(())
}

/// Deny a specific device request
#[tauri::command]
pub async fn devices_revoke_device(
    device_id: DeviceId,
    devices: State<'_, Devices>,
) -> CmdResult<()> {
    devices.revoke_device(device_id).await?;
    Ok(())
}
