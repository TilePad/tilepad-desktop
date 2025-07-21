import type { QueryClient } from "@tanstack/svelte-query";

import { pluginsKey } from "./plugins.keys";

export function invalidatePluginsQuery(client: QueryClient) {
  client.invalidateQueries({
    queryKey: pluginsKey.root,
    exact: false,
  });
}
