<script lang="ts">
  import { createTileQuery } from "$lib/api/tiles";
  import type { TileId } from "$lib/api/types/tiles";
  import { watch } from "runed";
  import { getFolderContext } from "../folders/FolderProvider.svelte";
  import PropertyInspector from "../property/PropertyInspector.svelte";
  import { createActionQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";

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

  const tile = $derived($tileQuery.data);
  const tileConfig = $derived(tile?.config);

  const actionQuery = createActionQuery(
    () => tileConfig?.plugin_id ?? null,
    () => tileConfig?.action_id ?? null,
  );

  const action = $derived($actionQuery.data);

  watch(
    () => $tileQuery.data,
    (tile) => {
      if (tile === null) onClose();
    },
  );
</script>

<div class="editor">
  {#if $tileQuery.isSuccess && $actionQuery.isSuccess && tile && action}
    <div class="left">
      <h2>Configure Tile</h2>
      {tile.column}
      {tile.row}
      {JSON.stringify(tile.config)}
      {JSON.stringify(action)}
      <button onclick={onClose}> Close</button>
    </div>

    {#if action.inspector !== null}
      <div class="right">
        <PropertyInspector
          pluginId={action.plugin_id}
          inspector={action.inspector}
        />
      </div>
    {/if}
  {:else if $tileQuery.isError}
    Failed to load tile: {getErrorMessage($tileQuery.error)}
  {:else if $actionQuery.isError}
    Failed to load action: {getErrorMessage($actionQuery.error)}
  {:else if $tileQuery.isLoading}
    Loading tile...
  {:else if $actionQuery.isLoading}
    Loading action...
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
    overflow-x: hidden;
    overflow-y: auto;
  }

  .right {
    display: flex;
    flex: auto;
    flex-shrink: 0;
    overflow: hidden;
  }
</style>
