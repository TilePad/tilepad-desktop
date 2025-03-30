import { invoke } from "@tauri-apps/api/core";
import { createQuery } from "@tanstack/svelte-query";

import type { IconPack, IconPackId } from "./types/icons";

import { queryClient } from "./client";

const iconsKeys = {
  root: ["icons"],
  list: ["icons", "list"],
};

// [REQUESTS] ------------------------------------------------------

function getIconPacks() {
  return invoke<IconPack[]>("icons_get_icon_packs");
}

export async function installIconPack(file: File) {
  const data = await file.arrayBuffer();

  await invoke<void>("icons_install_icon_pack", {
    data,
  });

  invalidateIconPacksQuery();
}
export async function uninstallIconPack(packId: IconPackId) {
  await invoke<void>("icons_uninstall_icon_pack", {
    packId,
  });

  invalidateIconPacksQuery();
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

function invalidateIconPacksQuery() {
  queryClient.invalidateQueries({
    queryKey: iconsKeys.list,
    exact: false,
  });
}
