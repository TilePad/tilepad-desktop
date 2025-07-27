<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
  import SolarSettingsBold from "~icons/solar/settings-bold";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
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
        <div class="wrapper">
          <Button variant="secondary" {...props} class="button">
            <SolarSettingsBold width="1.25rem" height="1.25rem" />
          </Button>
        </div>
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
  .wrapper:global(> .button) {
    border-radius: 0;
  }
</style>
