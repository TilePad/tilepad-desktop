<script lang="ts">
  import type { ActionWithCategory } from "$lib/api/types/actions";

  import { createUpdateTileMutation } from "$lib/api/tiles";
  import {
    type TileId,
    TileIconType,
    type TileConfig,
    type TileIcon as ITileIcon,
  } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";
  import IconSelector from "../icons/IconSelector.svelte";

  type Props = {
    tileId: TileId;
    action: ActionWithCategory | undefined;
    config: TileConfig;
  };

  const { tileId, action, config }: Props = $props();

  const updateTile = createUpdateTileMutation();

  const onClickIconPackIcon = (icon: ITileIcon) => {
    $updateTile.mutate({
      tileId,
      update: {
        config: {
          ...config,
          icon,
          user_flags: {
            ...config.user_flags,
            icon: true,
          },
        },
      },
    });
  };

  const onResetDefault = () => {
    $updateTile.mutate({
      tileId,
      update: {
        config: {
          ...config,
          icon:
            !action || action.icon === null
              ? { type: TileIconType.None }
              : {
                  type: TileIconType.PluginIcon,
                  plugin_id: action.plugin_id,
                  icon: action.icon,
                },
          user_flags: {
            ...config.user_flags,
            icon: false,
          },
        },
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
  <IconSelector onSelectIcon={onClickIconPackIcon} {onResetDefault} />
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
