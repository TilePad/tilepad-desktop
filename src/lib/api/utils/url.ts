import type { PluginId } from "../types/plugin";

const BACKEND_URL_PROD = "http://localhost:59371/";
const BACKEND_URL_DEV = "http://localhost:59371/";

export function getBackendURL() {
  return import.meta.env.DEV ? BACKEND_URL_DEV : BACKEND_URL_PROD;
}

export function getPluginAssetPath(pluginId: PluginId, path: string) {
  return new URL(
    `/plugins/${pluginId}/assets/${path}`,
    getBackendURL(),
  ).toString();
}

export function getIconAssetPath(packId: PluginId, path: string) {
  return new URL(`/icons/${packId}/assets/${path}`, getBackendURL()).toString();
}
