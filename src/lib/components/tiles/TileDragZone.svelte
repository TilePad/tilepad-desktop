<script lang="ts" module>
  import type { Action } from "$lib/api/types/actions";

  import { TileIconType, type TileModel } from "$lib/api/types/tiles";

  import type { MovableAction } from "../actions/ActionList.svelte";

  export type TileDropItem = TileDropItemAction | TileDropItemTile;

  export type TileDropItemAction = { type: "Action" } & Action &
    TileDropItemBase;

  export type TileDropItemTile = { type: "Tile" } & TileModel &
    TileDropItemBase;

  export type TileDropItemBase = {
    id: string;
    [SHADOW_ITEM_MARKER_PROPERTY_NAME]?: boolean;
  };
</script>

<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import {
    createCreateTileMutation,
    createUpdateTilePositionMutation,
  } from "$lib/api/tiles";
  import {
    dndzone,
    type DndEvent,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
  } from "svelte-dnd-action";

  import FilledTile from "./FilledTile.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";

  type Props = {
    row: number;
    column: number;
    tile: TileModel | undefined;
    onClickTile: (tile: TileModel) => void;
  };

  const { row, column, tile, onClickTile }: Props = $props();

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const createTile = createCreateTileMutation();
  const updateTilePosition = createUpdateTilePositionMutation();

  let items: TileDropItem[] = $state(createTileItems(tile));

  function handleDndConsider(e: CustomEvent<DndEvent<TileDropItem>>) {
    items = e.detail.items;
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<TileDropItem>>) {
    items = e.detail.items;

    if (items.length < 1) return;

    const item = e.detail.items[0];

    if (item.type === "Tile") {
      const model = item as TileModel;
      $updateTilePosition.mutateAsync({
        tileId: model.id,
        row,
        column,
      });
    } else if (item.type === "Action") {
      const action = item as MovableAction;
      const createPromise = $createTile.mutateAsync({
        create: {
          row,
          column,
          folder_id: currentFolder.id,
          action_id: action.action_id,
          plugin_id: action.plugin_id,
          config: {
            icon:
              action.icon === null
                ? { type: TileIconType.None }
                : {
                    type: TileIconType.PluginIcon,
                    plugin_id: action.plugin_id,
                    icon: action.icon,
                  },
          },
          properties: {},
        },
      });

      toast.promise(createPromise, {
        loading: $t("tile_creating"),
        success: $t("tile_created"),
        error: toastErrorMessage($t("tile_create_error")),
      });
    }
  }

  function createTileItems(tile: TileModel | undefined): TileDropItem[] {
    return tile ? [{ type: "Tile", ...tile }] : [];
  }

  $effect(() => {
    items = createTileItems(tile);
  });
</script>

<div
  class="tile"
  class:tile--filled={!!items.length}
  class:tile--hovered={items.length > 0 &&
    items[0][SHADOW_ITEM_MARKER_PROPERTY_NAME]}
  use:dndzone={{
    type: "tile",
    items,
    dropTargetStyle: {},
    flipDurationMs: 0,
    morphDisabled: true,
    centreDraggedOnCursor: true,
    dropAnimationDisabled: true,
    // Disable dragging on an empty slot
    dragDisabled: items.length < 1,
    dropFromOthersDisabled: items.length > 0,
  }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  {#each items as item (item.id)}
    {#if item.type === "Tile"}
      <FilledTile tile={item} onClick={() => onClickTile(item)} />
    {:else if item.type === "Action"}
      <div></div>
    {/if}
  {/each}
</div>

<style>
  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-width);
    color: #ccc;
    user-select: none;
    overflow: hidden;
  }

  .tile--hovered {
    background-color: #3a3542;
    border-color: #36313d;
  }

  .tile--filled {
    border: 2px solid #715c8f;
    cursor: pointer;
  }
</style>
