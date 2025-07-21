import { invoke } from "@tauri-apps/api/core";

import type { ProfileId, ProfileModel, CreateProfile } from "../types/profiles";

import { queryClient } from "../client";
import { profilesKeys } from "./profiles.keys";
import { invalidateProfilesList } from "./profiles.mutators";

export function getProfiles() {
  return invoke<ProfileModel[]>("profiles_get_profiles");
}

export function getProfile(profileId: ProfileId) {
  return invoke<ProfileModel | null>("profiles_get_profile", { profileId });
}

export async function createProfile(create: CreateProfile) {
  const profile = await invoke<ProfileModel>("profiles_create_profile", {
    create,
  });

  invalidateProfilesList();
  queryClient.setQueryData(profilesKeys.specific(profile.id), profile);

  return profile;
}

export async function setProfileName(profileId: ProfileId, name: string) {
  const profile = await invoke<ProfileModel>("profiles_set_name", {
    profileId,
    name,
  });

  invalidateProfilesList();
  queryClient.setQueryData(profilesKeys.specific(profile.id), profile);

  return profile;
}

export async function deleteProfile(profileId: ProfileId) {
  await invoke("profiles_delete_profile", { profileId });

  invalidateProfilesList();
}
