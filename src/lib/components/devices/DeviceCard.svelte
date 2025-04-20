<!-- Card for a known device -->
<script lang="ts">
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { DeviceModel } from "$lib/api/types/devices";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarTrashBin2BoldDuotone from "~icons/solar/trash-bin-2-bold-duotone";
  import SolarTranslationBoldDuotone from "~icons/solar/translation-bold-duotone";
  import {
    revokeDevice,
    setDeviceFolder,
    setDeviceProfile,
  } from "$lib/api/devices";

  import Button from "../input/Button.svelte";
  import DeviceFolderSelector from "./DeviceFolderSelector.svelte";
  import DeviceProfileSelector from "./DeviceProfileSelector.svelte";
  type Props = {
    device: DeviceModel;
    connected: boolean;
  };

  const { device, connected }: Props = $props();

  function handleRevoke() {
    const revokePromise = revokeDevice(device.id);

    toast.promise(revokePromise, {
      loading: "Revoking device",
      success: "Revoked device",
      error: toastErrorMessage("Failed to revoke device"),
    });
  }

  function onChangeProfile(profileId: ProfileId) {
    setDeviceProfile(device.id, profileId);
  }

  function onChangeFolder(folderId: FolderId) {
    setDeviceFolder(device.id, folderId);
  }
</script>

<div class="device">
  <span class="device__id">{device.id}</span>

  <h2 class="device__name">
    {device.name}

    <span class="state" data-connected={connected}>
      {#if connected}
        <SolarTranslationBoldDuotone />
        Connected
      {:else}
        <SolarTranslationBoldDuotone />
        Not Connected
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
    <SolarTrashBin2BoldDuotone /> Revoke
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
