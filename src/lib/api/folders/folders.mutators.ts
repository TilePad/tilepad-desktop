import type { QueryClient } from "@tanstack/svelte-query";

import type { ProfileId } from "../types/profiles";

import { foldersKeys } from "./folders.keys";

export function invalidateFoldersList(
  client: QueryClient,
  profileId: ProfileId,
) {
  client.invalidateQueries({
    queryKey: foldersKeys.list(profileId),
    exact: false,
  });
}
