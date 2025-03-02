import type { TileId } from "./tiles";
import type { ActionId } from "./actions";
import type { FolderId } from "./folders";
import type { ProfileId } from "./profiles";

export type PluginId = string;

export interface PluginMessageContext {
  profile_id: ProfileId;
  folder_id: FolderId;

  plugin_id: PluginId;
  action_id: ActionId;

  tile_id: TileId;
}
