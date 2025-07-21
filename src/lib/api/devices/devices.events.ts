import { listen } from "@tauri-apps/api/event";

import type { DeviceId, DeviceRequestId } from "../types/devices";

import {
  invalidateDevices,
  invalidateDevicesAll,
  invalidateDeviceRequests,
  invalidateConnectedDevices,
} from "./devices.mutators";

listen<DeviceRequestId>("device_requests:added", () => {
  invalidateDeviceRequests();
});

listen<DeviceRequestId>("device_requests:removed", () => {
  invalidateDeviceRequests();
});

listen<DeviceRequestId>("device_requests:accepted", () => {
  invalidateDevicesAll();
});

listen<DeviceRequestId>("device_requests:declined", () => {
  invalidateDeviceRequests();
});

listen<DeviceId>("device:authenticated", () => {
  invalidateConnectedDevices();
});

listen<DeviceId>("device:disconnected", () => {
  invalidateConnectedDevices();
});

listen<DeviceId>("device:revoked", () => {
  invalidateDevices();
  invalidateConnectedDevices();
});
