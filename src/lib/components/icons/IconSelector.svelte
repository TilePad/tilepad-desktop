<script lang="ts">
  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";
  import SolarGalleryEditBoldDuotone from "~icons/solar/gallery-edit-bold-duotone";

  import Button from "../input/Button.svelte";
  import IconFilePicker from "./IconFilePicker.svelte";
  import IconPackSelector from "./IconPackSelector.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = {
    onSelectIcon: (icon: TileIcon) => void;
    onResetDefault: VoidFunction;
  };

  const { onSelectIcon, onResetDefault }: Props = $props();

  let open = $state(false);
</script>

<PopoverButton rootProps={{ open, onOpenChange: (value) => (open = value) }}>
  {#snippet button({ props })}
    <Button {...props}
      ><SolarGalleryEditBoldDuotone width="1.65rem" height="1.65rem" /></Button
    >
  {/snippet}

  {#snippet content()}
    <div class="content">
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
  {/snippet}
</PopoverButton>
