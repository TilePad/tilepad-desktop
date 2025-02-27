<script lang="ts">
  import {
    connectedDevicesQuery,
    deviceRequestsQuery,
    devicesQuery,
  } from "$lib/api/devices";
  import DeviceCard from "$lib/components/devices/DeviceCard.svelte";
  import DeviceRequestCard from "$lib/components/devices/DeviceRequestCard.svelte";

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

<a href="/">Home</a>

<h2>Requests</h2>

{#if $requests.isLoading}
  Loading requests...
{:else if $requests.isError}
  Failed to load device requests {$requests.error}
{:else if $requests.isSuccess}
  <div>
    {#each $requests.data as request}
      <DeviceRequestCard {request} />
    {/each}
  </div>
{/if}

<h2>Devices</h2>

{#if $devices.isLoading}
  Loading devices...
{:else if $devices.isError}
  Failed to load devices {$requests.error}
{:else if $devices.isSuccess}
  <div>
    {#each $devices.data as device}
      {@const connected = isDeviceConnected(device.id)}
      <DeviceCard {device} {connected} />
    {/each}
  </div>
{/if}
