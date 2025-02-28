<script lang="ts">
  import EmptyTile from "./EmptyTile.svelte";

  type Props = {
    row: number;
    column: number;
  };

  const { row, column }: Props = $props();

  let tile = null;
</script>

{#if tile !== null}{:else}
  <EmptyTile {row} {column} />
{/if}
