import type { Option } from "./shared";
import type { PluginId } from "./plugin";

export type ActionId = string;

export interface ActionCollection {
  categories: ActionCategory[];
}

export interface ActionCategory {
  plugin_id: PluginId;

  label: string;
  icon: Option<string>;
  actions: Action[];
}

export interface Action {
  plugin_id: PluginId;
  action_id: ActionId;

  label: string;
  icon: Option<string>;
  description: Option<string>;
}
