<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import SolarSettingsBold from "~icons/solar/settings-bold";

  import PopoverButton from "../popover/PopoverButton.svelte";
  import DeleteProfileDialog from "./DeleteProfileDialog.svelte";

  type Props = {
    profile: ProfileModel;
  };

  const { profile }: Props = $props();
</script>

<PopoverButton>
  {#snippet button({ props })}
    <button {...props} class="button">
      <SolarSettingsBold width="1.25rem" height="1.25rem" />
    </button>
  {/snippet}
  {#snippet content()}
    <DeleteProfileDialog {profile} />
  {/snippet}
</PopoverButton>

<style>
  .button {
    padding: 0rem 0.5rem;
    border: none;
    background-color: #141316;
    color: #fff;
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;

    justify-content: space-between;
  }
</style>
