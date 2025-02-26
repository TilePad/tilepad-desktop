import { invoke } from "@tauri-apps/api/core";
import type { DeviceRequest, DeviceRequestId } from "./types/devices";
import { createQuery } from "@tanstack/svelte-query";
import { queryClient } from "./client";
import { listen } from "@tauri-apps/api/event";

const devicesKeys = {
  requests: ["devices", "requests"],
};

// [REQUESTS] ------------------------------------------------------

function getDeviceRequests() {
  return invoke<DeviceRequest[]>("devices_get_requests");
}

function approveDeviceRequest(requestId: DeviceRequestId) {
  return invoke<void>("devices_approve_request", { requestId });
}

function declineDeviceRequest(requestId: DeviceRequestId) {
  return invoke<void>("devices_decline_request", { requestId });
}

// [QUERIES] ------------------------------------------------------

export function deviceRequestsQuery() {
  return createQuery({
    queryKey: devicesKeys.requests,
    queryFn: getDeviceRequests,
  });
}

// [MUTATORS] ------------------------------------------------------

export function invalidateDeviceRequests() {
  queryClient.invalidateQueries({
    queryKey: devicesKeys.requests,
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
