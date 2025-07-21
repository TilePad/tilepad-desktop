import { invoke } from "@tauri-apps/api/core";

import type { IconRegistryEntry } from "../types/icons_registry";

export async function getIconRegistry(): Promise<IconRegistryEntry[]> {
  const response = await fetch(
    "https://raw.githubusercontent.com/TilePad/tilepad-plugins/main/icon-packs.json",
  );

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.json();
  return data;
}

export async function getIconPackManifest(
  repo: string,
): Promise<{ version: string }> {
  const manifestURL = `https://raw.githubusercontent.com/${repo}/HEAD/tilepadIcons.json`;

  // Download the manifest
  const manifestResponse = await fetch(manifestURL);
  if (!manifestResponse.ok) {
    throw new Error("error fetching manifest file");
  }

  const manifest = await manifestResponse.json();
  if (!manifest.version) throw new Error("manifest missing version");

  return manifest;
}

export async function getIconPackBundle(
  repo: string,
  version: string,
  fileName: string,
): Promise<ArrayBuffer> {
  return await invoke<ArrayBuffer>("icons_download_bundle", {
    repo,
    version,
    fileName,
  });
}

export async function getIconPackReadme(repo: string) {
  const baseURL = `https://raw.githubusercontent.com/${repo}/HEAD`;
  const url = `${baseURL}/README.md`;
  const response = await fetch(url);

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.text();
  return { readme: data, baseURL };
}
