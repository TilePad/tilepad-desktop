<script lang="ts">
  import {
    approveDeviceRequest,
    declineDeviceRequest,
    deviceRequestsQuery,
  } from "$lib/api/devices";
  import type { DeviceRequestId } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";

  const requestsQuery = deviceRequestsQuery();

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
</script>

{#if $requestsQuery.isLoading}
  Loading requests...
{:else if $requestsQuery.isError}
  Failed to load device requests {$requestsQuery.error}
{:else if $requestsQuery.isSuccess}
  <div>
    {#each $requestsQuery.data as request}
      <div>
        <b>Name</b>: {request.device_name}
        <b>Address</b>: {request.socket_addr}

        <button onclick={() => handleApprove(request.id)}>Approve</button>
        <button onclick={() => handleDecline(request.id)}>Decline</button>
      </div>
    {/each}
  </div>
{/if}
