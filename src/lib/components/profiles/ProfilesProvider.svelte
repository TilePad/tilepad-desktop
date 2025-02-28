<script module>
  const PROFILE_STORE_KEY = Symbol("ProfileStore");

  interface ProfileContext {
    profile: ProfileModel;
    setProfile: (value: ProfileModel) => void;
  }

  export function getProfileContext(): ProfileContext {
    return getContext(PROFILE_STORE_KEY);
  }
</script>

<!-- Provider for profiles, loads the active default profile, creates one if missing -->
<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { watch } from "runed";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getContext, setContext, type Snippet } from "svelte";
  import {
    createProfilesQuery,
    createCreateProfileMutation,
  } from "$lib/api/profiles";

  type Props = {
    children?: Snippet | undefined;
  };

  const { children }: Props = $props();

  const profilesQuery = createProfilesQuery();
  const profilesQueryData = $derived($profilesQuery.data);

  const createDefaultProfile = createCreateProfileMutation();

  // State for the actively selected profile
  let profile: ProfileModel | undefined = $state(undefined);

  function getDefaultProfile(
    profiles: ProfileModel[],
  ): ProfileModel | undefined {
    return profiles.find((profile) => profile.default);
  }

  setContext(PROFILE_STORE_KEY, {
    profile: () => profile,
    setProfile: (value: ProfileModel) => (profile = value),
  });

  watch(
    () => profilesQueryData,
    (profiles) => {
      // Profiles are loaded yet ignore
      if (profiles === undefined) return;

      // Try and set the profile to the default
      if (profiles.length > 0) {
        profile = getDefaultProfile(profiles);
      }

      // Default profile is set
      if (profile !== undefined) return;

      // Create a new default profile
      $createDefaultProfile.mutate(
        {
          create: {
            name: "Default",
            default: true,
            config: {},
            order: 0,
          },
        },
        {
          // Use the newly create profile
          onSuccess: (data) => {
            profile = data;
          },
        },
      );
    },
  );
</script>

{#if $createDefaultProfile.isIdle || $createDefaultProfile.isSuccess}
  {#if $profilesQuery.isLoading}
    Loading...
  {:else if $profilesQuery.isError}
    Failed to load profiles {getErrorMessage($profilesQuery.error)}
  {:else if $profilesQuery.isSuccess}
    {#if profile}
      {@render children?.()}
    {/if}
  {/if}
{:else if $createDefaultProfile.isPending}
  Creating default profile...
{:else if $createDefaultProfile.isError}
  Failed to create default profile {getErrorMessage(
    $createDefaultProfile.error,
  )}
{/if}
