import type { DeepPartial } from "$lib/types";

import type { Uuid } from "./shared";
import type { PluginId } from "./plugin";
import type { ActionId } from "./actions";
import type { FolderId } from "./folders";
import type { IconPackId } from "./icons";

export type TileId = Uuid;

export enum UpdateKind {
  Reset = "Reset",
  User = "User",
  Program = "Program",
}

export interface TileModel {
  id: TileId;
  config: TileConfig;
  properties: object;
  folder_id: FolderId;
  row: number;
  column: number;
}

export interface TileConfig {
  plugin_id: PluginId;
  action_id: ActionId;
  icon: TileIcon;
  icon_options: TileIconOptions;
  label: TileLabel;
  user_flags: UserFlags;
}

export interface TileIconOptions {
  padding: number;
  background_color: string;
}

export type TileIconNone = object;
export type TileIconPluginIcon = {
  plugin_id: PluginId;
  icon: string;
};
export type TileIconIconPack = {
  pack_id: IconPackId;
  path: string;
};
export type TileIconUploaded = {
  path: string;
};

export enum TileIconType {
  None = "None",
  PluginIcon = "PluginIcon",
  IconPack = "IconPack",
  Uploaded = "Uploaded",
}

export type TileIcon =
  | ({ type: TileIconType.None } & TileIconNone)
  | ({ type: TileIconType.PluginIcon } & TileIconPluginIcon)
  | ({ type: TileIconType.IconPack } & TileIconIconPack)
  | ({ type: TileIconType.Uploaded } & TileIconUploaded);

export type CreateTile = Omit<TileModel, "id" | "config"> & {
  // When creating a tile only the plugin_id and action_id are
  // required parts of the configuration
  config: Pick<TileConfig, "plugin_id" | "action_id"> &
    DeepPartial<Omit<TileConfig, "plugin_id" | "action_id">>;
};

export type UpdateTile = Partial<Omit<TileModel, "id" | "config">> & {
  //
  config: Pick<TileConfig, "plugin_id" | "action_id"> &
    DeepPartial<Omit<TileConfig, "plugin_id" | "action_id">>;
};

export interface TileLabel {
  enabled: boolean;
  label: string;
  align: LabelAlign;
  font: string;
  font_size: number;
  bold: boolean;
  italic: boolean;
  underline: boolean;
  outline: boolean;
  color: string;
  outline_color: string;
}

export enum LabelAlign {
  Bottom = "Bottom",
  Middle = "Middle",
  Top = "Top",
}

export interface UserFlags {
  icon: boolean;
  label: boolean;
}
