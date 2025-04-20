<script lang="ts">
  import { Dialog } from "bits-ui";
  import { fade, slide } from "svelte/transition";
  import { deviceRequestsQuery } from "$lib/api/devices";

  import DeviceRequestDialog from "./DeviceRequestDialog.svelte";

  const requests = deviceRequestsQuery();

  const request = $derived.by(() => {
    const requestsList = $requests.data;
    if (!requestsList || requestsList.length < 1) return undefined;
    return requestsList[0];
  });
</script>

<Dialog.Root open={request !== undefined}>
  <Dialog.Portal>
    <Dialog.Overlay forceMount>
      {#snippet child({ props })}
        {#if request !== undefined}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content
      trapFocus={false}
      forceMount
      interactOutsideBehavior="ignore"
    >
      {#snippet child({ props })}
        {#if request !== undefined}
          <div
            {...props}
            class="content"
            transition:slide={{ axis: "y", duration: 150 }}
          >
            <div class="content-inner">
              <DeviceRequestDialog {request} />
            </div>
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 19;
  }

  .content {
    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    width: 25rem;

    overflow: hidden;

    background-color: #18161b;
    border: 1px solid #222;
    border-radius: 0.25rem;

    z-index: 20;
  }

  .content-inner {
    display: flex;
    flex-flow: column;

    overflow: hidden;

    width: 25rem;
    height: 100%;
  }
</style>
