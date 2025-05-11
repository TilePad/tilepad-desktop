<script lang="ts">
  import type { TileLabel } from "$lib/api/types/tiles";

  type Props = {
    label: TileLabel;
  };

  const { label }: Props = $props();
</script>

{#if label.enabled && label.label.length > 0}
  <div class="label-container" data-align={label.align}>
    <p
      class="label"
      style="font-size: calc({label.font_size}pt * var(--tile-size-adjustment)); font-family: {label.font}, 'Roboto'; color: {label.color}; --outline-color: {label.outline_color};"
      data-align={label.align}
      class:label--bold={label.bold}
      class:label--italic={label.italic}
      class:label--underline={label.underline}
      class:label--outline={label.outline}
    >
      {label.label}
    </p>
  </div>
{/if}

<style>
  .label-container {
    position: absolute;
    left: 0;
    top: 0;
    display: flex;
    width: 100%;
    height: 100%;
    padding: 0.25rem;
    justify-content: center;
  }

  .label-container[data-align="Bottom"] {
    align-items: flex-end;
  }

  .label-container[data-align="Middle"] {
    align-items: center;
  }
  .label-container[data-align="Top"] {
    align-items: flex-start;
  }

  .label {
    text-align: center;
    font-weight: normal;
  }

  .label--bold {
    font-weight: bold;
  }

  .label--italic {
    font-style: italic;
  }

  .label--underline {
    text-decoration: underline;
  }

  .label--outline {
    text-shadow:
      -1px -1px 0 var(--outline-color),
      1px -1px 0 var(--outline-color),
      -1px 1px 0 var(--outline-color),
      1px 1px 0 var(--outline-color);
  }
</style>
