<script lang="ts">
  import { watch } from "runed";
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

    onSelectIcon: (icon: TileIcon) => void;
    onResetDefault: VoidFunction;
  };

  const {
    iconOptions,
    onChangePadding,
    onChangeBackgroundColor,
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

<Tooltip>
  {#snippet trigger({ props })}
    <PopoverButton
      triggerProps={props}
      rootProps={{ open, onOpenChange: (value) => (open = value) }}
    >
      {#snippet button({ props })}
        <Button {...props}
          ><SolarGalleryEditBoldDuotone
            width="1.65rem"
            height="1.65rem"
          /></Button
        >
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
            Padding (px)
            <NumberInput
              value={iconOptions.padding}
              oninput={(event) =>
                onChangePadding(event.currentTarget.valueAsNumber)}
            />
          </div>

          <div class="color-picker">
            <ColorPicker
              hex={iconOptions.background_color}
              on:input={(event) => {
                if (event.detail.hex) onChangeBackgroundColor(event.detail.hex);
              }}
              position="responsive"
              label="Background color"
            />
          </div>

          <div class="content">
            <Button style="width: 100%;" onclick={() => (pickPack = true)}>
              Icon Pack Icon
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
              Reset to default
            </Button>
          </div>
        {/if}
      {/snippet}
    </PopoverButton>
  {/snippet}

  {#snippet children()}
    Icon Options
  {/snippet}
</Tooltip>
