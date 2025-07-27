<script lang="ts" module>
  export interface ActionItemData {
    pluginId: PluginId;
    actionId: ActionId;

    label: string;
    icon?: string;
  }
</script>

<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";
  import type { ActionId } from "$lib/api/types/actions";

  import { getDraggingContext } from "../tiles/TileDraggingProvider.svelte";

  type Props = ActionItemData;

  const { pluginId, actionId, label, icon }: Props = $props();
  const { onStartDragging } = getDraggingContext();

  let button: HTMLDivElement | undefined = $state();

  function onPointerDown(event: PointerEvent) {
    if (!button) return;
    onStartDragging(
      event,
      {
        type: "action",
        actionId,
        pluginId,
      },
      button,
    );
  }
</script>

<div class="action" bind:this={button} onpointerdown={onPointerDown}>
  {#if icon}
    <img class="icon" src={icon} alt="Action Icon" />
  {/if}

  <div class="action__text">
    <span class="label">{label}</span>
  </div>
</div>

<style>
  .action {
    width: 15rem;
    height: 40px;

    padding: 0.5rem;
    display: flex;
    flex-flow: row;
    align-items: center;

    gap: 0.5rem;
    padding-left: 1rem;
    background-color: #28262c;
    cursor: grab;
  }

  .icon {
    width: 1.5rem;
    max-height: 1.5rem;
  }

  .action__text {
    display: flex;
    flex-flow: column;
  }

  .label {
    font-weight: bold;
    font-size: 0.9rem;
  }
</style>
