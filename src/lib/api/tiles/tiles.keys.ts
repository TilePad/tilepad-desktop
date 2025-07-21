import type { TileId } from "../types/tiles";
import type { FolderId } from "../types/folders";

export const tilesKeys = {
  root: ["tiles"],
  list: (folderId: FolderId | null) => ["tiles", folderId, "list"],
  specific: (folderId: FolderId | null, tileId: TileId | null) => [
    "tiles",
    folderId,
    "specific",
    tileId,
  ],
};
