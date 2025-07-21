import { listen } from "@tauri-apps/api/event";

import type { PluginId } from "../types/plugin";

import { queryClient } from "../client";
import { invalidatePluginsQuery } from "./plugins.mutators";

listen<PluginId>("plugins:loaded", ({ payload: plugin_id }) => {
  invalidatePluginsQuery(queryClient);
});

listen<PluginId>("plugins:unloaded", ({ payload: plugin_id }) => {
  invalidatePluginsQuery(queryClient);
});

listen<{ plugin_id: PluginId; state: string }>(
  "plugins:task_state_changed",
  ({ payload: { plugin_id, state } }) => {
    invalidatePluginsQuery(queryClient);
  },
);
