<!-- Card for a device that is requesting to connect -->
<script lang="ts">
  import { approveDeviceRequest, declineDeviceRequest } from "$lib/api/devices";
  import type { DeviceRequest } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";
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
  <h2 class="device__name">
    {request.device_name}
  </h2>
  <span class="device__id">{request.socket_addr}</span>

  <div class="actions">
    <Button onclick={handleApprove}>Approve</Button>
    <Button variant="error" onclick={handleDecline}>Decline</Button>
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
    background-color: #2f2c36;
  }

  .actions {
    display: flex;
    gap: 1rem;
  }

  .device__id {
    color: #ccc;
    font-size: 0.8rem;
  }

  .device__name {
    font-size: 1.2rem;
  }
</style>
