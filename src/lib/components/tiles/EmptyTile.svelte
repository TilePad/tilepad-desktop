<!-- Empty tile that can be used to add a new tile -->
<script lang="ts">
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createCreateTileMutation } from "$lib/api/tiles";
  import { LabelAlign, TileIconType } from "$lib/api/types/tiles";
  import {
    dndzone,
    type DndEvent,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
  } from "svelte-dnd-action";

  import type { MovableAction } from "../actions/ActionList.svelte";

  import { getFolderContext } from "../folders/FolderProvider.svelte";

  type Props = {
    row: number;
    column: number;
  };

  let item: MovableAction | undefined = $state(undefined);

  const { row, column }: Props = $props();

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const createTile = createCreateTileMutation();

  function handleDndConsider(e: CustomEvent<DndEvent<MovableAction>>) {
    item = e.detail.items[0];
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<MovableAction>>) {
    if (e.detail.items.length < 1) return;

    const action = e.detail.items[0];
    const createPromise = $createTile.mutateAsync({
      create: {
        row,
        column,
        folder_id: currentFolder.id,
        config: {
          label: {
            enabled: true,
            label: "",
            align: LabelAlign.Bottom,
            font_size: 12,
            bold: false,
            italic: false,
            underline: false,
            color: "#ffffff",
          },
          action_id: action.action_id,
          plugin_id: action.plugin_id,
          icon:
            action.icon === null
              ? { type: TileIconType.None }
              : {
                  type: TileIconType.PluginIcon,
                  plugin_id: action.plugin_id,
                  icon: action.icon,
                },
          user_flags: {
            icon: false,
            label: false,
          },
        },
        properties: {},
      },
    });

    toast.promise(createPromise, {
      loading: "Creating tile",
      success: "Created tile",
      error: toastErrorMessage("Failed to create tile"),
    });
  }
</script>

<div
  class="tile"
  class:tile--hovered={item && item[SHADOW_ITEM_MARKER_PROPERTY_NAME]}
  use:dndzone={{
    items: item ? [item] : [],
    dropTargetStyle: {},
    flipDurationMs: 0,
    morphDisabled: true,
    centreDraggedOnCursor: true,
    dragDisabled: true,
  }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  <div></div>
</div>

<style>
  .tile {
    background-color: #242129;
    border: 2px solid #36313d;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    width: 100%;
    height: 100%;
    color: #ccc;
    font-size: 1.5rem;
    user-select: none;
    overflow: hidden;
  }

  .tile--hovered {
    background-color: #3a3542;
    border-color: #36313d;
  }
</style>
