<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import { getPluginAssetPath } from "$lib/api/utils/url";

  import { getServerContext } from "../ServerProvider.svelte";
  import { getDraggingContext } from "../tiles/TileDraggingProvider.svelte";

  type Props = {
    action: Action;
  };

  const { action }: Props = $props();
  const { onStartDragging } = getDraggingContext();
  const serverContext = getServerContext();
  let button: HTMLDivElement | undefined = $state();

  function onPointerDown(event: PointerEvent) {
    if (!button) return;
    onStartDragging(event, { type: "action", ...action }, button);
  }
</script>

<div class="action" bind:this={button} onpointerdown={onPointerDown}>
  {#if action.icon !== null}
    <img
      class="icon"
      src={getPluginAssetPath(
        serverContext.serverURL,
        action.plugin_id,
        action.icon,
      )}
      alt="Action Icon"
    />
  {/if}

  <div class="action__text">
    <span class="label">{action.label}</span>
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
