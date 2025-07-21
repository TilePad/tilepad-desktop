import { createMutation } from "@tanstack/svelte-query";

import type { CreateProfile } from "../types/profiles";

import { createProfile } from "./profiles.requests";

export function createCreateProfileMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateProfile }) =>
      createProfile(create),
  });
}
