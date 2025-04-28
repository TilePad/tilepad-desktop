import { invoke } from "@tauri-apps/api/core";

import type { ServerConnectionInfo } from "./types/server";

export function getConnectionInfo() {
  return invoke<ServerConnectionInfo>("server_get_connection_info");
}

export function getLicenses() {
  return invoke<string>("server_get_licenses");
}
