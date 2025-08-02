<script lang="ts">
  import type { ActionWithCategory } from "$lib/api/types/actions";

  import { watch, useDebounce } from "runed";
  import {
    type DisplayContext,
    CONTROLLER_DEVICE_ID,
  } from "$lib/api/types/plugin";
  import {
    createUpdateTileIconMutation,
    createUpdateTileIconOptionsMutation,
  } from "$lib/api/tiles";
  import {
    UpdateKind,
    type TileId,
    TileIconType,
    type TileConfig,
    type TileIconOptions,
    type TileIcon as ITileIcon,
  } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import TileLabel from "./TileLabel.svelte";
  import IconSelector from "../icons/IconSelector.svelte";

  type Props = {
    tileId: TileId;
    action: ActionWithCategory | undefined;
    config: TileConfig;
  };

  const { tileId, action, config }: Props = $props();

  const tileSize = 120;

  const updateTileIcon = createUpdateTileIconMutation();
  const updateTileIconOptions = createUpdateTileIconOptionsMutation();
  const displayCtx: DisplayContext = $derived({
    device_id: CONTROLLER_DEVICE_ID,
    tile_id: tileId,
    action_id: action?.action_id ?? "",
    plugin_id: action?.plugin_id ?? "",
  });

  let lastOptionsUpdate: TileIconOptions = $state(config.icon_options);
  let iconOptions = $state(config.icon_options);

  const onClickIconPackIcon = (icon: ITileIcon) => {
    updateTileIcon.mutate({
      tileId,
      icon,
      kind: UpdateKind.User,
    });
  };

  const onResetDefault = () => {
    let icon: ITileIcon = { type: TileIconType.None };
    if (action) {
      if (action.display) {
        icon = { type: TileIconType.Display, path: action.display };
      } else if (action.icon) {
        icon = {
          type: TileIconType.PluginIcon,
          plugin_id: action.plugin_id,
          icon: action.icon,
        };
      }
    }

    const defaultIconOptions = {
      ...iconOptions,
      padding: 0,
      background_color: "#00000000",
      border_color: "#715c8f",
      ...(action?.icon_options ?? {}),
    };

    console.log(action?.icon_options, defaultIconOptions);

    updateTileIcon.mutate({
      tileId,
      icon,
      kind: UpdateKind.Reset,
    });

    iconOptions = defaultIconOptions;
    updateIconOptions(iconOptions);
  };

  const updateIconOptions = useDebounce((iconOptions: TileIconOptions) => {
    lastOptionsUpdate = iconOptions;

    updateTileIconOptions.mutate({
      tileId,
      iconOptions,
    });
  }, 150);

  const onChangePadding = (padding: number) => {
    iconOptions = { ...iconOptions, padding };
    updateIconOptions(iconOptions);
  };

  const onChangeBackgroundColor = (color: string) => {
    iconOptions = { ...iconOptions, background_color: color };
    updateIconOptions(iconOptions);
  };

  const onChangeBorderColor = (color: string) => {
    iconOptions = { ...iconOptions, border_color: color };
    updateIconOptions(iconOptions);
  };

  // Only update icon options state when remote state is different from the
  // last debounced saved state (Prevent UI flickering from controlled changes)
  watch(
    () => ({ config }),
    ({ config }) => {
      if (
        JSON.stringify(lastOptionsUpdate) ===
        JSON.stringify(config.icon_options)
      )
        return;
      iconOptions = config.icon_options;
    },
  );
</script>

<div
  class="tile-container"
  style="--tile-size-adjustment: 1; --tile-width: {tileSize}px; --tile-height: {tileSize}px; --tile-border-color: {iconOptions.border_color}"
>
  <div class="tile">
    <TileIcon ctx={displayCtx} icon={config.icon} {iconOptions} />
    <TileLabel {...config.label} />
  </div>
</div>

<IconSelector
  {iconOptions}
  onSelectIcon={onClickIconPackIcon}
  {onChangeBackgroundColor}
  {onChangeBorderColor}
  {onChangePadding}
  {onResetDefault}
/>

<style>
  .tile {
    position: relative;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    width: var(--tile-width);
    height: var(--tile-height);
    color: #ccc;

    border: 2px solid var(--tile-border-color);
    cursor: pointer;

    background-color: #151318;

    user-select: none;
    overflow: hidden;
  }
</style>
