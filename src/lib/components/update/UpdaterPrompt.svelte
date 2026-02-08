<script lang="ts" module>
  export type UpdateState =
    | "ready"
    | "downloading"
    | "installable"
    | "installing";
</script>

<script lang="ts">
  import type { Update } from "@tauri-apps/plugin-updater";

  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import Button from "$lib/components/input/Button.svelte";
  import SolarCloseCircleBold from "~icons/solar/close-circle-bold";

  interface Props {
    update: Update;
    state: UpdateState;

    onUpdate: VoidFunction;
    onClose: VoidFunction;
  }

  const { update, state, onUpdate, onClose }: Props = $props();

  const i18n = i18nContext.get();
</script>

<div class="wrapper">
  <div class="container">
    {#if state === "ready"}
      <div class="close-wrapper">
        <Button transparent onclick={onClose} size="icon">
          <SolarCloseCircleBold width={18} height={18} />
        </Button>
      </div>
    {/if}

    <p class="title">
      {#if state === "installing"}
        {i18n.f("installing")}
      {:else if state === "installable"}
        {i18n.f("install_update", {
          values: { version: update.version },
        })}
      {:else if state === "downloading"}
        {i18n.f("update_downloading", {
          values: { version: update.version },
        })}
      {:else if state === "ready"}
        {i18n.f("update_available", {
          values: { version: update.version },
        })}
      {/if}
    </p>

    <Button
      loading={["downloading", "installing"].includes(state)}
      onclick={onUpdate}
    >
      {#if ["installable", "installing"].includes(state)}
        {i18n.f("install")}
      {:else if ["ready", "downloading"].includes(state)}
        {i18n.f("update")}
      {/if}
    </Button>
  </div>
</div>

<style>
  .wrapper {
    position: fixed;
    bottom: 1rem;
    left: 50%;
    transform: translateX(-50%);
  }

  .container {
    display: flex;
    flex-flow: row;
    align-items: center;
    gap: var(--tp-space-4);

    padding: var(--tp-space-3);
    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-tertiary);
    border: 1px solid var(--tp-border-hover);
    box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.1);
  }

  .close-wrapper {
    position: absolute;
    left: 0;
    top: 0;
    transform: translate(-40%, -40%);
  }
</style>
