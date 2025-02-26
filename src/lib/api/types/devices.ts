import type { ProfileId } from "./profiles";
import type { Option, Uuid } from "./shared";

export type DeviceId = Uuid;

export type DeviceRequestId = Uuid;

export type DeviceSessionId = Uuid;

export interface DeviceRequest {
  id: DeviceRequestId;
  socket_addr: string;
  session_id: DeviceSessionId;
  device_name: string;
}

export interface DeviceModel {
  id: DeviceId;
  name: string;
  config: DeviceConfig;
  order: number;
  profile_id: Option<ProfileId>;
  created_at: string;
  last_connected_at: string;
}

export interface DeviceConfig {}

export interface ConnectedDevice {
  device_id: DeviceId;
  session_id: DeviceSessionId;
}
