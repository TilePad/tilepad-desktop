<script lang="ts">
  import { t } from "svelte-i18n";
  import { watch, useDebounce } from "runed";
  import SolarEyeBold from "~icons/solar/eye-bold";
  import ColorPicker from "svelte-awesome-color-picker";
  import SolarEyeClosedBold from "~icons/solar/eye-closed-bold";
  import { createUpdateTileLabelMutation } from "$lib/api/tiles";
  import SolarTextBoldDuotone from "~icons/solar/text-bold-duotone";
  import SolarAlignTopBoldDuotone from "~icons/solar/align-top-bold-duotone";
  import SolarTextBoldBoldDuotone from "~icons/solar/text-bold-bold-duotone";
  import SolarTextItalicBoldDuotone from "~icons/solar/text-italic-bold-duotone";
  import SolarTextBoldSquareOutline from "~icons/solar/text-bold-square-outline";
  import SolarAlignBottomBoldDuotone from "~icons/solar/align-bottom-bold-duotone";
  import SolarTextUnderlineBoldDuotone from "~icons/solar/text-underline-bold-duotone";
  import SolarAlignVerticalSpacingBoldDuotone from "~icons/solar/align-vertical-spacing-bold-duotone";
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
  import NumberInput from "../input/NumberInput.svelte";
  import ToggleButton from "../input/ToggleButton.svelte";
  import FontSelector from "../fonts/FontSelector.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";
  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  const updateTileLabel = createUpdateTileLabelMutation();

  // Last persisted update
  let lastUpdate: TileLabel = $state(config.label);
  let label = $state(config.label);

  const updateLabel = useDebounce((label: TileLabel) => {
    lastUpdate = label;

    $updateTileLabel.mutate({
      tileId,
      label,
      kind: UpdateKind.User,
    });
  }, 150);

  const onChangeTileName = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    label = { ...label, label: value };
    updateLabel(label);
  };

  const onChangeFontSize = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    label = { ...label, font_size: Number(value) };
    updateLabel(label);
  };

  const onChangeColor = (value: string) => {
    label = { ...label, color: value };
    updateLabel(label);
  };

  const onChangeOutlineColor = (value: string) => {
    label = { ...label, outline_color: value };
    updateLabel(label);
  };

  const onToggleEnabled = () => {
    label = { ...label, enabled: !label.enabled };
    updateLabel(label);
  };

  const onToggleBold = () => {
    label = { ...label, bold: !label.bold };
    updateLabel(label);
  };

  const onToggleItalic = () => {
    label = { ...label, italic: !label.italic };
    updateLabel(label);
  };

  const onToggleUnderline = () => {
    label = { ...label, underline: !label.underline };
    updateLabel(label);
  };

  const onToggleOutline = () => {
    label = { ...label, outline: !label.outline };
    updateLabel(label);
  };

  const onChangeAlign = (align: LabelAlign) => {
    label = { ...label, align };
    updateLabel(label);
  };

  const onChangeFont = (font: string) => {
    label = { ...label, font };
    updateLabel(label);
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
    placeholder={$t("tile_label_placeholder")}
    oninput={onChangeTileName}
  />

  <Tooltip title={$t("toggle_label_tooltip")}>
    {#snippet trigger({ props })}
      <Button {...props} onclick={onToggleEnabled}>
        {#if label.enabled}
          <SolarEyeBold width="1.5rem" height="1.5rem" />
        {:else}
          <SolarEyeClosedBold width="1.5rem" height="1.5rem" />
        {/if}
      </Button>
    {/snippet}
  </Tooltip>

  <Tooltip title={$t("label_options_tooltip")}>
    {#snippet trigger({ props })}
      <PopoverButton triggerProps={props}>
        <SolarTextBoldDuotone width="1.5rem" height="1.5rem" />

        {#snippet content()}
          <FontSelector value={label.font} onChangeValue={onChangeFont} />

          <div>
            {$t("font_size")}
            <NumberInput value={label.font_size} oninput={onChangeFontSize} />
          </div>

          <div class="toggles">
            <div class="toggle-button-list">
              <ToggleButton active={label.bold} onclick={onToggleBold}>
                <SolarTextBoldBoldDuotone />
              </ToggleButton>
              <ToggleButton active={label.italic} onclick={onToggleItalic}>
                <SolarTextItalicBoldDuotone />
              </ToggleButton>
              <ToggleButton
                active={label.underline}
                onclick={onToggleUnderline}
              >
                <SolarTextUnderlineBoldDuotone />
              </ToggleButton>
              <ToggleButton active={label.outline} onclick={onToggleOutline}>
                <SolarTextBoldSquareOutline />
              </ToggleButton>
            </div>

            <div class="toggle-button-list">
              <ToggleButton
                active={label.align === LabelAlign.Bottom}
                onclick={() => onChangeAlign(LabelAlign.Bottom)}
              >
                <SolarAlignBottomBoldDuotone />
              </ToggleButton>
              <ToggleButton
                active={label.align === LabelAlign.Middle}
                onclick={() => onChangeAlign(LabelAlign.Middle)}
              >
                <SolarAlignVerticalSpacingBoldDuotone />
              </ToggleButton>
              <ToggleButton
                active={label.align === LabelAlign.Top}
                onclick={() => onChangeAlign(LabelAlign.Top)}
              >
                <SolarAlignTopBoldDuotone />
              </ToggleButton>
            </div>
          </div>

          <ColorPicker
            hex={label.color}
            onInput={(color) => {
              if (color.hex) onChangeColor(color.hex);
            }}
            position="responsive"
            label={$t("text_color")}
          />
          <ColorPicker
            hex={label.outline_color}
            onInput={(color) => {
              if (color.hex) onChangeOutlineColor(color.hex);
            }}
            position="responsive"
            label={$t("text_outline_color")}
          />
        {/snippet}
      </PopoverButton>
    {/snippet}
  </Tooltip>
</div>

<style>
  .toggle-button-list {
    display: flex;
    flex-flow: row;
    gap: 0.5rem;
  }

  .toggles {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
  }

  .layout {
    display: flex;
    gap: 0.5rem;
  }
</style>
