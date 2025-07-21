import { createQuery } from "@tanstack/svelte-query";

import type { PluginId } from "../types/plugin";
import type { ActionId } from "../types/actions";

import { actionsKeys } from "./actions.keys";
import { runeStore } from "../utils/svelte.svelte";
import { getAction, getActions } from "./actions.requests";

export function createActionsQuery() {
  return createQuery({
    queryKey: actionsKeys.list,
    queryFn: () => getActions(),
  });
}

export function createActionQuery(
  pluginId: () => PluginId | null,
  actionId: () => ActionId | null,
) {
  return createQuery(
    runeStore(() => {
      const pid = pluginId();
      const aid = actionId();

      return {
        enabled: pid !== null && aid !== null,
        queryKey: actionsKeys.specific(pid, aid),
        queryFn: () => getAction(pid!, aid!),
      };
    }),
  );
}
