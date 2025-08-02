import { createMutation } from "@tanstack/svelte-query";

import { installIconPackBuffer } from "../icons";
import { getIconPackBundle } from "./icons_registry.requests";

export function createInstallIconPackFromRegistry() {
  return createMutation(() => ({
    mutationFn: async ({
      repo,
      version,
      fileName,
    }: {
      repo: string;
      version: string;
      fileName: string;
    }) => {
      const bundle = await getIconPackBundle(repo, version, fileName);
      await installIconPackBuffer(bundle);
    },
  }));
}
