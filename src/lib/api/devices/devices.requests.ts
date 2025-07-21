import { invoke } from "@tauri-apps/api/core";

import type { FolderId } from "../types/folders";
import type { ProfileId } from "../types/profiles";
import type {
  DeviceId,
  DeviceModel,
  DeviceRequest,
  ConnectedDevice,
  DeviceRequestId,
} from "../types/devices";

import { invalidateDevices } from "./devices.mutators";

export function getDeviceRequests() {
  return invoke<DeviceRequest[]>("devices_get_requests");
}

export function getDevices() {
  return invoke<DeviceModel[]>("devices_get_devices");
}

export function getConnectedDevices() {
  return invoke<ConnectedDevice[]>("devices_get_connected_devices");
}

export function approveDeviceRequest(requestId: DeviceRequestId) {
  return invoke<void>("devices_approve_request", { requestId });
}

export function declineDeviceRequest(requestId: DeviceRequestId) {
  return invoke<void>("devices_decline_request", { requestId });
}

export function revokeDevice(deviceId: DeviceId) {
  return invoke<void>("devices_revoke_device", { deviceId });
}

export async function setDeviceProfile(
  deviceId: DeviceId,
  profileId: ProfileId,
) {
  await invoke<void>("devices_set_device_profile", { deviceId, profileId });
  invalidateDevices();
}

export async function setDeviceFolder(deviceId: DeviceId, folderId: FolderId) {
  await invoke<void>("devices_set_device_folder", { deviceId, folderId });
  invalidateDevices();
}
