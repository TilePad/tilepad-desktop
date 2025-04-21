import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { createQuery } from "@tanstack/svelte-query";

import type {
  PluginId,
  PluginWithState,
  InspectorContext,
} from "./types/plugin";

import { queryClient } from "./client";

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

export function getPluginProperties(pluginId: PluginId) {
  return invoke<object>("plugins_get_plugin_properties", { pluginId });
}

export function setPluginProperties(
  pluginId: PluginId,
  properties: object,
  partial: boolean = true,
) {
  return invoke<void>("plugins_set_plugin_properties", {
    pluginId,
    properties,
    partial,
  });
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

export async function stopPluginTask(pluginId: PluginId) {
  await invoke<void>("plugins_stop_plugin_task", {
    pluginId,
  });

  invalidatePluginsQuery();
}

export async function startPluginTask(pluginId: PluginId) {
  await invoke<void>("plugins_start_plugin_task", {
    pluginId,
  });

  invalidatePluginsQuery();
}

export async function restartPluginTask(pluginId: PluginId) {
  await invoke<void>("plugins_restart_plugin_task", {
    pluginId,
  });

  invalidatePluginsQuery();
}

export async function reloadPlugin(pluginId: PluginId) {
  await invoke<void>("plugins_reload_plugin", {
    pluginId,
  });

  invalidatePluginsQuery();
}

export async function installPlugin(file: File) {
  const data = await file.arrayBuffer();
  await installPluginBuffer(data);
}

export async function installPluginBuffer(data: ArrayBuffer) {
  await invoke<void>("plugins_install_plugin_manual", {
    data,
  });

  invalidatePluginsQuery();
}

export async function uninstallPlugin(pluginId: PluginId) {
  await invoke<void>("plugins_uninstall_plugin", {
    pluginId,
  });

  invalidatePluginsQuery();
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

// [LISTENERS] ------------------------------------------------------

listen<PluginId>("plugins:loaded", ({ payload: plugin_id }) => {
  invalidatePluginsQuery();
});

listen<PluginId>("plugins:unloaded", ({ payload: plugin_id }) => {
  invalidatePluginsQuery();
});

listen<{ plugin_id: PluginId; state: string }>(
  "plugins:task_state_changed",
  ({ payload: { plugin_id, state } }) => {
    invalidatePluginsQuery();
  },
);
