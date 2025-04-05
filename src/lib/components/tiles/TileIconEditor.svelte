<script lang="ts">
  import { updateTile } from "$lib/api/tiles";
  import {
    type TileId,
    type TileConfig,
    type TileIcon as ITileIcon,
  } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";
  import IconSelector from "../icons/IconSelector.svelte";

  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  const onClickIconPackIcon = (icon: ITileIcon) => {
    updateTile(tileId, {
      config: {
        ...config,
        icon,
      },
    });
  };

  const desiredWidth = 120;
  const tileWidth = 100;
  const sizeAdjust = $derived.by(() => {
    const ratio = (tileWidth - desiredWidth) / desiredWidth;
    return 1 - Math.max(0.0, -ratio);
  });
</script>

<div class="tile" style="--font-size-adjustment: {sizeAdjust};">
  <IconSelector onSelectIcon={onClickIconPackIcon} />
  <TileIcon icon={config.icon} />
  <TileLabelElm label={config.label} />
</div>

<style>
  .tile {
    position: relative;
    background-color: #242129;
    border: 2px solid #715c8f;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    width: 100px;
    height: 100px;
    color: #ccc;
    font-size: 1.5rem;
  }
</style>
