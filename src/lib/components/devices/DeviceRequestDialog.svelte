<script lang="ts">
  import { tick } from "svelte";
  import { Dialog } from "bits-ui";
  import { fade, slide } from "svelte/transition";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { fingerprint } from "$lib/utils/fingerprint";
  import SolarKeyBoldDuotone from "~icons/solar/key-bold-duotone";
  import SolarRoutingBoldDuotone from "~icons/solar/routing-bold-duotone";
  import SolarMonitorBoldDuotone from "~icons/solar/monitor-bold-duotone";
  import SolarSmartphoneBoldDuotone from "~icons/solar/smartphone-bold-duotone";

  import Button from "../input/Button.svelte";

  type Props = {
    deviceName: string;
    address: string;
    clientPublicKey: number[];

    onApprove: VoidFunction;
    onDecline: VoidFunction;
  };

  const { deviceName, address, clientPublicKey, onApprove, onDecline }: Props =
    $props();

  const i18n = i18nContext.get();

  const fingerprintPromise = $derived(
    fingerprint(new Uint8Array(clientPublicKey)),
  );

  // Delayed open state till next tick to play dialog animations
  let open = $state(false);

  $effect(() => {
    tick().then(() => {
      open = true;
    });
  });
</script>

<Dialog.Root {open}>
  <Dialog.Portal>
    <Dialog.Overlay forceMount>
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content forceMount interactOutsideBehavior="ignore">
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="content"
            transition:slide={{ axis: "y", duration: 250 }}
          >
            <div class="graphic">
              <SolarSmartphoneBoldDuotone width="3rem" height="3rem" />
              <SolarRoutingBoldDuotone
                class="graphic__line"
                width="3rem"
                height="3rem"
              />
              <SolarMonitorBoldDuotone width="3rem" height="3rem" />
            </div>

            <h2 class="title">{i18n.f("device_approval_request")}</h2>
            <p class="description">{i18n.f("device_approval_request_desc")}</p>

            <h3 class="name">
              {deviceName}
            </h3>

            <span class="address">{i18n.f("ip_address")}: {address}</span>

            {#await fingerprintPromise then print}
              <div class="fingerprint">
                <p class="fingerprint__label">
                  <SolarKeyBoldDuotone
                    style="display: inline; vertical-align: middle;"
                  />
                  {i18n.f("fingerprint")}:
                </p>
                <p class="fingerprint__value">
                  {print}
                </p>
              </div>
            {/await}

            <div class="actions">
              <Button variant="error" onclick={onDecline}>
                {i18n.f("decline")}
              </Button>
              <Button onclick={onApprove}>
                {i18n.f("approve")}
              </Button>
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
    backdrop-filter: blur(4px);
  }

  .graphic {
    display: flex;
    justify-content: center;
    width: 100%;
    margin: var(--tp-space-4) 0;
  }

  .graphic:global(> .graphic__line) {
    animation: blink forwards infinite 2000ms;
  }

  @keyframes blink {
    0%,
    100% {
      opacity: 0.5;
      color: #fff;
    }
    50% {
      opacity: 1;
    }
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

    display: flex;
    flex-flow: column;

    overflow: hidden;

    width: 25rem;
    height: 28rem;
    padding: var(--tp-space-6);
    align-items: flex-start;
    gap: var(--tp-space-2);
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    flex: auto;
    width: 100%;
  }

  .actions > :global(*) {
    flex: auto;
  }

  .description {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-md);
  }

  .address {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-sm);
  }

  .name {
    color: var(--tp-text-primary);
    font-size: var(--tp-text-lg);
  }

  .fingerprint__label {
    color: var(--tp-text-primary);
    display: flex;
    gap: var(--tp-space-2);
    align-items: center;
  }

  .fingerprint__value {
    color: var(--tp-text-secondary);
    word-break: break-all;
    white-space: pre-wrap;
    font-family: monospace;
    padding: 0.5rem;
    max-width: 100%;
    overflow-wrap: break-word;
  }
</style>
