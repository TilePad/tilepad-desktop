<script lang="ts">
  import { createTileQuery } from "$lib/api/tiles";
  import type { TileId } from "$lib/api/types/tiles";
  import { watch } from "runed";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import PropertyInspector from "../property/PropertyInspector.svelte";

  type Props = {
    tileId: TileId;
    onClose: VoidFunction;
  };

  const { tileId, onClose }: Props = $props();

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const tileQuery = createTileQuery(
    () => currentFolder.id,
    () => tileId,
  );

  watch(
    () => $tileQuery.data,
    (tile) => {
      if (tile === null) onClose();
    },
  );
</script>

<div class="editor">
  {#if $tileQuery.isLoading}{:else if $tileQuery.isError}{:else if $tileQuery.isSuccess}
    {@const tile = $tileQuery.data}
    {#if tile === null}
      <div>Unknown tile</div>
    {:else}
      <div class="left">
        <h2>Configure Tile</h2>
        {tile.column}
        {tile.row}
        {JSON.stringify(tile.config)}
        <button onclick={onClose}> Close</button>
      </div>
      <div class="right">
        <PropertyInspector />
      </div>
    {/if}
  {/if}
</div>

<style>
  .editor {
    position: relative;
    height: 250px;
    background-color: #28252c;
    flex: auto;
    width: 100%;
    padding: 2rem;
    border-top: 2px solid #302d36;
    flex-grow: 0;
    flex-shrink: 0;

    display: flex;
    flex-flow: row;
    gap: 0.5rem;
    margin-top: 1rem;
  }

  .left {
    overflow: hidden;
  }

  .right {
    display: flex;
    flex: auto;
    flex-shrink: 0;
    overflow: hidden;
  }
</style>
