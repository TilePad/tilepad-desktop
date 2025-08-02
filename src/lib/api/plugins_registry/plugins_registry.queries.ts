import { createQuery } from "@tanstack/svelte-query";

import { queryClient } from "../client";
import { pluginRegistryKey } from "./plugins_registry.keys";
import {
  getPluginReadme,
  getPluginManifest,
  getPluginRegistry,
} from "./plugins_registry.requests";

export function createPluginRegistryQuery() {
  return createQuery(() => ({
    queryKey: pluginRegistryKey.list,
    queryFn: getPluginRegistry,
  }));
}

export function fetchPluginRegistry() {
  return queryClient.fetchQuery({
    queryKey: pluginRegistryKey.list,
    queryFn: getPluginRegistry,
  });
}

export function createPluginManifestQuery(repo: () => string) {
  return createQuery(() => {
    const r = repo();
    return {
      queryKey: pluginRegistryKey.specificManifest(r),
      queryFn: () => getPluginManifest(r),
      staleTime: Infinity,
    };
  });
}

export function fetchPluginManifest(repo: string) {
  return queryClient.fetchQuery({
    queryKey: pluginRegistryKey.specificManifest(repo),
    queryFn: () => getPluginManifest(repo),
    staleTime: Infinity,
  });
}

export function createPluginReadmeQuery(repo: () => string) {
  return createQuery(() => {
    const r = repo();
    return {
      queryKey: pluginRegistryKey.specificReadme(r),
      queryFn: () => getPluginReadme(r),
      staleTime: Infinity,
    };
  });
}
