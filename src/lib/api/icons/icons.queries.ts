import { createQuery } from "@tanstack/svelte-query";

import { iconsKeys } from "./icons.keys";
import { getIconPacks } from "./icons.requests";

export function createIconPacksQuery() {
  return createQuery(() => ({
    queryKey: iconsKeys.list,
    queryFn: getIconPacks,
  }));
}
