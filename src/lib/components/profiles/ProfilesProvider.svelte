<script module lang="ts">
  const currentProfileKey = "currentProfileId";

  const profileStore = new Context<ProfileContext>("profileStore");

  export function getPersistedProfileId() {
    return localStorage.getItem(currentProfileKey) ?? undefined;
  }

  function setPersistedProfileId(profileId: string) {
    localStorage.setItem(currentProfileKey, profileId);
  }

  interface ProfileContext {
    profile(): ProfileModel;
    setProfileId: (value: ProfileId) => void;
  }

  export function getProfileContext(): ProfileContext {
    return profileStore.get();
  }

  function getDefaultProfile(
    profiles: ProfileModel[],
  ): ProfileModel | undefined {
    return profiles.find((profile) => profile.default);
  }
</script>

<!-- Provider for profiles, loads the active default profile, creates one if missing -->
<script lang="ts">
  import type { ProfileId, ProfileModel } from "$lib/api/types/profiles";

  import { type Snippet } from "svelte";
  import { watch, Context } from "runed";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { getErrorMessage } from "$lib/api/utils/error";
  import {
    createProfileQuery,
    createProfilesQuery,
    createCreateProfileMutation,
  } from "$lib/api/profiles";

  import Aside from "../Aside.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";

  type Props = {
    children?: Snippet | undefined;
  };

  const { children }: Props = $props();

  const i18n = i18nContext.get();

  // State for the actively selected profile
  let profileId: ProfileId | undefined = $state(getPersistedProfileId());

  const profilesQuery = createProfilesQuery();
  const profilesQueryData = $derived(profilesQuery.data);

  const createProfile = createCreateProfileMutation();

  const profileQuery = createProfileQuery(() => profileId ?? null);
  const profile = $derived(profileQuery.data);

  profileStore.set({
    profile: () => profile!,
    setProfileId: (value: string) => {
      profileId = value;
      setPersistedProfileId(value);
    },
  });

  watch(
    () => profilesQueryData,
    (profiles) => {
      // Profiles are loaded yet ignore
      if (profiles === undefined) return;

      // Check if the current profile is a valid profile to use
      if (profiles.length > 0 && profileId !== undefined) {
        const currentProfile = profiles.find(
          (profile) => profile.id === profileId,
        );
        if (currentProfile !== undefined) {
          return;
        }
      }

      // Try and set the profile to the default
      if (profiles.length > 0) {
        const defaultProfile = getDefaultProfile(profiles);
        if (defaultProfile !== undefined) profileId = defaultProfile.id;
      }

      // Default profile is set
      if (profileId !== undefined) return;

      // Create a new default profile
      createProfile.mutate(
        {
          create: {
            name: i18n.f("default_profile"),
            default: true,
            config: {},
            order: 0,
          },
        },
        {
          // Use the newly create profile
          onSuccess: (data) => {
            profileId = data.id;
          },
        },
      );
    },
  );
</script>

{#if createProfile.isPending || profilesQuery.isLoading || profileQuery.isLoading || profileQuery.isRefetching}
  <!-- Loading states -->
  <SkeletonList style="margin: 1rem;" />
{:else if createProfile.isError}
  <!-- Error creating current profile -->
  <Aside severity="error" style="margin: 1rem;">
    {i18n.f("create_profile_error", {
      values: { error: getErrorMessage(createProfile.error) },
    })}
  </Aside>
{:else if profilesQuery.isError}
  <!-- Error loading profiles list -->
  <Aside severity="error" style="margin: 1rem;">
    {i18n.f("profiles_error", {
      values: { error: getErrorMessage(profilesQuery.error) },
    })}
  </Aside>
{:else if profileQuery.isError}
  <!-- Error loading current profile -->
  <Aside severity="error" style="margin: 1rem;">
    {i18n.f("profile_error", {
      values: { error: getErrorMessage(profileQuery.error) },
    })}
  </Aside>
{:else if (createProfile.isIdle || createProfile.isSuccess) && profilesQuery.isSuccess && profileQuery.isSuccess}
  <!-- Profiles are loaded, current profile is loaded, current profile is created -->
  {@render children?.()}
{/if}
