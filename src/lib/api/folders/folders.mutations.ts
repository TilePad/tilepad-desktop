import { createMutation } from "@tanstack/svelte-query";

import type { CreateFolder } from "../types/folders";

import { createFolder } from "./folders.requests";

export function createCreateFolderMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateFolder }) => createFolder(create),
  });
}
