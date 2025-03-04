<script lang="ts">
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import { type TileId, type TileIcon } from "$lib/api/types/tiles";

  type Props = {
    tileId: TileId;
    icon: TileIcon;
  };

  const { tileId, icon }: Props = $props();
</script>

<div class="tile">
  {#if icon.type === "PluginIcon"}
    <img
      class="tile__icon"
      src={getPluginAssetPath(icon.plugin_id, icon.icon)}
      alt="Tile Icon"
    />
  {/if}
</div>

<style>
  .tile {
    position: relative;
    background-color: #000000;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    cursor: pointer;
    width: 80px;
    height: 80px;
    color: #ccc;
    font-size: 1.5rem;
  }

  .tile__icon {
    width: 3.5rem;
  }
</style>
