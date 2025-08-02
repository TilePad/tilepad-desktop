import { createQuery } from "@tanstack/svelte-query";

import { pluginsKey } from "./plugins.keys";
import { getPluginsWithState } from "./plugins.requests";

export function createPluginsQuery() {
  return createQuery(() => ({
    queryKey: pluginsKey.list,
    queryFn: getPluginsWithState,
  }));
}
