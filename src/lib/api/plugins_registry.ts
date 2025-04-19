import { invoke } from "@tauri-apps/api/core";
import { fetch as tauriFetch } from "@tauri-apps/plugin-http";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { PluginManifest } from "./types/plugin";
import type { PluginRegistryEntry } from "./types/plugins_registry";

import { runeStore } from "./utils/svelte.svelte";
import { installPluginBlob as installPluginBuffer } from "./plugins";

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
  const manifestURL = `https://raw.githubusercontent.com/${repo}/HEAD/.tilepadPlugin/manifest.toml`;

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

async function getPluginBundle(
  repo: string,
  version: string,
): Promise<ArrayBuffer> {
  const bundleURL = `https://github.com/${repo}/releases/download/${version}/plugin.tilepadPlugin`;
  // Download the manifest
  const bundleResponse = await tauriFetch(bundleURL);
  if (!bundleResponse.ok) {
    throw new Error("error fetching bundle file");
  }

  const bundleBlob = await bundleResponse.arrayBuffer();
  return bundleBlob;
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
