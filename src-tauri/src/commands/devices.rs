use tauri::State;

use crate::device::{DeviceRequest, Devices};

/// Gets a list of current device approval requests
#[tauri::command]
pub fn devices_get_requests(devices: State<'_, Devices>) -> Vec<DeviceRequest> {
    devices.get_device_requests()
}
