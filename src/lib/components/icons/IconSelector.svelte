<script lang="ts">
  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";

  import IconFilePicker from "./IconFilePicker.svelte";
  import IconPackSelector from "./IconPackSelector.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";

  type Props = { onSelectIcon: (icon: TileIcon) => void };

  const { onSelectIcon }: Props = $props();
</script>

<PopoverButton>
  {#snippet button({ props })}
    <button class="btn" {...props} type="button"> + </button>
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
        }}
      />
      <IconFilePicker {onSelectIcon} />
    </div>
  {/snippet}
</PopoverButton>

<style>
  .btn {
    position: absolute;
    right: 0;
    top: 0;
    z-index: 1;
    cursor: pointer;

    width: 1.5rem;
    height: 1.5rem;

    padding: 0.35rem;
    background-color: #544d5e;
    border: none;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
  }
</style>
