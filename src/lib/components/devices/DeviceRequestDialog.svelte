<!-- Card for a device that is requesting to connect -->
<script lang="ts">
  import type { DeviceRequest } from "$lib/api/types/devices";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { approveDeviceRequest, declineDeviceRequest } from "$lib/api/devices";

  import Button from "../input/Button.svelte";

  type Props = {
    request: DeviceRequest;
  };

  const { request }: Props = $props();

  function handleApprove() {
    const approvePromise = approveDeviceRequest(request.id);

    toast.promise(approvePromise, {
      loading: "Approving device",
      success: "Approved device",
      error: toastErrorMessage("Failed to approve device"),
    });
  }

  function handleDecline() {
    const declinePromise = declineDeviceRequest(request.id);

    toast.promise(declinePromise, {
      loading: "Declining device",
      success: "Declined device",
      error: toastErrorMessage("Failed to decline device"),
    });
  }
</script>

<div class="device">
  <h2>Device Approval Request</h2>
  <p>Pending request for device approval</p>

  <h3 class="device__name">
    {request.device_name}
  </h3>
  <span class="device__id">Address: {request.socket_addr}</span>

  <div class="actions">
    <Button variant="error" onclick={handleDecline}>Decline</Button>
    <Button onclick={handleApprove}>Approve</Button>
  </div>
</div>

<style>
  .device {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    overflow: hidden;
    width: 100%;
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
    flex: auto;
    width: 100%;
  }

  .actions > :global(*) {
    flex: auto;
  }

  .device__id {
    color: #ccc;
    font-size: 0.8rem;
  }

  .device__name {
    font-size: 1.2rem;
  }
</style>
