import { createMutation, getQueryClientContext } from "@tanstack/svelte-query";

import type { IconPackId } from "../types/icons";

import { uninstallIconPack } from "./icons.requests";
import { invalidateIconPacksQuery } from "./icons.mutators";

export function createUninstallIconPackMutation() {
  const queryClient = getQueryClientContext();

  return createMutation({
    mutationFn: ({ packId }: { packId: IconPackId }) =>
      uninstallIconPack(packId),
    onSuccess() {
      invalidateIconPacksQuery(queryClient);
    },
  });
}
