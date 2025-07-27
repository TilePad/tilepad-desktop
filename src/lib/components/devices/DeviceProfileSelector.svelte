<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import { createProfilesQuery } from "$lib/api/profiles";
  import DownArrow from "~icons/solar/alt-arrow-down-bold";
  import Button from "$lib/components/input/Button.svelte";

  type Props = {
    profileId: string;
    setProfileId: (profileId: string) => void;
  };

  const { profileId, setProfileId }: Props = $props();

  const profilesQuery = createProfilesQuery();
  const profiles = $derived($profilesQuery.data ?? []);

  const currentProfile = $derived(
    profiles.find((profile) => profile.id === profileId),
  );

  let open = $state(false);
</script>

{#snippet item(profile: ProfileModel)}
  <span>{profile.name} </span>
{/snippet}

<Select.Root
  allowDeselect={false}
  type="single"
  onOpenChange={(value) => (open = value)}
  value={currentProfile?.id}
  onValueChange={(value) => setProfileId(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      <div class="wrapper" data-open={open}>
        <Button class="trigger" variant="secondary" {...props}>
          {#if currentProfile}
            {currentProfile.name}
          {:else}
            {$t("choose_profile")}
          {/if}

          <DownArrow class="trigger__icon" />
        </Button>
      </div>
    {/snippet}
  </Select.Trigger>

  <Select.Portal>
    <Select.Content sideOffset={8} forceMount>
      {#snippet child({ props, open, wrapperProps })}
        <div {...wrapperProps} class="content-wrapper">
          {#if open}
            <div
              {...props}
              class="content"
              transition:slide={{ duration: 100 }}
            >
              {#each profiles as value (value.id)}
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

  .wrapper {
    width: 100%;
  }

  .wrapper:global(> .trigger) {
    width: 100%;
    justify-content: space-between;
  }

  .wrapper:global(> .trigger > .trigger__icon) {
    transition: all var(--tp-transition-fast);
    transform-origin: center;
  }

  .wrapper[data-open="true"]:global(> .trigger > .trigger__icon) {
    transform: rotate(-180deg);
  }
</style>
