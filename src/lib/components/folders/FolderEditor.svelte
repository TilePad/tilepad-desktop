<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { createTilesQuery } from "$lib/api/tiles";
  import { getErrorMessage } from "$lib/api/utils/error";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";

  import Tile from "../tiles/Tile.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const tilesQuery = createTilesQuery(() => currentFolder.id);

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
    <TileGrid
      rows={currentFolder.config.rows}
      columns={currentFolder.config.columns}
    >
      {#snippet tile(row, column)}
        <Tile
          tile={getTile($tilesQuery.data, row, column) ?? null}
          {row}
          {column}
        />
      {/snippet}
    </TileGrid>
  </div>
{/if}

<style>
  .layout {
    display: flex;
    flex: auto;
    flex-flow: column;

    padding: 1rem;
    height: 100%;

    overflow: hidden;
  }
</style>
