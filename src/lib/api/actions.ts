import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { ActionCategory } from "./types/actions";

import { queryClient } from "./client";

export const actionsKeys = {
  root: ["actions"],
  list: ["actions", "list"],
};

// [REQUESTS] ------------------------------------------------------

function getActions() {
  return invoke<ActionCategory[]>("actions_get_actions");
}

// [QUERIES] ------------------------------------------------------

export function createActionsQuery() {
  return createQuery({
    queryKey: actionsKeys.list,
    queryFn: () => getActions(),
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidateActionsList() {
  queryClient.invalidateQueries({
    queryKey: actionsKeys.list,
    exact: false,
  });
}
