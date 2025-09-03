import { createMutation } from "@tanstack/svelte-query";

import type { PluginId } from "../types/plugin";

import { getPluginBundle } from "./plugins_registry.requests";
import {
  uninstallPlugin,
  installPluginBuffer,
} from "../plugins/plugins.requests";

export function createInstallPluginFromRegistry(pluginId: () => string) {
  return createMutation(() => {
    const id = pluginId();
    return {
      mutationKey: ["install-plugin", id],
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
    };
  });
}

export function createUpdatePlugin() {
  return createMutation(() => ({
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
  }));
}
