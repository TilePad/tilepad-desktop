import type { QueryClient } from "@tanstack/svelte-query";

import { iconsKeys } from "./icons.keys";

export function invalidateIconPacksQuery(client: QueryClient) {
  client.invalidateQueries({
    queryKey: iconsKeys.list,
    exact: false,
  });
}
