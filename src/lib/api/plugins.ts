import { invoke } from "@tauri-apps/api/core";

import type { PluginId, PluginMessageContext } from "./types/plugin";

import { queryClient } from "./client";

export const pluginsKey = {
  root: ["plugins"],
  specific: {
    asset: (pluginId: PluginId | null, asset: string | null) => [
      "plugins",
      pluginId,
      "asset",
      asset,
    ],
  },
};

// [REQUESTS] ------------------------------------------------------

export function sendPluginMessage(
  context: PluginMessageContext,
  message: unknown,
) {
  return invoke<void>("plugins_send_plugin_message", {
    context,
    message,
  });
}

export function openPluginInspector(context: PluginMessageContext) {
  return invoke<void>("plugins_open_inspector", {
    context,
  });
}

export function closePluginInspector(context: PluginMessageContext) {
  return invoke<void>("plugins_close_inspector", {
    context,
  });
}

// [QUERIES] ------------------------------------------------------

// [MUTATORS] ------------------------------------------------------

function invalidatePluginQueries() {
  queryClient.invalidateQueries({
    queryKey: pluginsKey.root,
    exact: false,
  });
}
