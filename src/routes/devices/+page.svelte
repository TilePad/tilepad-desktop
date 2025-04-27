<script lang="ts">
  import { t } from "svelte-i18n";
  import Aside from "$lib/components/Aside.svelte";
  import { getErrorMessage } from "$lib/api/utils/error";
  import DeviceCard from "$lib/components/devices/DeviceCard.svelte";
  import { devicesQuery, connectedDevicesQuery } from "$lib/api/devices";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import ConnectInfo from "$lib/components/devices/DeviceConnectQR.svelte";

  const devices = devicesQuery();
  const connectedDevices = connectedDevicesQuery();

  const connectedDeviceIds = $derived.by(() => {
    const data = $connectedDevices.data;
    if (data === undefined) return [];
    return data.map((data) => data.device_id);
  });

  function isDeviceConnected(deviceId: string) {
    return connectedDeviceIds.includes(deviceId);
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
      <div class="header">
        <h2>{$t("devices")}</h2>
      </div>

      <div class="devices-wrapper">
        <div class="devices">
          {#each $devices.data as device}
            {@const connected = isDeviceConnected(device.id)}
            <DeviceCard {device} {connected} />
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

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
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
    grid-template-columns: repeat(2, 1fr);
    gap: 1rem;
    padding: 1rem;
  }
</style>
