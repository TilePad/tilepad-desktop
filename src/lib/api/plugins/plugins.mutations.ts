import { createMutation } from "@tanstack/svelte-query";

import type { PluginId } from "../types/plugin";

import { uninstallPlugin } from "./plugins.requests";

export function createUninstallPlugin() {
  return createMutation(() => ({
    mutationFn: ({ pluginId }: { pluginId: PluginId }) =>
      uninstallPlugin(pluginId),
  }));
}
