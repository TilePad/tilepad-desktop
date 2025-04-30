<script lang="ts">
  import { watch } from "runed";
  import { t } from "svelte-i18n";
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

  let open = $state(false);
  let pickPack: boolean = $state(false);

  watch(
    () => open,
    (open) => {
      if (!open) pickPack = false;
    },
  );
</script>

<Tooltip title={$t("icon_options")}>
  {#snippet trigger({ props })}
    <PopoverButton
      triggerProps={props}
      rootProps={{ open, onOpenChange: (value) => (open = value) }}
    >
      {#snippet button({ props })}
        <Button {...props}>
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
            {$t("padding_px")}
            <NumberInput
              value={iconOptions.padding}
              oninput={(event) =>
                onChangePadding(event.currentTarget.valueAsNumber)}
            />
          </div>

          <ColorPicker
            hex={iconOptions.background_color}
            on:input={(event) => {
              if (event.detail.hex) onChangeBackgroundColor(event.detail.hex);
            }}
            position="responsive"
            label={$t("background_color")}
          />

          <ColorPicker
            hex={iconOptions.border_color}
            on:input={(event) => {
              if (event.detail.hex) onChangeBorderColor(event.detail.hex);
            }}
            position="responsive"
            label={$t("border_color")}
          />

          <div class="content">
            <Button style="width: 100%;" onclick={() => (pickPack = true)}>
              {$t("icon_pack_icon")}
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
              {$t("reset_default")}
            </Button>
          </div>
        {/if}
      {/snippet}
    </PopoverButton>
  {/snippet}
</Tooltip>
