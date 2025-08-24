<script lang="ts">
  import type { PluginId } from "$lib/api/types/plugin";
  import type { FolderId } from "$lib/api/types/folders";
  import type { ActionId, ActionCategory } from "$lib/api/types/actions";
  import type { ActionCategoryData } from "$lib/components/actions/ActionCategory.svelte";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { createActionsQuery } from "$lib/api/actions";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { serverContext } from "$lib/contexts/server.context";
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import FolderProvider from "$lib/components/folders/FolderProvider.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";
  import TileDraggingProvider from "$lib/components/tiles/TileDraggingProvider.svelte";
  import {
    type TileId,
    TileIconType,
    type TileIcon,
  } from "$lib/api/types/tiles";
  import {
    createCreateTileMutation,
    createUpdateTilePositionMutation,
  } from "$lib/api/tiles";

  const i18n = i18nContext.get();

  const actionsQuery = createActionsQuery();
  const actions = $derived(actionsQuery.data ?? []);

  const createTile = createCreateTileMutation();
  const updateTilePosition = createUpdateTilePositionMutation();

  const currentServerContext = serverContext.get();

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
</script>

<ProfilesProvider>
  <FolderProvider>
    {#snippet content({ folder })}
      <TileDraggingProvider
        {onMoveTile}
        onPlaceTile={(pluginId, actionId, row, column) => {
          onPlaceTile(folder.id, pluginId, actionId, row, column);
        }}
      >
        <div class="layout">
          <FolderEditor />
          <ActionsSidebar
            actions={actionCategoryData}
            actionsLoading={actionsQuery.isLoading}
            actionsError={actionsQuery.error}
          />
        </div>
      </TileDraggingProvider>
    {/snippet}
  </FolderProvider>
</ProfilesProvider>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
    overflow: hidden;
  }
</style>
