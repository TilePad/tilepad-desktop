<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import {
    deviceRequestsQuery,
    approveDeviceRequest,
    declineDeviceRequest,
  } from "$lib/api/devices";

  import DeviceRequestDialog from "./DeviceRequestDialog.svelte";

  const requests = deviceRequestsQuery();

  const request = $derived.by(() => {
    const requestsList = $requests.data;
    if (!requestsList || requestsList.length < 1) return undefined;
    return requestsList[0];
  });

  function onApprove(requestId: string) {
    const approvePromise = approveDeviceRequest(requestId);

    toast.promise(approvePromise, {
      loading: $t("device_approving"),
      success: $t("device_approved"),
      error: toastErrorMessage($t("device_approve_error")),
    });
  }

  function onDecline(requestId: string) {
    const declinePromise = declineDeviceRequest(requestId);

    toast.promise(declinePromise, {
      loading: $t("device_declining"),
      success: $t("device_declined"),
      error: toastErrorMessage($t("device_decline_error")),
    });
  }
</script>

{#if request !== undefined}
  <DeviceRequestDialog
    deviceName={request.device_name}
    address={request.socket_addr}
    clientPublicKey={request.client_public_key}
    onApprove={() => onApprove(request.id)}
    onDecline={() => onDecline(request.id)}
  />
{/if}
