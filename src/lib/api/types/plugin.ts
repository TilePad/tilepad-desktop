import type { TileId } from "./tiles";
import type { ActionId } from "./actions";
import type { FolderId } from "./folders";
import type { ProfileId } from "./profiles";

export type PluginId = string;

export interface InspectorContext {
  profile_id: ProfileId;
  folder_id: FolderId;

  plugin_id: PluginId;
  action_id: ActionId;

  tile_id: TileId;
}

export function encodeInspectorContext(ctx: InspectorContext): string {
  return (
    ctx.profile_id +
    "-" +
    ctx.folder_id +
    "-" +
    ctx.plugin_id +
    "-" +
    ctx.action_id +
    "-" +
    ctx.tile_id
  );
}

export function isInspectorContextEqual(
  a: InspectorContext,
  b: InspectorContext,
): boolean {
  return (
    a.profile_id === b.profile_id &&
    a.folder_id === b.folder_id &&
    a.plugin_id === b.plugin_id &&
    a.action_id === b.action_id &&
    a.tile_id === b.tile_id
  );
}

export type PluginWithState = Plugin & {
  state: PluginTaskState;
};

export type Plugin = {
  path: string;
  manifest: PluginManifest;
};

export type PluginManifest = {
  plugin: ManifestPlugin;
  bin: ManifestBin | null;
  category: ManifestCategory;
  actions: Record<ActionId, ManifestAction>;
};

export interface ManifestPlugin {
  id: string;
  name: string;
  version: string;
  authors: string[];
  description: string | null;
  icon: string | null;
  internal: boolean | null;
}
export interface ManifestCategory {
  label: string;
  icon: string | null;
}
export interface ManifestAction {
  label: string;
  icon: string | null;
  description: string | null;
  inspector: string | null;
}

export type ManifestBin =
  | { node: { entrypoint: string; version: string } }
  | { native: { os: string; arch: string; path: string }[] };

export enum PluginTaskState {
  NotStarted = "NotStarted",
  Starting = "Starting",
  Unavailable = "Unavailable",
  Running = "Running",
  Error = "Error",
  Stopped = "Stopped",
}
