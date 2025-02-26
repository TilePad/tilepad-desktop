import type { Uuid } from "./shared";

export type DeviceRequestId = Uuid;

export type DeviceSessionId = Uuid;

export interface DeviceRequest {
  id: DeviceRequestId;
  socket_addr: string;
  session_id: DeviceSessionId;
  device_name: string;
}
