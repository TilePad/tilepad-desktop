export type DisplayMessage =
  | { type: "SEND_TO_PLUGIN"; message: object }
  | { type: "GET_TILE" };
