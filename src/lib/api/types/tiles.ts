import type { Uuid } from "./shared";
import type { PluginId } from "./plugin";
import type { ActionId } from "./actions";
import type { FolderId } from "./folders";

export type TileId = Uuid;

export interface TileModel {
  id: TileId;
  config: TileConfig;
  folder_id: FolderId;
  row: number;
  column: number;
}

export interface TileConfig {
  plugin_id: PluginId;
  action_id: ActionId;
  icon: TileIcon;
  properties: object;
}

export type TileIconNone = object;
export type TileIconPluginIcon = {
  plugin_id: PluginId;
  icon: string;
};

export enum TileIconType {
  None = "None",
  PluginIcon = "PluginIcon",
}

export type TileIcon =
  | ({ type: TileIconType.None } & TileIconNone)
  | ({ type: TileIconType.PluginIcon } & TileIconPluginIcon);

export type CreateTile = Omit<TileModel, "id">;

export type UpdateTile = Partial<Omit<TileModel, "id">>;
