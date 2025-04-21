<script lang="ts">
  import {
    TileIconType,
    type TileIcon,
    type TileIconOptions,
  } from "$lib/api/types/tiles";
  import {
    getIconAssetPath,
    getPluginAssetPath,
    getUploadedIconAssetPath,
  } from "$lib/api/utils/url";

  type Props = {
    icon: TileIcon;
    iconOptions: TileIconOptions;
  };

  const { icon, iconOptions }: Props = $props();

  const style = $derived(
    `padding: calc(${iconOptions.padding}px * var(--tile-size-adjustment)); background-color: ${iconOptions.background_color}`,
  );

  let error = $state(false);

  function onError(event: Event) {
    error = true;
  }
</script>

{#if icon.type === TileIconType.PluginIcon}
  <img
    class="tile__icon"
    class:tile__icon--error={error}
    src={getPluginAssetPath(icon.plugin_id, icon.icon)}
    alt="Tile Icon"
    onerror={onError}
    {style}
  />
{:else if icon.type === TileIconType.IconPack}
  <img
    class="tile__icon"
    class:tile__icon--error={error}
    src={getIconAssetPath(icon.pack_id, icon.path)}
    alt="Tile Icon"
    onerror={onError}
    {style}
  />
{:else if icon.type === TileIconType.Uploaded}
  <img
    class="tile__icon"
    class:tile__icon--error={error}
    src={getUploadedIconAssetPath(icon.path)}
    alt="Tile Icon"
    onerror={onError}
    {style}
  />
{:else}
  <div class="tile__icon" {style}></div>
{/if}

<style>
  .tile__icon {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .tile__icon--error {
    display: none;
  }
</style>
