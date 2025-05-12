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

  import { getServerContext } from "../ServerProvider.svelte";

  type Props = {
    icon: TileIcon;
    iconOptions: TileIconOptions;
  };

  const { icon, iconOptions }: Props = $props();

  const serverContext = getServerContext();

  const src = $derived(getIconSrc(icon));
  const style = $derived(
    `padding: calc(${iconOptions.padding}px * var(--tile-size-adjustment)); background-color: ${iconOptions.background_color}`,
  );

  let error = $state(false);

  function onError() {
    error = true;
  }

  function getIconSrc(icon: TileIcon) {
    switch (icon.type) {
      case TileIconType.PluginIcon:
        return getPluginAssetPath(
          serverContext.serverURL,
          icon.plugin_id,
          icon.icon,
        );
      case TileIconType.IconPack:
        return getIconAssetPath(
          serverContext.serverURL,
          icon.pack_id,
          icon.path,
        );
      case TileIconType.Uploaded:
        return getUploadedIconAssetPath(serverContext.serverURL, icon.path);
      case TileIconType.Url:
        return icon.src;
      default:
        return null;
    }
  }
</script>

{#if src !== null}
  <img
    class="tile__icon"
    class:tile__icon--error={error}
    {src}
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
