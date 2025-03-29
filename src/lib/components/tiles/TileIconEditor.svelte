<script lang="ts">
  import type { Icon, IconPackId } from "$lib/api/types/icons";

  import { updateTile } from "$lib/api/tiles";
  import {
    type TileId,
    TileIconType,
    type TileConfig,
  } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import IconSelector from "../icons/IconSelector.svelte";

  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  const onClickIconPackIcon = (packId: IconPackId, icon: Icon) => {
    updateTile(tileId, {
      config: {
        ...config,
        icon: { type: TileIconType.IconPack, pack_id: packId, path: icon.path },
      },
    });
  };
</script>

<div class="tile">
  <TileIcon icon={config.icon} />
</div>

<IconSelector onClickIcon={onClickIconPackIcon} />

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
    cursor: pointer;
    width: 80px;
    height: 80px;
    color: #ccc;
    font-size: 1.5rem;
  }
</style>
