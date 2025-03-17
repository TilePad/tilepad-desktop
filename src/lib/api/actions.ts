import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { PluginId } from "./types/plugin";
import type {
  Action,
  ActionId,
  ActionCategory,
  ActionWithCategory,
} from "./types/actions";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";
import { getPluginAssetPath } from "./utils/url";

export const actionsKeys = {
  root: ["actions"],
  list: ["actions", "list"],
  specific: (pluginId: PluginId | null, actionId: ActionId | null) => [
    "actions",
    "specific",
    pluginId,
    actionId,
  ],
};

// [REQUESTS] ------------------------------------------------------

function getActions() {
  return invoke<ActionCategory[]>("actions_get_actions");
}

function getAction(pluginId: PluginId, actionId: ActionId) {
  return invoke<ActionWithCategory>("actions_get_action", {
    pluginId,
    actionId,
  });
}

// [QUERIES] ------------------------------------------------------

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

// [MUTATORS] ------------------------------------------------------

function invalidateActionsList() {
  queryClient.invalidateQueries({
    queryKey: actionsKeys.list,
    exact: false,
  });
}
