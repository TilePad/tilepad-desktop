<script lang="ts">
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import Aside from "$lib/components/Aside.svelte";
  import { createProfilesQuery } from "$lib/api/profiles";
  import DeviceCard from "$lib/components/devices/DeviceCard.svelte";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import ConnectInfo from "$lib/components/devices/DeviceConnectQR.svelte";
  import FoldersLoader from "$lib/components/folders/FoldersLoader.svelte";
  import { getErrorMessage, toastErrorMessage } from "$lib/api/utils/error";
  import {
    devicesQuery,
    connectedDevicesQuery,
    createSetDeviceFolderMutation,
    createSetDeviceProfileMutation,
    createRevokeDeviceFolderMutation,
  } from "$lib/api/devices";

  const devices = devicesQuery();
  const connectedDevices = connectedDevicesQuery();

  const profilesQuery = createProfilesQuery();
  const profiles = $derived($profilesQuery.data ?? []);

  const setDeviceProfileMutation = createSetDeviceProfileMutation();
  const setDeviceFolderMutation = createSetDeviceFolderMutation();
  const revokeDeviceMutation = createRevokeDeviceFolderMutation();

  const connectedDeviceIds = $derived.by(() => {
    const data = $connectedDevices.data;
    if (data === undefined) return [];
    return data.map((data) => data.device_id);
  });

  function isDeviceConnected(deviceId: string) {
    return connectedDeviceIds.includes(deviceId);
  }

  function onRevoke(deviceId: string) {
    const revokePromise = $revokeDeviceMutation.mutateAsync({
      deviceId,
    });

    toast.promise(revokePromise, {
      loading: $t("device_revoking"),
      success: $t("device_revoked"),
      error: toastErrorMessage($t("device_revoke_error")),
    });
  }

  function onChangeProfile(deviceId: string, profileId: ProfileId) {
    $setDeviceProfileMutation.mutate({ deviceId, profileId });
  }

  function onChangeFolder(deviceId: string, folderId: FolderId) {
    $setDeviceFolderMutation.mutate({ deviceId, folderId });
  }
</script>

<div class="layout">
  <div class="layout__devices">
    {#if $devices.isLoading}
      <SkeletonList style="margin: 1rem" />
    {:else if $devices.isError}
      <Aside severity="error" style="margin: 1rem">
        {$t("devices_error", {
          values: { error: getErrorMessage($devices.error) },
        })}
      </Aside>
    {:else if $devices.isSuccess}
      <div class="devices-wrapper">
        <div class="devices">
          {#each $devices.data as device (device.id)}
            {@const connected = isDeviceConnected(device.id)}
            <FoldersLoader profileId={device.profile_id}>
              {#snippet content({ folders })}
                <DeviceCard
                  id={device.id}
                  name={device.name}
                  publicKey={device.public_key}
                  profileId={device.profile_id}
                  folderId={device.folder_id}
                  {connected}
                  {profiles}
                  {folders}
                  onRevoke={() => onRevoke(device.id)}
                  onChangeProfile={(profileId) =>
                    onChangeProfile(device.id, profileId)}
                  onChangeFolder={(folderId) =>
                    onChangeFolder(device.id, folderId)}
                />
              {/snippet}
            </FoldersLoader>
          {:else}
            {$t("devices_none")}
          {/each}
        </div>
      </div>
    {/if}
  </div>

  <div class="layout__connect">
    <ConnectInfo />
  </div>
</div>

<style>
  .layout {
    display: flex;
    height: 100%;
  }

  .layout__devices {
    flex: auto;

    display: flex;
    flex-flow: column;
  }

  .layout__connect {
    max-width: 16rem;
    height: 100%;
    background-color: #232029;
    border-left: 2px solid #393444;
  }

  .devices-wrapper {
    flex: auto;
    overflow: auto;
  }

  .devices {
    width: 100%;
    display: grid;
    grid-template-rows: 1f;
    gap: var(--tp-space-4);
    padding: var(--tp-space-4);
  }
</style>
