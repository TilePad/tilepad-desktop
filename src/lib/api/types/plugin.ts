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
