<!-- Editor for editing the contents of a folder -->
<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { t } from "svelte-i18n";
  import { createTilesQuery } from "$lib/api/tiles";
  import { getErrorMessage } from "$lib/api/utils/error";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";
  import SolarAltArrowRightLinear from "~icons/solar/alt-arrow-right-linear";

  import Aside from "../Aside.svelte";
  import TileEditor from "../tiles/TileEditor.svelte";
  import FolderSelector from "./FolderSelector.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
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
      <SkeletonList />
    </div>
  {:else if $tilesQuery.isError}
    <div class="content">
      <Aside severity="error" style="width: 100%">
        {$t("tiles_error", {
          values: { error: getErrorMessage($tilesQuery.error) },
        })}
      </Aside>
    </div>
  {:else if $tilesQuery.isSuccess}
    <div class="header">
      <ProfileSelector />
      <SolarAltArrowRightLinear />
      <FolderSelector />
    </div>
    <div class="content">
      <TileGrid
        tiles={$tilesQuery.data}
        rows={currentFolder.config.rows}
        columns={currentFolder.config.columns}
        onClickTile={(tile) => {
          if (activeTileId === tile.id) {
            activeTileId = null;
          } else {
            activeTileId = tile.id;
          }
        }}
      ></TileGrid>
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
