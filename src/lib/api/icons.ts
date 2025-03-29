import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { IconPack } from "./types/icons";

const iconsKeys = {
  root: ["icons"],
  list: ["icons", "list"],
};

// [REQUESTS] ------------------------------------------------------

function getIconPacks() {
  return invoke<IconPack[]>("icons_get_icon_packs");
}

// [QUERIES] ------------------------------------------------------

export function createIconPacksQuery() {
  return createQuery({
    queryKey: iconsKeys.list,
    queryFn: () => getIconPacks(),
  });
}

// [MUTATIONS] ------------------------------------------------------

// [MUTATORS] ------------------------------------------------------
