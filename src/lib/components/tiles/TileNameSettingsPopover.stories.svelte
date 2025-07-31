<script lang="ts" module>
  import type { ComponentProps } from "svelte";

  import { Tooltip } from "bits-ui";
  import { LabelAlign } from "$lib/api/types/tiles";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import TileNameSettingsPopover from "./TileNameSettingsPopover.svelte";

  const { Story } = defineMeta({
    title: "Tiles/TileNameSettingsPopover",
    component: TileNameSettingsPopover,
    render: template,
    tags: ["autodocs"],
    argTypes: {},
    decorators: [],
  });
</script>

{#snippet template(args: ComponentProps<typeof TileNameSettingsPopover>)}
  <Tooltip.Provider>
    <TileNameSettingsPopover {...args} />
  </Tooltip.Provider>
{/snippet}

<Story
  name="Default"
  args={{
    fonts: [],
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
