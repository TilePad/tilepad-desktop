import { invoke } from "@tauri-apps/api/core";
import { createQuery, createMutation } from "@tanstack/svelte-query";

import type { ProfileId, ProfileModel, CreateProfile } from "./types/profiles";

import { queryClient } from "./client";
import { runeStore } from "./utils/svelte.svelte";

const profilesKeys = {
  root: ["profiles"],
  list: ["profiles", "list"],
  specific: (profileId: ProfileId | null) => ["profiles", "profile", profileId],
};

// [REQUESTS] ------------------------------------------------------

function getProfiles() {
  return invoke<ProfileModel[]>("profiles_get_profiles");
}

function getProfile(profileId: ProfileId) {
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

// [QUERIES] ------------------------------------------------------

export function createProfilesQuery() {
  return createQuery({
    queryKey: profilesKeys.list,
    queryFn: getProfiles,
  });
}

export function createProfileQuery(profileId: () => ProfileId | null) {
  return createQuery(
    runeStore(() => {
      const id = profileId();
      return {
        queryKey: profilesKeys.specific(id),
        queryFn: () => getProfile(id!),
      };
    }),
  );
}

// [MUTATIONS] ------------------------------------------------------

export function createCreateProfileMutation() {
  return createMutation({
    mutationFn: ({ create }: { create: CreateProfile }) =>
      createProfile(create),
  });
}

// [MUTATORS] ------------------------------------------------------

function invalidateProfilesList() {
  queryClient.invalidateQueries({
    queryKey: profilesKeys.list,
    exact: false,
  });
}
