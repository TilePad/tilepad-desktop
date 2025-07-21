import type { PluginId } from "../types/plugin";
import type { ActionId } from "../types/actions";

export const actionsKeys = {
  root: ["actions"],
  list: ["actions", "list"],
  specific: (pluginId: PluginId | null, actionId: ActionId | null) => [
    "actions",
    "specific",
    pluginId,
    actionId,
  ],
};
