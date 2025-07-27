<!-- Card for a device that is requesting to connect -->
<script lang="ts">
  import type { DeviceRequest } from "$lib/api/types/devices";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { fingerprint } from "$lib/utils/fingerprint";
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
      loading: $t("device_approving"),
      success: $t("device_approved"),
      error: toastErrorMessage($t("device_approve_error")),
    });
  }

  function handleDecline() {
    const declinePromise = declineDeviceRequest(request.id);

    toast.promise(declinePromise, {
      loading: $t("device_declining"),
      success: $t("device_declined"),
      error: toastErrorMessage($t("device_decline_error")),
    });
  }
</script>

<div class="device">
  <h2>{$t("device_approval_request")}</h2>
  <p>{$t("device_approval_request_desc")}</p>

  <h3 class="device__name">
    {request.device_name}
  </h3>
  <span class="device__id">Address: {request.socket_addr}</span>

  {#await fingerprint(new Uint8Array(request.client_public_key)) then print}
    <p class="device__print">Fingerprint:<br />{print}</p>
  {/await}

  <div class="actions">
    <Button variant="error" onclick={handleDecline}>
      {$t("decline")}
    </Button>
    <Button onclick={handleApprove}>
      {$t("approve")}
    </Button>
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

  .device__print {
    word-break: break-all;
    white-space: pre-wrap;
    font-family: monospace;
    padding: 0.5rem;
    max-width: 100%;
    overflow-wrap: break-word;
  }
</style>
