<script lang="ts" module>
  import type { ComponentProps } from "svelte";

  import { defineMeta } from "@storybook/addon-svelte-csf";
  import { LabelAlign, TileIconType } from "$lib/api/types/tiles";

  import TileGrid from "./TileGrid.svelte";
  import TileDraggingProvider from "./TileDraggingProvider.svelte";

  const { Story } = defineMeta({
    title: "Tiles/TileGrid",
    component: TileGrid,
    tags: ["autodocs"],
    argTypes: {},
    render: template,
  });
</script>

{#snippet template(args: ComponentProps<typeof TileGrid>)}
  <TileDraggingProvider onMoveTile={() => {}} onPlaceTile={() => {}}>
    <div style="width: 900px; height: 900px">
      <TileGrid {...args} onClickTile={() => {}} />
    </div>
  </TileDraggingProvider>
{/snippet}

<Story
  name="Default"
  args={{
    tiles: [],
    rows: 5,
    columns: 8,
  }}
/>

<Story
  name="Equal Sized"
  args={{
    tiles: [],
    rows: 5,
    columns: 5,
  }}
/>

<Story
  name="Mini pad"
  args={{
    tiles: [],
    rows: 2,
    columns: 3,
  }}
/>

<Story
  name="Wider"
  args={{
    tiles: [],
    rows: 3,
    columns: 10,
  }}
/>

<Story
  name="Taller"
  args={{
    tiles: [],
    rows: 10,
    columns: 3,
  }}
/>

<Story
  name="Populated"
  args={{
    tiles: [
      {
        id: "00000000-0000-0000-0000-000000000000",
        config: {
          icon: {
            type: TileIconType.None,
          },
          icon_options: {
            padding: 0,
            background_color: "#000000",
            border_color: "#ffffff",
          },
          label: {
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
          },
          user_flags: {
            icon: false,
            label: false,
          },
        },
        properties: {},
        folder_id: "00000000-0000-0000-0000-000000000000",
        plugin_id: "00000000-0000-0000-0000-000000000000",
        action_id: "test",
        position: {
          row: 0,
          column: 0,
          row_span: 1,
          column_span: 1,
        },
      },
    ],
    rows: 5,
    columns: 8,
  }}
/>
