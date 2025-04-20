<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import { createProfilesQuery } from "$lib/api/profiles";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";
  import SolarUsersGroupRoundedBoldDuotone from "~icons/solar/users-group-rounded-bold-duotone";

  import { getProfileContext } from "./ProfilesProvider.svelte";
  import CreateProfileDialog from "./CreateProfileDialog.svelte";
  import ProfileSelectorSettings from "./ProfileSelectorSettings.svelte";
  const profilesQuery = createProfilesQuery();
  const profiles = $derived($profilesQuery.data ?? []);

  const { profile, setProfileId } = getProfileContext();
  const currentProfile = $derived.by(profile);

  let open = $state(false);
</script>

{#snippet item(profile: ProfileModel)}
  <span>{profile.name} </span>
{/snippet}

<Select.Root
  allowDeselect={false}
  type="single"
  onOpenChange={(value) => (open = value)}
  value={currentProfile.id}
  onValueChange={(value) => setProfileId(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      <div class="trigger-wrapper">
        <button class="trigger" {...props}>
          <SolarUsersGroupRoundedBoldDuotone />

          {currentProfile.name}
          {#if open}
            <SolarAltArrowUpBold />
          {:else}
            <SolarAltArrowDownBold />
          {/if}
        </button>

        <CreateProfileDialog order={profiles.length} />
        <ProfileSelectorSettings profile={currentProfile} />
      </div>
    {/snippet}
  </Select.Trigger>

  <Select.Portal>
    <Select.Content sideOffset={8} forceMount align="start">
      {#snippet child({ props, open, wrapperProps })}
        <div {...wrapperProps} class="content-wrapper">
          {#if open}
            <div
              {...props}
              class="content"
              transition:slide={{ duration: 100 }}
            >
              {#each profiles as value}
                <Select.Item value={value.id} label={value.name}>
                  {#snippet child({ props, selected, highlighted })}
                    <div
                      {...props}
                      class="item"
                      class:item--selected={selected}
                      class:item--highlighted={highlighted}
                    >
                      {@render item(value)}
                    </div>
                  {/snippet}
                </Select.Item>
              {/each}
            </div>
          {/if}
        </div>
      {/snippet}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  .content {
    border-radius: 0.5rem;
    background-color: #2a2631;
    padding: 0.25rem;
    max-height: 40vh;
    min-width: 20rem;
    overflow: auto;
    box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.1);
  }

  .form-input {
    display: inline-flex;
    flex-flow: column;
  }

  .item {
    cursor: pointer;
    display: flex;
    border-radius: 0.25rem;
    padding: 0.5rem;
    max-width: 500px;
  }

  .item:hover {
    background-color: #706580;
  }

  .item--selected {
    background-color: #675d75;
  }

  .item--highlighted {
    outline: 1px solid white;
  }

  .trigger {
    padding: 0.5rem;
    border: none;
    background-color: #1f1d22;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    justify-content: space-between;
  }

  .trigger-wrapper {
    display: flex;
  }
</style>
