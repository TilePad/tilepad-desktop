<!-- Empty tile that can be used to add a new tile -->
<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import { toast } from "svelte-sonner";
  import { TileIconType } from "$lib/api/types/tiles";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createCreateTileMutation } from "$lib/api/tiles";

  import ActionChooser from "../actions/ActionChooser.svelte";
  import { getFolderContext } from "../folders/FolderProvider.svelte";

  type Props = {
    row: number;
    column: number;
  };

  const { row, column }: Props = $props();

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const createTile = createCreateTileMutation();

  function onSelectAction(action: Action) {
    const createPromise = $createTile.mutateAsync({
      create: {
        row,
        column,
        folder_id: currentFolder.id,
        config: {
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
          properties: {},
        },
      },
    });

    toast.promise(createPromise, {
      loading: "Creating tile",
      success: "Created tile",
      error: toastErrorMessage("Failed to create tile"),
    });
  }
</script>

{#if $createTile.isIdle || $createTile.isError}
  <ActionChooser onSelect={onSelectAction}>
    {#snippet button({ props })}
      <button {...props} class="tile"> + </button>
    {/snippet}
  </ActionChooser>
{:else if $createTile.isPending}
  Creating...
{/if}

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
    cursor: pointer;
    width: 100%;
    height: 100%;
    color: #ccc;
    font-size: 1.5rem;
  }
</style>
