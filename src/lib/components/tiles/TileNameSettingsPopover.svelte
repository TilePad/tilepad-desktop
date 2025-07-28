<script lang="ts">
  import { t } from "svelte-i18n";
  import { LabelAlign } from "$lib/api/types/tiles";
  import ColorPicker from "svelte-awesome-color-picker";
  import SolarTextBoldDuotone from "~icons/solar/text-bold-duotone";
  import SolarAlignTopBoldDuotone from "~icons/solar/align-top-bold-duotone";
  import SolarTextBoldBoldDuotone from "~icons/solar/text-bold-bold-duotone";
  import SolarTextItalicBoldDuotone from "~icons/solar/text-italic-bold-duotone";
  import SolarTextBoldSquareOutline from "~icons/solar/text-bold-square-outline";
  import SolarAlignBottomBoldDuotone from "~icons/solar/align-bottom-bold-duotone";
  import SolarTextUnderlineBoldDuotone from "~icons/solar/text-underline-bold-duotone";
  import SolarAlignVerticalSpacingBoldDuotone from "~icons/solar/align-vertical-spacing-bold-duotone";

  import Tooltip from "../Tooltip.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import ToggleButton from "../input/ToggleButton.svelte";
  import FontSelector from "../fonts/FontSelector.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = {
    font: string;
    onChangeFont: (font: string) => void;
    //
    fontSize: number;
    onChangeFontSize: (fontSize: number) => void;
    //
    bold: boolean;
    onToggleBold: VoidFunction;
    //
    italic: boolean;
    onToggleItalic: VoidFunction;
    //
    underline: boolean;
    onToggleUnderline: VoidFunction;
    //
    outline: boolean;
    onToggleOutline: VoidFunction;
    //
    align: LabelAlign;
    onChangeAlign: (align: LabelAlign) => void;
    //
    color: string;
    onChangeColor: (color: string) => void;
    //
    outlineColor: string;
    onChangeOutlineColor: (color: string) => void;
  };

  const {
    font,
    onChangeFont,
    //
    fontSize,
    onChangeFontSize,
    //
    bold,
    onToggleBold,
    //
    italic,
    onToggleItalic,
    //
    underline,
    onToggleUnderline,
    //
    outline,
    onToggleOutline,
    //
    align,
    onChangeAlign,
    //
    color,
    onChangeColor,
    //
    outlineColor,
    onChangeOutlineColor,
  }: Props = $props();
</script>

<Tooltip title={$t("label_options_tooltip")}>
  {#snippet trigger({ props })}
    <PopoverButton triggerProps={{ ...props }} size="icon">
      <SolarTextBoldDuotone width="1.5rem" height="1.5rem" />

      {#snippet content()}
        <FontSelector value={font} onChangeValue={onChangeFont} />

        <div>
          {$t("font_size")}
          <NumberInput
            value={fontSize}
            oninput={(event) => {
              const value = event.currentTarget.valueAsNumber;
              onChangeFontSize(value);
            }}
          />
        </div>

        <div class="toggles">
          <div class="toggle-button-list">
            <ToggleButton active={bold} onclick={onToggleBold}>
              <SolarTextBoldBoldDuotone />
            </ToggleButton>
            <ToggleButton active={italic} onclick={onToggleItalic}>
              <SolarTextItalicBoldDuotone />
            </ToggleButton>
            <ToggleButton active={underline} onclick={onToggleUnderline}>
              <SolarTextUnderlineBoldDuotone />
            </ToggleButton>
            <ToggleButton active={outline} onclick={onToggleOutline}>
              <SolarTextBoldSquareOutline />
            </ToggleButton>
          </div>

          <div class="toggle-button-list">
            <ToggleButton
              active={align === LabelAlign.Bottom}
              onclick={() => onChangeAlign(LabelAlign.Bottom)}
            >
              <SolarAlignBottomBoldDuotone />
            </ToggleButton>
            <ToggleButton
              active={align === LabelAlign.Middle}
              onclick={() => onChangeAlign(LabelAlign.Middle)}
            >
              <SolarAlignVerticalSpacingBoldDuotone />
            </ToggleButton>
            <ToggleButton
              active={align === LabelAlign.Top}
              onclick={() => onChangeAlign(LabelAlign.Top)}
            >
              <SolarAlignTopBoldDuotone />
            </ToggleButton>
          </div>
        </div>

        <ColorPicker
          hex={color}
          onInput={(color) => {
            if (color.hex) onChangeColor(color.hex);
          }}
          position="responsive"
          label={$t("text_color")}
        />

        <ColorPicker
          hex={outlineColor}
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
</style>
