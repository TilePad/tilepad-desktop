import { queryClient } from "../client";
import { devicesKeys } from "./devices.keys";

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
