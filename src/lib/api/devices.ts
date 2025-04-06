import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createQuery } from "@tanstack/svelte-query";

import type {
  DeviceId,
  DeviceModel,
  DeviceRequest,
  ConnectedDevice,
  DeviceRequestId,
} from "./types/devices";

import { queryClient } from "./client";

const devicesKeys = {
  root: ["devices"],
  requests: ["devices", "requests"],
  devices: ["devices", "devices"],
  connectedDevices: ["devices", "connected"],
};

// [REQUESTS] ------------------------------------------------------

function getDeviceRequests() {
  return invoke<DeviceRequest[]>("devices_get_requests");
}

function getDevices() {
  return invoke<DeviceModel[]>("devices_get_devices");
}

function getConnectedDevices() {
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

// [QUERIES] ------------------------------------------------------

export function deviceRequestsQuery() {
  return createQuery({
    queryKey: devicesKeys.requests,
    queryFn: getDeviceRequests,
  });
}

export function devicesQuery() {
  return createQuery({
    queryKey: devicesKeys.devices,
    queryFn: getDevices,
  });
}

export function connectedDevicesQuery() {
  return createQuery({
    queryKey: devicesKeys.connectedDevices,
    queryFn: getConnectedDevices,
  });
}

// [MUTATORS] ------------------------------------------------------

export function invalidateDeviceRequests() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.requests,
    exact: false,
  });
}

export function invalidateDevices() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.devices,
    exact: false,
  });
}

export function invalidateConnectedDevices() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.connectedDevices,
    exact: false,
  });
}

export function invalidateDevicesAll() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.root,
    exact: false,
  });
}

// [LISTENERS] ------------------------------------------------------

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
