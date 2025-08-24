<script lang="ts">
  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import {
    deviceRequestsQuery,
    approveDeviceRequest,
    declineDeviceRequest,
  } from "$lib/api/devices";

  import DeviceRequestDialog from "./DeviceRequestDialog.svelte";

  const i18n = i18nContext.get();

  const requests = deviceRequestsQuery();

  const request = $derived.by(() => {
    const requestsList = requests.data;
    if (!requestsList || requestsList.length < 1) return undefined;
    return requestsList[0];
  });

  function onApprove(requestId: string) {
    const approvePromise = approveDeviceRequest(requestId);

    toast.promise(approvePromise, {
      loading: i18n.f("device_approving"),
      success: i18n.f("device_approved"),
      error: toastErrorMessage(i18n.f("device_approve_error")),
    });
  }

  function onDecline(requestId: string) {
    const declinePromise = declineDeviceRequest(requestId);

    toast.promise(declinePromise, {
      loading: i18n.f("device_declining"),
      success: i18n.f("device_declined"),
      error: toastErrorMessage(i18n.f("device_decline_error")),
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
