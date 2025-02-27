<!-- Card for a device that is requesting to connect -->
<script lang="ts">
  import { approveDeviceRequest, declineDeviceRequest } from "$lib/api/devices";
  import type { DeviceRequest } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";

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
  <b>Name</b>: {request.device_name}
  <b>Address</b>: {request.socket_addr}

  <button onclick={handleApprove}>Approve</button>
  <button onclick={handleDecline}>Decline</button>
</div>

<style></style>
