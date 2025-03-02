<script lang="ts">
  import type { TileId, TileModel } from "$lib/api/types/tiles";

  import { createTilesQuery } from "$lib/api/tiles";
  import { getErrorMessage } from "$lib/api/utils/error";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import EmptyTile from "../tiles/EmptyTile.svelte";
  import FilledTile from "../tiles/FilledTile.svelte";
  import TileEditor from "../tiles/TileEditor.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const tilesQuery = createTilesQuery(() => currentFolder.id);

  let activeTileId: TileId | null = $state(null);

  function getTile(tiles: TileModel[], row: number, column: number) {
    return tiles.find((tile) => tile.row === row && tile.column === column);
  }
</script>

{#if $tilesQuery.isLoading}
  Loading tiles...
{:else if $tilesQuery.isError}
  Failed to load tiles {getErrorMessage($tilesQuery.error)}
{:else if $tilesQuery.isSuccess}
  <div class="layout">
    <div class="grid-wrapper">
      <TileGrid
        rows={currentFolder.config.rows}
        columns={currentFolder.config.columns}
      >
        {#snippet tile(row, column)}
          {@const tile = getTile($tilesQuery.data, row, column) ?? null}
          {#if tile !== null}
            <FilledTile
              {tile}
              onClick={() => {
                if (activeTileId === tile.id) {
                  activeTileId = null;
                } else {
                  activeTileId = tile.id;
                }
              }}
            />
          {:else}
            <EmptyTile {row} {column} />
          {/if}
        {/snippet}
      </TileGrid>
    </div>

    {#if activeTileId !== null}
      <TileEditor tileId={activeTileId} onClose={() => (activeTileId = null)} />
    {/if}
  </div>
{/if}

<style>
  .layout {
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;

    overflow: hidden;
  }

  .grid-wrapper {
    flex: auto;
    padding: 1rem;
  }
</style>
