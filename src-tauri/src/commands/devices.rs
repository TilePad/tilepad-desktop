use tauri::State;

use crate::device::{DeviceRequest, DeviceRequestId, Devices};

use super::CmdResult;

/// Gets a list of current device approval requests
#[tauri::command]
pub fn devices_get_requests(devices: State<'_, Devices>) -> Vec<DeviceRequest> {
    devices.get_device_requests()
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
