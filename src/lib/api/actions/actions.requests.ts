import { invoke } from "@tauri-apps/api/core";

import type { PluginId } from "../types/plugin";
import type {
  ActionId,
  ActionCategory,
  ActionWithCategory,
} from "../types/actions";

export function getActions() {
  return invoke<ActionCategory[]>("actions_get_actions");
}

export function getAction(pluginId: PluginId, actionId: ActionId) {
  return invoke<ActionWithCategory | null>("actions_get_action", {
    pluginId,
    actionId,
  });
}
