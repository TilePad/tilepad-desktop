import { invoke } from "@tauri-apps/api/core";

import type {
  PluginId,
  InspectorContext,
  PluginWithState,
} from "./types/plugin";

import { queryClient } from "./client";
import { createQuery } from "@tanstack/svelte-query";

export const pluginsKey = {
  root: ["plugins"],
  list: ["plugins", "list"],
  specific: {
    asset: (pluginId: PluginId | null, asset: string | null) => [
      "plugins",
      "specific",
      pluginId,
      "asset",
      asset,
    ],
  },
};

// [REQUESTS] ------------------------------------------------------

export function getPluginsWithState() {
  return invoke<PluginWithState[]>("plugins_get_plugins");
}

export function sendPluginMessage(context: InspectorContext, message: unknown) {
  return invoke<void>("plugins_send_plugin_message", {
    context,
    message,
  });
}

export function openPluginInspector(context: InspectorContext) {
  return invoke<void>("plugins_open_inspector", {
    context,
  });
}

export function closePluginInspector(context: InspectorContext) {
  return invoke<void>("plugins_close_inspector", {
    context,
  });
}

// [QUERIES] ------------------------------------------------------

export function createPluginsQuery() {
  return createQuery({
    queryKey: pluginsKey.list,
    queryFn: getPluginsWithState,
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidatePluginsQuery() {
  queryClient.invalidateQueries({
    queryKey: pluginsKey.root,
    exact: false,
  });
}
