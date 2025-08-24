<script lang="ts">
  import { watch, useDebounce } from "runed";
  import SolarEyeBold from "~icons/solar/eye-bold";
  import { createFontsQuery } from "$lib/api/fonts";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import SolarEyeClosedBold from "~icons/solar/eye-closed-bold";
  import { createUpdateTileLabelMutation } from "$lib/api/tiles";
  import {
    LabelAlign,
    UpdateKind,
    type TileId,
    type TileLabel,
    type TileConfig,
  } from "$lib/api/types/tiles";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
  import TextInput from "../input/TextInput.svelte";
  import TileNameSettingsPopover from "./TileNameSettingsPopover.svelte";

  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  const i18n = i18nContext.get();

  const fontsQuery = createFontsQuery();
  const fonts = $derived(fontsQuery.data ?? []);
  const updateTileLabel = createUpdateTileLabelMutation();

  // Last persisted update
  let lastUpdate: TileLabel = $state(config.label);
  let label = $state(config.label);

  const updateLabelDebounced = useDebounce((label: TileLabel) => {
    lastUpdate = label;

    updateTileLabel.mutate({
      tileId,
      label,
      kind: UpdateKind.User,
    });
  }, 150);

  const onUpdateLabel = (action: (label: TileLabel) => TileLabel) => {
    label = action(label);
    updateLabelDebounced(label);
  };

  const onChangeTileName = (value: string) => {
    onUpdateLabel((label) => ({ ...label, label: value }));
  };

  const onChangeFontSize = (font_size: number) => {
    onUpdateLabel((label) => ({ ...label, font_size }));
  };

  const onChangeColor = (color: string) => {
    onUpdateLabel((label) => ({ ...label, color }));
  };

  const onChangeOutlineColor = (outline_color: string) => {
    onUpdateLabel((label) => ({ ...label, outline_color }));
  };

  const onToggleEnabled = () => {
    onUpdateLabel((label) => ({ ...label, enabled: !label.enabled }));
  };

  const onToggleBold = () => {
    onUpdateLabel((label) => ({ ...label, bold: !label.bold }));
  };

  const onToggleItalic = () => {
    onUpdateLabel((label) => ({ ...label, italic: !label.italic }));
  };

  const onToggleUnderline = () => {
    onUpdateLabel((label) => ({ ...label, underline: !label.underline }));
  };

  const onToggleOutline = () => {
    onUpdateLabel((label) => ({ ...label, outline: !label.outline }));
  };

  const onChangeAlign = (align: LabelAlign) => {
    onUpdateLabel((label) => ({ ...label, align }));
  };

  const onChangeFont = (font: string) => {
    onUpdateLabel((label) => ({ ...label, font }));
  };

  // Only update label state when remote state is different from the
  // last debounced saved state (Prevent UI flickering from controlled changes)
  watch(
    () => ({ config }),
    ({ config }) => {
      if (JSON.stringify(lastUpdate) === JSON.stringify(config.label)) return;
      label = config.label;
    },
  );
</script>

<div class="layout">
  <TextInput
    value={label.label}
    placeholder={i18n.f("tile_label_placeholder")}
    oninput={(event) => {
      onChangeTileName(event.currentTarget.value);
    }}
  />

  <Tooltip title={i18n.f("toggle_label_tooltip")}>
    {#snippet trigger({ props })}
      <Button {...props} onclick={onToggleEnabled} size="icon">
        {#if label.enabled}
          <SolarEyeBold width="1.5rem" height="1.5rem" />
        {:else}
          <SolarEyeClosedBold width="1.5rem" height="1.5rem" />
        {/if}
      </Button>
    {/snippet}
  </Tooltip>

  <TileNameSettingsPopover
    {fonts}
    font={label.font}
    {onChangeFont}
    fontSize={label.font_size}
    {onChangeFontSize}
    bold={label.bold}
    {onToggleBold}
    italic={label.italic}
    {onToggleItalic}
    underline={label.underline}
    {onToggleUnderline}
    outline={label.outline}
    {onToggleOutline}
    align={label.align}
    {onChangeAlign}
    color={label.color}
    {onChangeColor}
    outlineColor={label.outline_color}
    {onChangeOutlineColor}
  />
</div>

<style>
  .layout {
    display: flex;
    gap: 0.5rem;
  }
</style>
