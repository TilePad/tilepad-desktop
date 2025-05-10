import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { PluginId, PluginManifest } from "./types/plugin";
import type { PluginRegistryEntry } from "./types/plugins_registry";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";
import { uninstallPlugin, installPluginBuffer } from "./plugins";

export const pluginRegistryKey = {
  root: ["plugin-registry"],
  list: ["plugin-registry", "list"],
  specificManifest: (repo: string) => ["plugin-registry", "manifest", repo],
  specificReadme: (repo: string) => ["plugin-registry", "readme", repo],
};

// [REQUESTS] ------------------------------------------------------

export async function getPluginRegistry(): Promise<PluginRegistryEntry[]> {
  const response = await fetch(
    "https://raw.githubusercontent.com/TilePad/tilepad-plugins/main/plugins.json",
  );

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.json();
  return data;
}

async function getPluginManifest(repo: string): Promise<PluginManifest> {
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

async function getPluginReadme(repo: string) {
  const baseURL = `https://raw.githubusercontent.com/${repo}/HEAD`;
  const url = `${baseURL}/README.md`;
  const response = await fetch(url);

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.text();
  return { readme: data, baseURL };
}

// [QUERIES] ------------------------------------------------------

export function createPluginRegistryQuery() {
  return createQuery({
    queryKey: pluginRegistryKey.list,
    queryFn: getPluginRegistry,
  });
}

export function fetchPluginRegistry() {
  return queryClient.fetchQuery({
    queryKey: pluginRegistryKey.list,
    queryFn: getPluginRegistry,
  });
}

export function createPluginManifestQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: pluginRegistryKey.specificManifest(r),
        queryFn: () => getPluginManifest(r),
        staleTime: Infinity,
      };
    }),
  );
}

export function fetchPluginManifest(repo: string) {
  return queryClient.fetchQuery({
    queryKey: pluginRegistryKey.specificManifest(repo),
    queryFn: () => getPluginManifest(repo),
    staleTime: Infinity,
  });
}

export function createPluginReadmeQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: pluginRegistryKey.specificReadme(r),
        queryFn: () => getPluginReadme(r),
        staleTime: Infinity,
      };
    }),
  );
}

// [MUTATIONS] ------------------------------------------------------

export function createInstallPluginFromRegistry() {
  return createMutation({
    mutationFn: async ({
      repo,
      version,
    }: {
      repo: string;
      version: string;
    }) => {
      const bundle = await getPluginBundle(repo, version);
      await installPluginBuffer(bundle);
    },
  });
}

export function createUpdatePlugin() {
  return createMutation({
    mutationFn: async ({
      repo,
      version,
      pluginId,
    }: {
      repo: string;
      version: string;
      pluginId: PluginId;
    }) => {
      // Download the new bundle
      const bundle = await getPluginBundle(repo, version);

      // Uninstall the current plugin
      await uninstallPlugin(pluginId);

      // Install the new version
      await installPluginBuffer(bundle);
    },
  });
}
