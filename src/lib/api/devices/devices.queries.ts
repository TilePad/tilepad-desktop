import { createQuery } from "@tanstack/svelte-query";

import { devicesKeys } from "./devices.keys";
import {
  getDevices,
  getDeviceRequests,
  getConnectedDevices,
} from "./devices.requests";

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
