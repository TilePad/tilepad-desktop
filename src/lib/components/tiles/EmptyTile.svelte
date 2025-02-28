<!-- Empty tile that can be used to add a new tile -->
<script lang="ts">
  import ActionChooser from "../actions/ActionChooser.svelte";

  type Props = {
    row: number;
    column: number;
  };

  const { row, column }: Props = $props();
</script>

<ActionChooser>
  {#snippet button({ props })}
    <button {...props} class="tile"> + </button>
  {/snippet}
</ActionChooser>

<style>
  .tile {
    background-color: #242129;
    border: 2px solid #36313d;
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
  }
</style>
