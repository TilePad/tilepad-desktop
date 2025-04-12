<script lang="ts">
  import { watch, useDebounce } from "runed";
  import ColorPicker from "svelte-awesome-color-picker";
  import { createUpdateTileMutation } from "$lib/api/tiles";
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

  const updateTile = createUpdateTileMutation();

  // Last persisted update
  let lastUpdate: TileLabel = $state(config.label);
  let label = $state(config.label);

  const updateLabel = useDebounce((label: TileLabel) => {
    lastUpdate = label;

    $updateTile.mutate({
      tileId,
      update: {
        config: {
          ...config,
          label,
        },
      },
    });
  }, 150);

  const updateLabelText = useDebounce((labelValue: string) => {
    lastUpdate = label;

    $updateTile.mutate({
      tileId,
      update: {
        config: {
          ...config,
          label: { ...config.label, label: labelValue },
          user_flags: {
            ...config.user_flags,
            // Label is only dirt if the user has provided a value
            label: labelValue.trim().length > 0,
          },
        },
      },
    });
  }, 150);

  const onChangeTileName = (event: Event) => {
    const value = (event.target as HTMLInputElement).value;
    label = { ...label, label: value };
    updateLabelText(value);
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

  const onChangeAlign = (align: LabelAlign) => {
    label = { ...label, align };
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
