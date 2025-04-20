<script lang="ts">
  import type { Snippet } from "svelte";

  import { range } from "$lib/api/utils/svelte.svelte";

  type Props = {
    rows: number;
    columns: number;
    tile: Snippet<[number, number]>;
  };

  const { rows, columns, tile }: Props = $props();

  let container: HTMLDivElement | undefined = $state();

  let containerWidth = $state(0);
  let containerHeight = $state(0);

  const gap = 10;

  const desiredWidth = 120;

  const tileWidth = $derived(
    Math.min(
      (containerWidth - gap * (columns - 1)) / columns,
      (containerHeight - gap * (rows - 1)) / rows,
    ),
  );

  const sizeAdjust = $derived.by(() => {
    const ratio = (tileWidth - desiredWidth) / desiredWidth;
    return 1 - Math.max(0.0, -ratio);
  });
</script>

<div
  class="grid"
  style="--font-size-adjustment: {sizeAdjust};"
  bind:this={container}
  bind:clientWidth={containerWidth}
  bind:clientHeight={containerHeight}
>
  {#each range(0, rows) as row}
    <div class="row">
      {#each range(0, columns) as column}
        <div class="tile" style="width: {tileWidth}px; height: {tileWidth}px;">
          {@render tile(row, column)}
        </div>
      {/each}
    </div>
  {/each}
</div>

<style>
  .grid {
    display: flex;
    flex-flow: column;

    width: 100%;
    height: 100%;
    gap: 10px;
  }

  .row {
    display: flex;
    flex-flow: row;
    justify-content: center;

    width: 100%;
    gap: 10px;
  }
</style>
