<script lang="ts">
  import {
    approveDeviceRequest,
    connectedDevicesQuery,
    declineDeviceRequest,
    deviceRequestsQuery,
    devicesQuery,
    revokeDevice,
  } from "$lib/api/devices";
  import type { DeviceId, DeviceRequestId } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";

  const devices = devicesQuery();
  const connectedDevices = connectedDevicesQuery();
  const requests = deviceRequestsQuery();

  const connectedDeviceIds = $derived.by(() => {
    const data = $connectedDevices.data;
    if (data === undefined) return [];
    return data.map((data) => data.device_id);
  });

  function handleApprove(requestId: DeviceRequestId) {
    const approvePromise = approveDeviceRequest(requestId);

    toast.promise(approvePromise, {
      loading: "Approving device",
      success: "Approved device",
      error: toastErrorMessage("Failed to approve device"),
    });
  }

  function handleDecline(requestId: DeviceRequestId) {
    const declinePromise = declineDeviceRequest(requestId);

    toast.promise(declinePromise, {
      loading: "Declining device",
      success: "Declined device",
      error: toastErrorMessage("Failed to decline device"),
    });
  }

  function handleRevoke(deviceId: DeviceId) {
    const revokePromise = revokeDevice(deviceId);

    toast.promise(revokePromise, {
      loading: "Revoking device",
      success: "Revoked device",
      error: toastErrorMessage("Failed to revoke device"),
    });
  }

  function isDeviceConnected(deviceId: string) {
    return connectedDeviceIds.includes(deviceId);
  }
</script>

<h2>Requests</h2>

{#if $requests.isLoading}
  Loading requests...
{:else if $requests.isError}
  Failed to load device requests {$requests.error}
{:else if $requests.isSuccess}
  <div>
    {#each $requests.data as request}
      <div>
        <b>Name</b>: {request.device_name}
        <b>Address</b>: {request.socket_addr}

        <button onclick={() => handleApprove(request.id)}>Approve</button>
        <button onclick={() => handleDecline(request.id)}>Decline</button>
      </div>
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
      <div>
        <b>Name</b>: {device.name}
        <b>Last connected at</b>: {device.last_connected_at}
        <b>Connected</b>: {isDeviceConnected(device.id)}

        <button onclick={() => handleRevoke(device.id)}>Revoke</button>
      </div>
    {/each}
  </div>
{/if}
