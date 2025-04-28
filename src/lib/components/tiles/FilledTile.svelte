<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import TileIcon from "./TileIcon.svelte";
  import TileLabelElm from "./TileLabelElm.svelte";

  type Props = {
    tile: TileModel;

    onClick: VoidFunction;
  };

  const { tile, onClick }: Props = $props();

  const config = $derived(tile.config);
</script>

<div
  class="tile"
  onclick={onClick}
  tabindex="0"
  role="button"
  aria-roledescription="button"
  onkeydown={() => {}}
>
  <TileIcon icon={config.icon} iconOptions={config.icon_options} />
  <TileLabelElm label={config.label} />
</div>

<style>
  .tile {
    background-color: #151318;
    position: relative;

    display: flex;
    justify-content: center;
    align-items: center;

    width: var(--tile-width);
    height: var(--tile-width);

    user-select: none;
    overflow: hidden;

    font-size: 1.5rem;
    text-align: center;
  }
</style>
