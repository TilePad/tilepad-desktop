import { invoke } from "@tauri-apps/api/core";

import type { Plugin, PluginManifest } from "../types/plugin";
import type { PluginRegistryEntry } from "../types/plugins_registry";

import {
  fetchPluginManifest,
  fetchPluginRegistry,
} from "./plugins_registry.queries";

export async function getPluginRegistry(): Promise<PluginRegistryEntry[]> {
  const response = await fetch(
    "https://raw.githubusercontent.com/TilePad/tilepad-plugins/main/plugins.json",
  );

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.json();
  return data;
}

export async function getPluginManifest(repo: string): Promise<PluginManifest> {
  const manifestURL = `https://raw.githubusercontent.com/${repo}/HEAD/.tilepadPlugin/manifest.json`;

  // Download the manifest
  const manifestResponse = await fetch(manifestURL);
  if (!manifestResponse.ok) {
    throw new Error("error fetching manifest file");
  }

  const manifestRaw = await manifestResponse.text();

  // Parse the downloaded manifest
  const manifest = await invoke<PluginManifest>("plugins_parse_manifest", {
    manifest: manifestRaw,
  });

  return manifest;
}

export async function getPluginBundle(
  repo: string,
  version: string,
): Promise<ArrayBuffer> {
  return await invoke<ArrayBuffer>("plugins_download_bundle", {
    repo,
    version,
  });
}

export async function getPluginReadme(repo: string) {
  const baseURL = `https://raw.githubusercontent.com/${repo}/HEAD`;
  const url = `${baseURL}/README.md`;
  const response = await fetch(url);

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.text();
  return { readme: data, baseURL };
}

const UPDATE_CHECK_BATCH_SIZE: number = 5;

export async function getLatestPluginVersions(plugins: Plugin[]) {
  // Load remote available plugins
  const remotePlugins = await fetchPluginRegistry();

  // Find the remote plugins that are currently installed
  const installedPlugins = remotePlugins.filter((remotePlugin) => {
    return plugins.find(
      (plugin) => remotePlugin.id === plugin.manifest.plugin.id,
    );
  });

  // List of latest versions
  const latestVersions = [];

  // Process in batches
  for (let i = 0; i < installedPlugins.length; i += UPDATE_CHECK_BATCH_SIZE) {
    const remotePluginSet = installedPlugins.slice(
      i,
      i + UPDATE_CHECK_BATCH_SIZE,
    );

    const remotePluginManifests = await Promise.all(
      remotePluginSet.map(async (remotePlugin) => {
        const manifest = await fetchPluginManifest(remotePlugin.repo);
        return { manifest, remotePlugin };
      }),
    );

    latestVersions.push(...remotePluginManifests);
  }

  return latestVersions;
}
