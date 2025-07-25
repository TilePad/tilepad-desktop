<!-- Card for a known device -->
<script lang="ts">
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { DeviceModel } from "$lib/api/types/devices";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
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

<div class="device">
  <span class="device__id">{device.id}</span>

  <h2 class="device__name">
    {device.name}

    <span class="state" data-connected={connected}>
      {#if connected}
        <SolarTranslationBoldDuotone />
        {$t("connected")}
      {:else}
        <SolarTranslationBoldDuotone />
        {$t("not_connected")}
      {/if}
    </span>
  </h2>

  <DeviceProfileSelector
    profileId={device.profile_id}
    setProfileId={onChangeProfile}
  />

  <DeviceFolderSelector
    profileId={device.profile_id}
    folderId={device.folder_id}
    setFolderId={onChangeFolder}
  />

  <Button variant="error" onclick={handleRevoke} style="margin-top: 0.5rem">
    <SolarTrashBin2BoldDuotone />
    {$t("revoke")}
  </Button>
</div>

<style>
  .device {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
  }

  .device__id {
    color: #ccc;
    font-size: 0.8rem;
  }

  .device__name {
    font-size: 1.2rem;
  }

  .state {
    padding: 0.5rem;
    display: inline-flex;
    gap: 0.5rem;
    font-size: 0.8rem;
    vertical-align: middle;
  }

  .state[data-connected="false"] {
    color: #da7070;
  }

  .state[data-connected="true"] {
    color: #81e668;
  }
</style>
