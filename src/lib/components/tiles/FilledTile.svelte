<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { toast } from "svelte-sonner";
  import { deleteTile } from "$lib/api/tiles";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import { getFolderContext } from "../folders/FolderProvider.svelte";

  type Props = {
    tile: TileModel;
  };

  const { tile }: Props = $props();

  const config = $derived(tile.config);

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  function onRemove() {
    const deletePromise = deleteTile(currentFolder.id, tile.id);

    toast.promise(deletePromise, {
      loading: "Deleting tile",
      success: "Deleted tile",
      error: toastErrorMessage("Failed to delete tile"),
    });
  }
</script>

<div class="tile">
  <button onclick={onRemove} class="remove"> R </button>

  {#if config.icon.type === "PluginIcon"}
    <img
      class="tile__icon"
      src={getPluginAssetPath(config.icon.plugin_id, config.icon.icon)}
      alt="Tile Icon"
    />
  {/if}
</div>

<style>
  .remove {
    position: absolute;
    top: 0;
    right: 0;
    width: 32px;
  }

  .tile {
    position: relative;
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

  .tile__icon {
    width: 3.5rem;
  }
</style>
