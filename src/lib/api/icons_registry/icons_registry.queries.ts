import { createQuery } from "@tanstack/svelte-query";

import { runeStore } from "../utils/svelte.svelte";
import { iconRegistryKey } from "./icons_registry.keys";
import {
  getIconRegistry,
  getIconPackReadme,
  getIconPackManifest,
} from "./icons_registry.requests";

export function createIconPackRegistryQuery() {
  return createQuery({
    queryKey: iconRegistryKey.list,
    queryFn: getIconRegistry,
  });
}

export function createIconPackManifestQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: iconRegistryKey.specificManifest(r),
        queryFn: () => getIconPackManifest(r),
        staleTime: Infinity,
      };
    }),
  );
}

export function createIconPackReadmeQuery(repo: () => string) {
  return createQuery(
    runeStore(() => {
      const r = repo();
      return {
        queryKey: iconRegistryKey.specificReadme(r),
        queryFn: () => getIconPackReadme(r),
        staleTime: Infinity,
      };
    }),
  );
}
