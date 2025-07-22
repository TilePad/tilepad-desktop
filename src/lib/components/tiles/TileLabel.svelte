<script lang="ts">
  import type { LabelAlign } from "$lib/api/types/tiles";

  type Props = {
    enabled: boolean;
    label: string;
    align: LabelAlign;
    font: string;
    font_size: number;
    bold: boolean;
    italic: boolean;
    underline: boolean;
    outline: boolean;
    color: string;
    outline_color: string;
  };

  const {
    enabled,
    label,
    align,
    font,
    font_size,
    bold,
    italic,
    underline,
    outline,
    color,
    outline_color,
  }: Props = $props();
</script>

{#if enabled && label.length > 0}
  <div class="label-container" data-align={align}>
    <p
      class="label"
      style="font-size: calc({font_size}pt * var(--tile-size-adjustment)); font-family: {font}, 'Roboto'; color: {color}; --outline-color: {outline_color};"
      data-align={align}
      class:label--bold={bold}
      class:label--italic={italic}
      class:label--underline={underline}
      class:label--outline={outline}
    >
      {label}
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
    margin: calc(0.5rem * var(--tile-size-adjustment));
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
