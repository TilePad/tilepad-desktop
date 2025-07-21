import type { PluginId } from "../types/plugin";

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
