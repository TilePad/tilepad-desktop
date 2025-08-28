import { createMutation } from "@tanstack/svelte-query";

import type { IconPackId } from "../types/icons";

import { getIconPackBundle } from "./icons_registry.requests";
import { uninstallIconPack, installIconPackBuffer } from "../icons";

export function createInstallIconPackFromRegistry() {
  return createMutation(() => ({
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
  }));
}

export function createUpdateIconPack() {
  return createMutation(() => ({
    mutationFn: async ({
      repo,
      version,
      packId,
      fileName,
    }: {
      repo: string;
      version: string;
      packId: IconPackId;
      fileName: string;
    }) => {
      // Download the new bundle
      const bundle = await getIconPackBundle(repo, version, fileName);

      // Uninstall the current pack
      await uninstallIconPack(packId);

      // Install the new version
      await installIconPackBuffer(bundle);
    },
  }));
}
