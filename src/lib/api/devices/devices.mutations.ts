import { createMutation } from "@tanstack/svelte-query";

import type { DeviceId } from "../types/devices";
import type { FolderId } from "../types/folders";
import type { ProfileId } from "../types/profiles";

import { invalidateDevices } from "./devices.mutators";
import {
  revokeDevice,
  setDeviceFolder,
  setDeviceProfile,
} from "./devices.requests";

export function createSetDeviceProfileMutation() {
  return createMutation({
    mutationFn: ({
      deviceId,
      profileId,
    }: {
      deviceId: DeviceId;
      profileId: ProfileId;
    }) => setDeviceProfile(deviceId, profileId),
    onSuccess() {
      invalidateDevices();
    },
  });
}

export function createSetDeviceFolderMutation() {
  return createMutation({
    mutationFn: ({
      deviceId,
      folderId,
    }: {
      deviceId: DeviceId;
      folderId: FolderId;
    }) => setDeviceFolder(deviceId, folderId),
    onSuccess() {
      invalidateDevices();
    },
  });
}

export function createRevokeDeviceFolderMutation() {
  return createMutation({
    mutationFn: ({ deviceId }: { deviceId: DeviceId }) =>
      revokeDevice(deviceId),
    onSuccess() {
      invalidateDevices();
    },
  });
}
