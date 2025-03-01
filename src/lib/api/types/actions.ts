import type { Option } from "./shared";

export type PluginId = string;
export type ActionId = string;

export interface ActionCollection {
  categories: ActionCategory[];
}

export interface ActionCategory {
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
