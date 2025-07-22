<script lang="ts" module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import {
    LabelAlign,
    type TileLabel as TileLabelType,
  } from "$lib/api/types/tiles";

  import TileLabel from "./TileLabel.svelte";
  import TestTileContainer from "./TestTileContainer.svelte";

  const { Story } = defineMeta({
    title: "Custom/TileLabel",
    component: TileLabel,
    tags: ["autodocs"],
    argTypes: {},
    decorators: [() => TestTileContainer as never],
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
