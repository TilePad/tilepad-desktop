<script lang="ts" module>
  import type { ComponentProps } from "svelte";

  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    LabelAlign,
    type TileLabel as TileLabelType,
  } from "$lib/api/types/tiles";

  import TileLabel from "./TileLabel.svelte";

  const { Story } = defineMeta({
    title: "Tiles/TileLabel",
    component: TileLabel,
    tags: ["autodocs"],
    argTypes: {},
    render: template,
  });

  const defaultLabel: TileLabelType = {
    enabled: true,
    label: "Example",
    align: LabelAlign.Bottom,
    font: "Roboto",
    font_size: 16,
    bold: false,
    italic: false,
    underline: false,
    outline: false,
    color: "#ffffff",
    outline_color: "#000000",
  };
</script>

{#snippet template(args: ComponentProps<typeof TileLabel>)}
  <button
    style="--tile-size-adjustment: 1; --tile-width: 128px; --tile-height: 128px;"
    class="tile"
    aria-roledescription="button"
    data-drop-zone="filledTile"
  >
    <TileLabel {...args} />
  </button>

  <style>
    .tile {
      position: relative;
      border-radius: 5px;
      display: flex;
      justify-content: center;
      align-items: center;
      width: var(--tile-width);
      height: var(--tile-height);
      color: #ccc;

      border: 2px solid var(--tile-border-color);
      cursor: pointer;

      background-color: #151318;

      user-select: none;
      overflow: hidden;
    }
  </style>
{/snippet}

<Story
  name="Default"
  args={{
    ...defaultLabel,
  }}
/>

<Story
  name="Middle Align"
  args={{
    ...defaultLabel,
    align: LabelAlign.Middle,
  }}
/>

<Story
  name="Top Align"
  args={{
    ...defaultLabel,
    align: LabelAlign.Top,
  }}
/>

<Story
  name="Bold"
  args={{
    ...defaultLabel,
    bold: true,
  }}
/>

<Story
  name="Italic"
  args={{
    ...defaultLabel,
    italic: true,
  }}
/>

<Story
  name="Underline"
  args={{
    ...defaultLabel,
    underline: true,
  }}
/>

<Story
  name="Outline"
  args={{
    ...defaultLabel,
    outline: true,
    outline_color: "red",
  }}
/>
