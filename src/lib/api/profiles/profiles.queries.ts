import { createQuery } from "@tanstack/svelte-query";

import type { ProfileId } from "../types/profiles";

import { profilesKeys } from "./profiles.keys";
import { getProfile, getProfiles } from "./profiles.requests";

export function createProfilesQuery() {
  return createQuery(() => ({
    queryKey: profilesKeys.list,
    queryFn: getProfiles,
  }));
}

export function createProfileQuery(profileId: () => ProfileId | null) {
  return createQuery(() => {
    const id = profileId();
    return {
      queryKey: profilesKeys.specific(id),
      queryFn: () => getProfile(id!),
    };
  });
}
