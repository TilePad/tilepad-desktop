<script lang="ts">
  import { watch } from "runed";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import ColorPicker from "svelte-awesome-color-picker";
  import SolarGalleryEditBoldDuotone from "~icons/solar/gallery-edit-bold-duotone";
  import {
    TileIconType,
    type TileIcon,
    type TileIconOptions,
  } from "$lib/api/types/tiles";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
  import IconFilePicker from "./IconFilePicker.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import IconPackSelector from "./IconPackSelector.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = {
    iconOptions: TileIconOptions;
    onChangePadding: (padding: number) => void;
    onChangeBackgroundColor: (color: string) => void;
    onChangeBorderColor: (color: string) => void;

    onSelectIcon: (icon: TileIcon) => void;
    onResetDefault: VoidFunction;
  };

  const {
    iconOptions,
    onChangePadding,
    onChangeBackgroundColor,
    onChangeBorderColor,
    onSelectIcon,
    onResetDefault,
  }: Props = $props();

  const i18n = i18nContext.get();

  let open = $state(false);
  let pickPack: boolean = $state(false);

  watch(
    () => open,
    (open) => {
      if (!open) pickPack = false;
    },
  );
</script>

<Tooltip title={i18n.f("icon_options")}>
  {#snippet trigger({ props })}
    <PopoverButton
      triggerProps={props}
      rootProps={{ open, onOpenChange: (value) => (open = value) }}
    >
      {#snippet button({ props })}
        <Button size="icon" {...props}>
          <SolarGalleryEditBoldDuotone width="1.65rem" height="1.65rem" />
        </Button>
      {/snippet}

      {#snippet content()}
        {#if pickPack}
          <IconPackSelector
            onClickIcon={(packId, icon) => {
              onSelectIcon({
                type: TileIconType.IconPack,
                pack_id: packId,
                path: icon.path,
              });
              open = false;
            }}
          />
        {:else}
          <div>
            {i18n.f("padding_px")}
            <NumberInput
              value={iconOptions.padding}
              oninput={(event) =>
                onChangePadding(event.currentTarget.valueAsNumber)}
            />
          </div>

          <ColorPicker
            hex={iconOptions.background_color}
            onInput={(color) => {
              if (color.hex) onChangeBackgroundColor(color.hex);
            }}
            position="responsive"
            label={i18n.f("background_color")}
          />

          <ColorPicker
            hex={iconOptions.border_color}
            onInput={(color) => {
              if (color.hex) onChangeBorderColor(color.hex);
            }}
            position="responsive"
            label={i18n.f("border_color")}
          />

          <div class="content">
            <Button style="width: 100%;" onclick={() => (pickPack = true)}>
              {i18n.f("icon_pack_icon")}
            </Button>

            <IconFilePicker
              onSelectIcon={(icon) => {
                onSelectIcon(icon);
                open = false;
              }}
            />
            <Button
              type="button"
              onclick={() => {
                onResetDefault();
                open = false;
              }}
              style="width: 100%"
            >
              {i18n.f("reset_default")}
            </Button>
          </div>
        {/if}
      {/snippet}
    </PopoverButton>
  {/snippet}
</Tooltip>
