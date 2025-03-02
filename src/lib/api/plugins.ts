import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { PluginId, PluginMessageContext } from "./types/plugin";

import { queryClient } from "./client";
import { getPluginAssetPath } from "./utils/url";
import { runeStore } from "./utils/svelte.svelte";

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

async function getPluginAsset(pluginId: PluginId, asset: string) {
  const res = await fetch(getPluginAssetPath(pluginId, asset));
  if (!res.ok) throw new Error("Failed to load plugin asset");
  const blob = await res.blob();
  return blob;
}

// [QUERIES] ------------------------------------------------------

export function createPluginAssetTextQuery(
  pluginId: () => PluginId | null,
  asset: () => string | null,
) {
  return createQuery(
    runeStore(() => {
      const pid = pluginId();
      const as = asset();

      return {
        enabled: pid !== null && as !== null,
        queryKey: pluginsKey.specific.asset(pid, as),
        queryFn: async () => {
          const blob = await getPluginAsset(pid!, as!);
          const text = await blob.text();
          return text;
        },
      };
    }),
  );
}

// [MUTATORS] ------------------------------------------------------

function invalidatePluginQueries() {
  queryClient.invalidateQueries({
    queryKey: pluginsKey.root,
    exact: false,
  });
}
