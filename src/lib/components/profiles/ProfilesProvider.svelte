<script module>
  const PROFILE_STORE_KEY = Symbol("ProfileStore");

  interface ProfileContext {
    profile(): ProfileModel;
    setProfileId: (value: ProfileId) => void;
  }

  export function getProfileContext(): ProfileContext {
    return getContext(PROFILE_STORE_KEY);
  }
</script>

<!-- Provider for profiles, loads the active default profile, creates one if missing -->
<script lang="ts">
  import type { ProfileId, ProfileModel } from "$lib/api/types/profiles";

  import { watch } from "runed";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    createProfileQuery,
    createProfilesQuery,
    createCreateProfileMutation,
  } from "$lib/api/profiles";

  type Props = {
    children?: Snippet | undefined;
  };

  const { children }: Props = $props();

  const profilesQuery = createProfilesQuery();
  const profilesQueryData = $derived($profilesQuery.data);

  const createProfile = createCreateProfileMutation();

  // State for the actively selected profile
  let profileId: ProfileId | undefined = $state(undefined);

  const profileQuery = createProfileQuery(() => profileId ?? null);
  const profile = $derived($profileQuery.data);

  function getDefaultProfile(
    profiles: ProfileModel[],
  ): ProfileModel | undefined {
    return profiles.find((profile) => profile.default);
  }

  setContext(PROFILE_STORE_KEY, {
    profile: () => profile!,
    setProfileId: (value: string) => (profileId = value),
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
      $createProfile.mutate(
        {
          create: {
            name: "Default Profile",
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

{#if $createProfile.isIdle || $createProfile.isSuccess}
  {#if $profilesQuery.isLoading}
    Loading...
  {:else if $profilesQuery.isError}
    Failed to load profiles {getErrorMessage($profilesQuery.error)}
  {:else if $profilesQuery.isSuccess}
    <!-- Query the current folder -->
    {#if $profileQuery.isLoading || $profileQuery.isRefetching}
      Loading profile...
    {:else if $profileQuery.isError}
      Failed to load profile {getErrorMessage($profileQuery.error)}
    {:else if $profileQuery.isSuccess}
      {@render children?.()}
    {/if}
  {/if}
{:else if $createProfile.isPending}
  Creating default profile...
{:else if $createProfile.isError}
  Failed to create default profile {getErrorMessage($createProfile.error)}
{/if}
