import type { TileIcon, TileLabel } from "$lib/api/types/tiles";

export type PropertyInspectorMessage =
  | { type: "SEND_TO_PLUGIN"; message: object }
  | { type: "GET_TILE" }
  | { type: "GET_PROPERTIES" }
  | { type: "SET_PROPERTIES"; properties: object }
  | { type: "GET_PLUGIN_PROPERTIES" }
  | { type: "SET_PLUGIN_PROPERTIES"; properties: object; partial: boolean }
  | { type: "SET_LABEL"; label: TileLabel }
  | { type: "SET_ICON"; icon: TileIcon };
