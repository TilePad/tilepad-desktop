<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
  import SolarSettingsBold from "~icons/solar/settings-bold";

  import Tooltip from "../Tooltip.svelte";
  import EditProfileDialog from "./EditProfileDialog.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";
  import DeleteProfileDialog from "./DeleteProfileDialog.svelte";

  type Props = {
    profile: ProfileModel;
  };

  const { profile }: Props = $props();
</script>

<Tooltip title={$t("profile_settings")}>
  {#snippet trigger({ props: triggerProps })}
    <PopoverButton {triggerProps}>
      {#snippet button({ props })}
        <button {...props} class="button">
          <SolarSettingsBold width="1.25rem" height="1.25rem" />
        </button>
      {/snippet}
      {#snippet content()}
        <EditProfileDialog {profile} />
        {#if !profile.default}
          <DeleteProfileDialog {profile} />
        {/if}
      {/snippet}
    </PopoverButton>
  {/snippet}
</Tooltip>

<style>
  .button {
    padding: 0rem 0.5rem;
    border: none;
    background-color: #141316;
    color: #fff;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;

    justify-content: space-between;
  }
</style>
