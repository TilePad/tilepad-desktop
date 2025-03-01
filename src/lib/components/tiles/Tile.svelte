<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import EmptyTile from "./EmptyTile.svelte";
  import FilledTile from "./FilledTile.svelte";

  type Props = {
    tile: TileModel | null;
    row: number;
    column: number;
  };

  const { tile, row, column }: Props = $props();
</script>

{#if tile !== null}
  <FilledTile {tile} />
{:else}
  <EmptyTile {row} {column} />
{/if}
