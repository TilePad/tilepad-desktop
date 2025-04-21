<script lang="ts">
  import type { ActionWithCategory } from "$lib/api/types/actions";

  import { watch, useDebounce } from "runed";
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
  import TileLabelElm from "./TileLabelElm.svelte";
  import IconSelector from "../icons/IconSelector.svelte";

  type Props = {
    tileId: TileId;
    action: ActionWithCategory | undefined;
    config: TileConfig;
  };

  const { tileId, action, config }: Props = $props();

  const updateTileIcon = createUpdateTileIconMutation();
  const updateTileIconOptions = createUpdateTileIconOptionsMutation();

  let lastOptionsUpdate: TileIconOptions = $state(config.icon_options);
  let iconOptions = $state(config.icon_options);

  const onClickIconPackIcon = (icon: ITileIcon) => {
    $updateTileIcon.mutate({
      tileId,
      icon,
      kind: UpdateKind.User,
    });
  };

  const onResetDefault = () => {
    const defaultIcon: ITileIcon =
      !action || action.icon === null
        ? { type: TileIconType.None }
        : {
            type: TileIconType.PluginIcon,
            plugin_id: action.plugin_id,
            icon: action.icon,
          };

    $updateTileIcon.mutate({
      tileId,
      icon: defaultIcon,
      kind: UpdateKind.Reset,
    });
  };

  const updateIconOptions = useDebounce((iconOptions: TileIconOptions) => {
    lastOptionsUpdate = iconOptions;

    $updateTileIconOptions.mutate({
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

<div class="tile" style="--font-size-adjustment: 1;">
  <TileIcon icon={config.icon} {iconOptions} />
  <TileLabelElm label={config.label} />
</div>
<IconSelector
  {iconOptions}
  onSelectIcon={onClickIconPackIcon}
  {onChangeBackgroundColor}
  {onChangePadding}
  {onResetDefault}
/>

<style>
  .tile {
    position: relative;
    background-color: #151318;
    border: 2px solid #715c8f;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    width: 120px;
    height: 120px;
    color: #ccc;
    font-size: 1.5rem;
  }
</style>
