import type { Uuid } from "./shared";
import type { FolderId } from "./folders";
import type { ProfileId } from "./profiles";

export type DeviceId = Uuid;

export type DeviceRequestId = Uuid;

export type DeviceSessionId = Uuid;

export interface DeviceRequest {
  id: DeviceRequestId;
  socket_addr: string;
  session_id: DeviceSessionId;
  device_name: string;
  client_public_key: number[];
}

export interface DeviceModel {
  id: DeviceId;
  name: string;
  config: DeviceConfig;
  order: number;
  profile_id: ProfileId;
  folder_id: FolderId;
  created_at: string;
  last_connected_at: string;
  public_key: number[];
}

export type DeviceConfig = object;

export interface ConnectedDevice {
  device_id: DeviceId;
  session_id: DeviceSessionId;
}
