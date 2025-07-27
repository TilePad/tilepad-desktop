<!-- Card for a known device -->
<script lang="ts">
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { DeviceModel } from "$lib/api/types/devices";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { fingerprint } from "$lib/utils/fingerprint";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarTrashBin2BoldDuotone from "~icons/solar/trash-bin-2-bold-duotone";
  import SolarTranslationBoldDuotone from "~icons/solar/translation-bold-duotone";
  import {
    createSetDeviceFolderMutation,
    createSetDeviceProfileMutation,
    createRevokeDeviceFolderMutation,
  } from "$lib/api/devices";

  import Button from "../input/Button.svelte";
  import DeviceFolderSelector from "./DeviceFolderSelector.svelte";
  import DeviceProfileSelector from "./DeviceProfileSelector.svelte";
  type Props = {
    device: DeviceModel;
    connected: boolean;
  };

  const { device, connected }: Props = $props();

  const setDeviceProfileMutation = createSetDeviceProfileMutation();
  const setDeviceFolderMutation = createSetDeviceFolderMutation();
  const revokeDeviceMutation = createRevokeDeviceFolderMutation();

  function handleRevoke() {
    const revokePromise = $revokeDeviceMutation.mutateAsync({
      deviceId: device.id,
    });

    toast.promise(revokePromise, {
      loading: $t("device_revoking"),
      success: $t("device_revoked"),
      error: toastErrorMessage($t("device_revoke_error")),
    });
  }

  function onChangeProfile(profileId: ProfileId) {
    $setDeviceProfileMutation.mutate({ deviceId: device.id, profileId });
  }

  function onChangeFolder(folderId: FolderId) {
    $setDeviceFolderMutation.mutate({ deviceId: device.id, folderId });
  }
</script>

<div class="card">
  <div class="head">
    <span class="identifier">{device.id}</span>

    <p class="state" data-connected={connected}>
      {#if connected}
        <SolarTranslationBoldDuotone />
        {$t("connected")}
      {:else}
        <SolarTranslationBoldDuotone />
        {$t("not_connected")}
      {/if}
    </p>
  </div>

  <h2 class="name">
    {device.name}
  </h2>

  {#await fingerprint(new Uint8Array(device.public_key)) then print}
    <p class="fingerprint">{print}</p>
  {/await}

  <div class="actions">
    <DeviceProfileSelector
      profileId={device.profile_id}
      setProfileId={onChangeProfile}
    />

    <DeviceFolderSelector
      profileId={device.profile_id}
      folderId={device.folder_id}
      setFolderId={onChangeFolder}
    />

    <Button variant="error" onclick={handleRevoke}>
      <SolarTrashBin2BoldDuotone />
      {$t("revoke")}
    </Button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-flow: column;
    gap: var(--tp-space-3);
    align-items: flex-start;

    padding: var(--tp-space-4);
    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-secondary);
    border: 1px solid var(--tp-border-secondary);
  }

  .head {
    display: flex;
    flex-flow: row;
    align-items: center;
    justify-content: space-between;
    gap: var(--tp-space-2);

    width: 100%;
  }

  .identifier {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
  }

  .name {
    font-size: var(--tp-text-lg);
    line-height: var(--tp-leading-tight);
  }

  .state {
    display: inline-flex;
    align-items: center;
    gap: var(--tp-space-2);
    font-size: var(--tp-text-sm);
    vertical-align: middle;
  }

  .state[data-connected="false"] {
    color: var(--tp-error-500);
  }

  .state[data-connected="true"] {
    color: var(--tp-success-500);
  }

  .actions {
    display: flex;
    align-items: center;
    width: 100%;
    gap: var(--tp-space-2);
    justify-content: flex-start;
  }

  .actions:global(> .wrapper) {
    max-width: 200px;
  }

  .fingerprint {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
  }
</style>
