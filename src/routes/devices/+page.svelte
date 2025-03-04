<script lang="ts">
  import DeviceCard from "$lib/components/devices/DeviceCard.svelte";
  import ConnectInfo from "$lib/components/devices/DeviceConnectQR.svelte";
  import DeviceRequestCard from "$lib/components/devices/DeviceRequestCard.svelte";
  import {
    devicesQuery,
    deviceRequestsQuery,
    connectedDevicesQuery,
  } from "$lib/api/devices";

  const devices = devicesQuery();
  const connectedDevices = connectedDevicesQuery();
  const requests = deviceRequestsQuery();

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
      Loading devices...
    {:else if $devices.isError}
      Failed to load devices {$requests.error}
    {:else if $devices.isSuccess && $devices.data.length > 0}
      <div class="section">
        <h2>Devices</h2>
        <div class="devices">
          {#each $devices.data as device}
            {@const connected = isDeviceConnected(device.id)}
            <DeviceCard {device} {connected} />
          {/each}
        </div>
      </div>
    {/if}
    {#if $requests.isLoading}
      Loading requests...
    {:else if $requests.isError}
      Failed to load device requests {$requests.error}
    {:else if $requests.isSuccess && $requests.data.length > 0}
      <div class="section">
        <h2>Requests</h2>
        <div class="devices">
          {#each $requests.data as request}
            <DeviceRequestCard {request} />
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
    gap: 0.5rem;

    padding: 1rem;
  }

  .section {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .layout__connect {
    max-width: 16rem;
    height: 100%;
    background-color: #232029;
    border-left: 2px solid #393444;
  }

  .devices {
    width: 100%;
    display: grid;
    grid-template-columns: repeat(2, 1fr);
  }
</style>
