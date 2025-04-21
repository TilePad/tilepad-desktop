import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { IconRegistryEntry } from "./types/icons_registry";

import { installIconPackBuffer } from "./icons";
import { runeStore } from "./utils/svelte.svelte";

export const iconRegistryKey = {
  root: ["icon-registry"],
  list: ["icon-registry", "list"],
  specificManifest: (repo: string) => ["icon-registry", "manifest", repo],
  specificReadme: (repo: string) => ["icon-registry", "readme", repo],
};

// [REQUESTS] ------------------------------------------------------

export async function getIconRegistry(): Promise<IconRegistryEntry[]> {
  const response = await fetch(
    "https://raw.githubusercontent.com/TilePad/tilepad-plugins/main/icon-packs.json",
  );

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.json();
  return data;
}

async function getIconPackManifest(repo: string): Promise<{ version: string }> {
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

async function getIconPackBundle(
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

async function getIconPackReadme(repo: string) {
  const baseURL = `https://raw.githubusercontent.com/${repo}/HEAD`;
  const url = `${baseURL}/README.md`;
  const response = await fetch(url);

  if (!response.ok) throw new Error("error response from registry");

  const data = await response.text();
  return { readme: data, baseURL };
}

// [QUERIES] ------------------------------------------------------

export function createIconPackRegistryQuery() {
  return createQuery({
    queryKey: iconRegistryKey.list,
    queryFn: getIconRegistry,
  });
}

export function createIconPackManifestQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: iconRegistryKey.specificManifest(r),
        queryFn: () => getIconPackManifest(r),
        staleTime: Infinity,
      };
    }),
  );
}

export function createIconPackReadmeQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: iconRegistryKey.specificReadme(r),
        queryFn: () => getIconPackReadme(r),
        staleTime: Infinity,
      };
    }),
  );
}

// [MUTATIONS] ------------------------------------------------------

export function createInstallIconPackFromRegistry() {
  return createMutation({
    mutationFn: async ({
      repo,
      version,
      fileName,
    }: {
      repo: string;
      version: string;
      fileName: string;
    }) => {
      const bundle = await getIconPackBundle(repo, version, fileName);
      await installIconPackBuffer(bundle);
    },
  });
}
