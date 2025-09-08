<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";
  import type { PluginId } from "$lib/api/types/plugin";
  import type { FolderId } from "$lib/api/types/folders";
  import type { ActionId, ActionCategory } from "$lib/api/types/actions";
  import type { ActionCategoryData } from "$lib/components/actions/ActionCategory.svelte";

  import { watch } from "runed";
  import { toast } from "svelte-sonner";
  import { fly } from "svelte/transition";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createFoldersQuery } from "$lib/api/folders";
  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { createProfilesQuery } from "$lib/api/profiles";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import TileGrid from "$lib/components/tiles/TileGrid.svelte";
  import { serverContext } from "$lib/contexts/server.context";
  import RightArrowIcon from "~icons/solar/alt-arrow-right-linear";
  import TileEditor from "$lib/components/tiles/TileEditor.svelte";
  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";
  import SkeletonList from "$lib/components/skeleton/SkeletonList.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import FolderSelector from "$lib/components/folders/FolderSelector.svelte";
  import DeleteTileDialog from "$lib/components/tiles/DeleteTileDialog.svelte";
  import ProfileSelector from "$lib/components/profiles/ProfileSelector.svelte";
  import { getFolderContext } from "$lib/components/folders/FolderProvider.svelte";
  import CreateFolderDialog from "$lib/components/folders/CreateFolderDialog.svelte";
  import TileDraggingProvider from "$lib/components/tiles/TileDraggingProvider.svelte";
  import { getProfileContext } from "$lib/components/profiles/ProfilesProvider.svelte";
  import CreateProfileDialog from "$lib/components/profiles/CreateProfileDialog.svelte";
  import FolderSelectorSettings from "$lib/components/folders/FolderSelectorSettings.svelte";
  import ProfileSelectorSettings from "$lib/components/profiles/ProfileSelectorSettings.svelte";
  import {
    createTilesQuery,
    createCreateTileMutation,
    createUpdateTilePositionMutation,
  } from "$lib/api/tiles";

  const i18n = i18nContext.get();

  const { profile, setProfileId } = getProfileContext();
  const { folder, setFolderId } = getFolderContext();

  const actionsQuery = createActionsQuery();
  const actions = $derived(actionsQuery.data ?? []);

  const currentFolder = $derived.by(folder);
  const currentProfile = $derived.by(profile);

  const currentFolderId = $derived(currentFolder.id);
  const currentProfileId = $derived(currentProfile.id);

  const profilesQuery = createProfilesQuery();
  const profiles = $derived(profilesQuery.data ?? []);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const folders = $derived(foldersQuery.data ?? []);

  const tilesQuery = createTilesQuery(() => currentFolderId);
  const tiles = $derived(tilesQuery.data ?? []);

  const createTile = createCreateTileMutation();
  const updateTilePosition = createUpdateTilePositionMutation();

  const currentServerContext = serverContext.get();

  let deleteTileId: TileId | null = $state(null);
  let activeTileId: TileId | null = $state(null);

  const deleteTile = $derived(tiles.find((tile) => tile.id === deleteTileId));

  const actionCategoryData: ActionCategoryData[] = $derived.by(() => {
    return actions.map((category) => ({
      pluginId: category.plugin_id,
      label: category.label,
      icon: category.icon
        ? getPluginAssetPath(
            currentServerContext.serverURL,
            category.plugin_id,
            category.icon,
          )
        : undefined,
      actions: category.actions.map((action) => ({
        pluginId: action.plugin_id,
        actionId: action.action_id,
        label: action.label,
        icon: action.icon
          ? getPluginAssetPath(
              currentServerContext.serverURL,
              action.plugin_id,
              action.icon,
            )
          : undefined,
      })),
    }));
  });

  function getActionById(
    actions: ActionCategory[],
    actionId: ActionId,
    pluginId: PluginId,
  ) {
    for (const category of actions) {
      if (category.plugin_id !== pluginId) continue;

      for (const action of category.actions) {
        if (action.action_id === actionId) {
          return action;
        }
      }
    }

    return undefined;
  }

  function onMoveTile(tileId: TileId, row: number, column: number) {
    updateTilePosition.mutateAsync({
      tileId,
      position: {
        row,
        column,
        // Reset spanning on move
        row_span: 1,
        column_span: 1,
      },
    });
  }

  function onPlaceTile(
    folderId: FolderId,
    pluginId: PluginId,
    actionId: ActionId,
    row: number,
    column: number,
  ) {
    const action = getActionById(actions, actionId, pluginId);
    if (action === undefined) return;

    let icon: TileIcon = { type: TileIconType.None };
    if (action.display) {
      icon = { type: TileIconType.Display, path: action.display };
    } else if (action.icon) {
      icon = {
        type: TileIconType.PluginIcon,
        plugin_id: action.plugin_id,
        icon: action.icon,
      };
    }

    const createPromise = createTile.mutateAsync({
      create: {
        position: {
          row,
          column,
          row_span: 1,
          column_span: 1,
        },
        folder_id: folderId,
        action_id: action.action_id,
        plugin_id: action.plugin_id,
        config: {
          icon,
          icon_options: action.icon_options ? action.icon_options : undefined,
        },
        properties: {},
      },
    });

    toast.promise(createPromise, {
      loading: i18n.f("tile_creating"),
      success: i18n.f("tile_created"),
      error: toastErrorMessage(i18n.f("tile_create_error")),
    });
  }

  // Clear the active tile whenever switching profile or folders
  watch(
    () => ({ currentFolderId, currentProfileId }),
    () => {
      activeTileId = null;
    },
  );
</script>

<TileDraggingProvider
  {onMoveTile}
  onPlaceTile={(pluginId, actionId, row, column) => {
    onPlaceTile(currentFolder.id, pluginId, actionId, row, column);
  }}
  onDeleteTile={(tileId) => {
    deleteTileId = tileId;
  }}
>
  <div class="layout">
    <div class="inner-layout">
      {#if tilesQuery.isLoading}
        <div class="content">
          <SkeletonList style="margin-top: 1rem" />
        </div>
      {:else if tilesQuery.isError}
        <div class="content">
          <Aside severity="error" style="width: 100%">
            {i18n.f("tiles_error", {
              values: { error: getErrorMessage(tilesQuery.error) },
            })}
          </Aside>
        </div>
      {:else if tilesQuery.isSuccess}
        <div class="header">
          <div class="profile-options-group">
            <ProfileSelector
              options={profiles}
              value={currentProfileId}
              onChangeValue={setProfileId}
            />

            <ProfileSelectorSettings profile={currentProfile} />
            <CreateProfileDialog
              order={profiles.length}
              onCreated={(profileId) => {
                // Switch to created profile
                setProfileId(profileId);
              }}
            />
          </div>

          <RightArrowIcon />

          <div class="folder-options-group">
            <FolderSelector
              options={folders}
              value={currentFolderId}
              onChangeValue={setFolderId}
            />
            <FolderSelectorSettings
              profileId={currentProfileId}
              folder={currentFolder}
            />
            <CreateFolderDialog
              profileId={currentProfileId}
              baseConfig={currentFolder.config}
              order={folders.length}
              onCreated={(folderId) => {
                // Switch to created folder
                setFolderId(folderId);
              }}
            />
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
                {tiles}
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
        <TileEditor
          profileId={currentProfileId}
          folderId={currentFolderId}
          tileId={activeTileId}
          onClose={() => (activeTileId = null)}
        />
      {/if}
    </div>

    <ActionsSidebar
      actions={actionCategoryData}
      actionsLoading={actionsQuery.isLoading}
      actionsError={actionsQuery.error}
    />

    {#if deleteTile !== undefined}
      <DeleteTileDialog
        tile={deleteTile}
        onClose={() => {
          deleteTileId = null;
        }}
        alwaysOpen
      />
    {/if}
  </div>
</TileDraggingProvider>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
    overflow: hidden;
  }

  .inner-layout {
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
