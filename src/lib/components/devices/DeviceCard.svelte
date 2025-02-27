<!-- Card for a known device -->
<script lang="ts">
  import { revokeDevice } from "$lib/api/devices";
  import type { DeviceModel } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";

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
</script>

<div class="device">
  <b>Name</b>: {device.name}
  <b>Last connected at</b>: {device.last_connected_at}
  <b>Connected</b>: {connected}

  <button onclick={handleRevoke}>Revoke</button>
</div>

<style>
</style>
