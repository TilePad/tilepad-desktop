<script lang="ts">
  import { watch, useDebounce } from "runed";
  import { updateTile } from "$lib/api/tiles";
  import ColorPicker from "svelte-awesome-color-picker";
  import SolarTextBoldDuotone from "~icons/solar/text-bold-duotone";
  import SolarAlignTopBoldDuotone from "~icons/solar/align-top-bold-duotone";
  import SolarTextBoldBoldDuotone from "~icons/solar/text-bold-bold-duotone";
  import SolarTextItalicBoldDuotone from "~icons/solar/text-italic-bold-duotone";
  import SolarAlignBottomBoldDuotone from "~icons/solar/align-bottom-bold-duotone";
  import SolarTextUnderlineBoldDuotone from "~icons/solar/text-underline-bold-duotone";
  import SolarAlignVerticalSpacingBoldDuotone from "~icons/solar/align-vertical-spacing-bold-duotone";
  import {
    LabelAlign,
    type TileId,
    type TileLabel,
    type TileConfig,
  } from "$lib/api/types/tiles";

  import TextInput from "../input/TextInput.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import ToggleButton from "../input/ToggleButton.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = {
    tileId: TileId;
    config: TileConfig;
  };

  const { tileId, config }: Props = $props();

  let label = $state(config.label);
  let dirty: TileId | null = $state(null);

  const updateLabel = useDebounce((label: TileLabel) => {
    updateTile(tileId, {
      config: {
        ...config,
        label,
      },
    });
  }, 150);

  const onChangeTileName = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    dirty = tileId;
    label = { ...label, label: value };
    updateLabel(label);
  };

  const onChangeFontSize = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    dirty = tileId;
    label = { ...label, font_size: Number(value) };
    updateLabel(label);
  };

  const onChangeColor = (value: string) => {
    dirty = tileId;
    label = { ...label, color: value };
    updateLabel(label);
  };

  const onToggleEnabled = () => {
    dirty = tileId;
    label = { ...label, enabled: !label.enabled };
    updateLabel(label);
  };

  const onToggleBold = () => {
    dirty = tileId;
    label = { ...label, bold: !label.bold };
    updateLabel(label);
  };

  const onToggleItalic = () => {
    dirty = tileId;
    label = { ...label, italic: !label.italic };
    updateLabel(label);
  };

  const onToggleUnderline = () => {
    dirty = tileId;
    label = { ...label, underline: !label.underline };
    updateLabel(label);
  };

  const onChangeAlign = (align: LabelAlign) => {
    dirty = tileId;
    label = { ...label, align };
    updateLabel(label);
  };

  // Only set initial site when not dirty (Prevent UI flickering from changes)
  watch(
    () => ({
      config,
      dirty,
      tileId,
    }),
    ({ config, dirty, tileId }) => {
      if (dirty === tileId) return;
      label = config.label;
    },
  );
</script>

<div class="layout">
  <TextInput value={label.label} oninput={onChangeTileName} />

  <PopoverButton>
    {#snippet children()}
      <SolarTextBoldDuotone />
    {/snippet}
    {#snippet content()}
      <button onclick={onToggleEnabled}>Enabled {label.enabled}</button>
      <div class="toggles">
        <div class="toggle-button-list">
          <ToggleButton active={label.bold} onclick={onToggleBold}>
            <SolarTextBoldBoldDuotone />
          </ToggleButton>
          <ToggleButton active={label.italic} onclick={onToggleItalic}>
            <SolarTextItalicBoldDuotone />
          </ToggleButton>
          <ToggleButton active={label.underline} onclick={onToggleUnderline}>
            <SolarTextUnderlineBoldDuotone />
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

      <div>
        <NumberInput value={label.font_size} oninput={onChangeFontSize} /> pt
      </div>

      <div class="color-picker">
        <ColorPicker
          hex={label.color}
          on:input={(event) => {
            if (event.detail.hex) onChangeColor(event.detail.hex);
          }}
          position="responsive"
        />
      </div>
    {/snippet}
  </PopoverButton>
</div>

<style>
  .toggle-button-list {
    display: flex;
    flex-flow: row;
    gap: 0.5rem;
  }

  .color-picker {
    --cp-bg-color: #2f2b35;
    --cp-border-color: #544e5e;
    --cp-text-color: white;
    --cp-input-color: #524b5c;
    --cp-button-hover-color: #777;
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
