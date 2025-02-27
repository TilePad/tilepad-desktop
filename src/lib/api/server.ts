import { invoke } from "@tauri-apps/api/core";
import type { ServerConnectionInfo } from "./types/server";

const serverKeys = {
  root: ["server"],
};

export function getConnectionInfo() {
  return invoke<ServerConnectionInfo>("server_get_connection_info");
}
