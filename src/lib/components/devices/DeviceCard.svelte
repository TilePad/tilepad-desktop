<!-- Card for a known device -->
<script lang="ts">
  import { revokeDevice } from "$lib/api/devices";
  import type { DeviceModel } from "$lib/api/types/devices";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { toast } from "svelte-sonner";
  import SolarTranslationBoldDuotone from "~icons/solar/translation-bold-duotone";
  import SolarTrashBin2BoldDuotone from "~icons/solar/trash-bin-2-bold-duotone";
  import Button from "../input/Button.svelte";
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
  <span class="device__id">{device.id}</span>

  <h2 class="device__name">
    {device.name}

    <span class="state" data-connected={connected}>
      {#if connected}
        <SolarTranslationBoldDuotone />
        Connected
      {:else}
        <SolarTranslationBoldDuotone />
        Not Connected
      {/if}
    </span>
  </h2>

  <Button variant="error" onclick={handleRevoke}
    ><SolarTrashBin2BoldDuotone /> Revoke</Button
  >
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

  .device__id {
    color: #ccc;
    font-size: 0.8rem;
  }

  .device__name {
    font-size: 1.2rem;
  }

  .state {
    padding: 0.5rem;
    display: inline-flex;
    gap: 0.5rem;
    font-size: 0.8rem;
    vertical-align: middle;
  }

  .state[data-connected="false"] {
    color: #da7070;
  }

  .state[data-connected="true"] {
    color: #81e668;
  }
</style>
