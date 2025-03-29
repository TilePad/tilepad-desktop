<script lang="ts">
  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";
  import { getIconAssetPath, getPluginAssetPath } from "$lib/api/utils/url";

  type Props = {
    icon: TileIcon;
  };

  const { icon }: Props = $props();
</script>

{#if icon.type === TileIconType.PluginIcon}
  <img
    class="tile__icon"
    src={getPluginAssetPath(icon.plugin_id, icon.icon)}
    alt="Tile Icon"
  />
{:else if icon.type === TileIconType.IconPack}
  <img
    class="tile__icon"
    src={getIconAssetPath(icon.pack_id, icon.path)}
    alt="Tile Icon"
  />
{/if}

<style>
  .tile__icon {
    width: 100%;
  }
</style>
