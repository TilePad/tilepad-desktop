import type { QueryClient } from "@tanstack/svelte-query";

import { actionsKeys } from "./actions.keys";

export function invalidateActionsQueries(client: QueryClient) {
  client.invalidateQueries({
    queryKey: actionsKeys.root,
    exact: false,
  });
}
