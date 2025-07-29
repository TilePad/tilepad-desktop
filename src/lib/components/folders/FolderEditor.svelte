<!-- Editor for editing the contents of a folder -->
<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";

  import { watch } from "runed";
  import { t } from "svelte-i18n";
  import { fly } from "svelte/transition";
  import { createTilesQuery } from "$lib/api/tiles";
  import { createFoldersQuery } from "$lib/api/folders";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createProfilesQuery } from "$lib/api/profiles";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";
  import RightArrowIcon from "~icons/solar/alt-arrow-right-linear";

  import Aside from "../Aside.svelte";
  import TileEditor from "../tiles/TileEditor.svelte";
  import FolderSelector from "./FolderSelector.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import CreateFolderDialog from "./CreateFolderDialog.svelte";
  import ProfileSelector from "../profiles/ProfileSelector.svelte";
  import FolderSelectorSettings from "./FolderSelectorSettings.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";
  import CreateProfileDialog from "../profiles/CreateProfileDialog.svelte";
  import ProfileSelectorSettings from "../profiles/ProfileSelectorSettings.svelte";

  const { profile, setProfileId } = getProfileContext();
  const { folder, setFolderId } = getFolderContext();

  const currentFolder = $derived.by(folder);
  const currentProfile = $derived.by(profile);

  const currentFolderId = $derived(currentFolder.id);
  const currentProfileId = $derived(currentProfile.id);

  const profilesQuery = createProfilesQuery();
  const profiles = $derived($profilesQuery.data ?? []);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const folders = $derived($foldersQuery.data ?? []);

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
      <div class="profile-options-group">
        <ProfileSelector
          options={profiles}
          value={currentProfileId}
          onChangeValue={setProfileId}
        />

        <ProfileSelectorSettings profile={currentProfile} />
        <CreateProfileDialog order={profiles.length} />
      </div>

      <RightArrowIcon />

      <div class="folder-options-group">
        <FolderSelector
          options={folders}
          value={currentFolderId}
          onChangeValue={setFolderId}
        />
        <FolderSelectorSettings folder={currentFolder} />
        <CreateFolderDialog order={folders.length} />
      </div>
    </div>
    <div class="content__wrapper">
      {#key currentFolderId}
        <div
          class="content"
          in:fly={{ x: -50, duration: 300, opacity: 0 }}
          out:fly={{ x: 50, duration: 300, opacity: 0 }}
        >
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
      {/key}
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
    padding: var(--tp-space-4);
    overflow: hidden;

    display: flex;
    gap: 0.5rem;
    align-items: center;
  }

  .content {
    height: 100%;
    width: 100%;
    position: absolute;
    left: 0;
    top: 0;

    overflow: hidden;
    padding: 1rem;
    padding-top: 0;
  }

  .content__wrapper {
    position: relative;
    flex: auto;
    overflow: hidden;
  }

  .folder-options-group,
  .profile-options-group {
    display: flex;
  }
</style>
