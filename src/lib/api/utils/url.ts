import type { PluginId } from "../types/plugin";

export function getPluginAssetPath(
  serverURL: string,
  pluginId: PluginId,
  path: string,
) {
  return new URL(`/plugins/${pluginId}/assets/${path}`, serverURL).toString();
}

export function getIconAssetPath(
  serverURL: string,
  packId: PluginId,
  path: string,
) {
  return new URL(`/icons/${packId}/assets/${path}`, serverURL).toString();
}

export function getUploadedIconAssetPath(serverURL: string, path: string) {
  return new URL(`/uploaded-icons/${path}`, serverURL).toString();
}
