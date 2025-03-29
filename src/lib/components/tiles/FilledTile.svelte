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
  <TileIcon icon={config.icon} />
  <TileLabelElm label={config.label} />
</div>

<style>
  .tile {
    position: relative;
    background-color: #151318;
    border: 2px solid #715c8f;
    border-radius: 5px;
    display: flex;
    justify-content: center;
    align-items: center;
    font-weight: bold;
    text-align: center;
    cursor: pointer;
    width: 100%;
    height: 100%;
    color: #ccc;
    font-size: 1.5rem;
    user-select: none;
    overflow: hidden;
  }
</style>
