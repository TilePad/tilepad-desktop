import type { FolderId } from "../types/folders";

import { queryClient } from "../client";
import { tilesKeys } from "./tiles.keys";

export function invalidateTilesList(folderId: FolderId) {
  queryClient.invalidateQueries({
    queryKey: tilesKeys.list(folderId),
    exact: false,
  });
}
