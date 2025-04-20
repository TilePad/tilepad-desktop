<!-- Editor for editing the contents of a folder -->
<script lang="ts">
  import type { TileId, TileModel } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { createTilesQuery } from "$lib/api/tiles";
  import { getErrorMessage } from "$lib/api/utils/error";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";
  import SolarAltArrowRightLinear from "~icons/solar/alt-arrow-right-linear";

  import Aside from "../Aside.svelte";
  import EmptyTile from "../tiles/EmptyTile.svelte";
  import FilledTile from "../tiles/FilledTile.svelte";
  import TileEditor from "../tiles/TileEditor.svelte";
  import FolderSelector from "./FolderSelector.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import ProfileSelector from "../profiles/ProfileSelector.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  const { profile } = getProfileContext();
  const { folder } = getFolderContext();

  const currentFolder = $derived.by(folder);
  const currentProfile = $derived.by(profile);

  const currentFolderId = $derived(currentFolder.id);
  const currentProfileId = $derived(currentProfile.id);

  const tilesQuery = createTilesQuery(() => currentFolderId);

  let activeTileId: TileId | null = $state(null);

  function getTile(tiles: TileModel[], row: number, column: number) {
    return tiles.find((tile) => tile.row === row && tile.column === column);
  }

  // Clear the active tile whenever switching profile or folders
  watch(
    () => ({ currentFolderId, currentProfileId }),
    () => {
      activeTileId = null;
    },
  );
</script>

<div class="layout">
  {#if $tilesQuery.isLoading}
    <div class="content">
      <div class="skeleton-list">
        <div class="skeleton" style="width: 80%; height: 1rem"></div>
        <div class="skeleton" style="width: 70%; height: 1rem"></div>
        <div class="skeleton" style="width: 30%; height: 1rem"></div>
      </div>
    </div>
  {:else if $tilesQuery.isError}
    <div class="content">
      <Aside severity="error" style="width: 100%">
        Failed to load tiles: {getErrorMessage($tilesQuery.error)}
      </Aside>
    </div>
  {:else if $tilesQuery.isSuccess}
    <div class="header">
      <div>
        <ProfileSelector />
      </div>

      <SolarAltArrowRightLinear />

      <div>
        <FolderSelector />
      </div>
    </div>
    <div class="content">
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

    <!-- Bottom segment that pops up to edit a tile -->
    <TileEditor tileId={activeTileId} onClose={() => (activeTileId = null)} />
  {/if}
</div>

<style>
  .layout {
    display: flex;
    flex: auto;
    flex-flow: column;

    height: 100%;

    overflow: hidden;
  }

  .header {
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    overflow: hidden;
    background-color: #29262e;

    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .content {
    flex: auto;
    padding: 1rem;
    overflow: hidden;
  }
</style>
