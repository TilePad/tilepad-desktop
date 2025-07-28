<script lang="ts" module>
  import { LabelAlign } from "$lib/api/types/tiles";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import TileNameSettingsPopover from "./TileNameSettingsPopover.svelte";

  const { Story } = defineMeta({
    title: "Tiles/TileNameSettingsPopover",
    component: TileNameSettingsPopover,
    tags: ["autodocs"],
    argTypes: {},
    decorators: [],
  });
</script>

<Story
  name="Default"
  args={{
    font: "Arial",
    onChangeFont: () => {},

    fontSize: 12,
    onChangeFontSize: () => {},

    bold: false,
    onToggleBold: () => {},

    italic: false,
    onToggleItalic: () => {},

    underline: false,
    onToggleUnderline: () => {},

    outline: false,
    onToggleOutline: () => {},

    align: LabelAlign.Bottom,
    onChangeAlign: () => {},

    color: "#ffffff",
    onChangeColor: () => {},

    outlineColor: "#ffffff",
    onChangeOutlineColor: () => {},
  }}
/>
