<script lang="ts" module>
  import type { ComponentProps } from "svelte";

  import { defineMeta } from "@storybook/addon-svelte-csf";

  import ActionsSidebar from "./ActionsSidebar.svelte";
  import TileDraggingProvider from "../tiles/TileDraggingProvider.svelte";

  const { Story } = defineMeta({
    title: "Action/ActionsSidebar",
    component: ActionsSidebar,
    render: template,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

{#snippet template(args: ComponentProps<typeof ActionsSidebar>)}
  <TileDraggingProvider
    onMoveTile={() => {}}
    onPlaceTile={() => {}}
    onDeleteTile={() => {}}
  >
    <ActionsSidebar {...args} />
  </TileDraggingProvider>
{/snippet}

<Story
  name="Default"
  args={{
    actions: [
      {
        pluginId: "00000000-0000-0000-0000-000000000000",
        icon: `data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23fff' d='M10 2h4c3.771 0 5.657 0 6.828 1.172S22 6.229 22 10v1c0 .552 0 1.55-.006 2H2.007C2 12.55 2 11.552 2 11v-1c0-3.771 0-5.657 1.172-6.828S6.229 2 10 2' opacity='0.5'/%3E%3Cpath fill='%23fff' d='M7.985 17.5c-2.84 0-4.26 0-5.141-.879C2.272 16.052 2.07 15.258 2 14v-1h20v1c-.07 1.258-.272 2.052-.844 2.621c-.882.879-2.301.879-5.14.879h-3.263v4h3.262c.416 0 .753.336.753.75s-.337.75-.753.75h-8.03a.75.75 0 0 1-.753-.75c0-.414.337-.75.753-.75h3.262v-4z'/%3E%3C/svg%3E`,
        label: "Example Category",
        actions: [
          {
            pluginId: "00000000-0000-0000-0000-000000000000",
            actionId: "test",
            icon: `data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24'%3E%3Cpath fill='%23fff' d='M10 2h4c3.771 0 5.657 0 6.828 1.172S22 6.229 22 10v1c0 .552 0 1.55-.006 2H2.007C2 12.55 2 11.552 2 11v-1c0-3.771 0-5.657 1.172-6.828S6.229 2 10 2' opacity='0.5'/%3E%3Cpath fill='%23fff' d='M7.985 17.5c-2.84 0-4.26 0-5.141-.879C2.272 16.052 2.07 15.258 2 14v-1h20v1c-.07 1.258-.272 2.052-.844 2.621c-.882.879-2.301.879-5.14.879h-3.263v4h3.262c.416 0 .753.336.753.75s-.337.75-.753.75h-8.03a.75.75 0 0 1-.753-.75c0-.414.337-.75.753-.75h3.262v-4z'/%3E%3C/svg%3E`,
            label: "Test Action",
          },
        ],
      },
    ],
    actionsLoading: false,
    actionsError: null,
  }}
/>

<Story
  name="Loading"
  args={{
    actions: [],
    actionsLoading: true,
    actionsError: null,
  }}
/>

<Story
  name="Error"
  args={{
    actions: [],
    actionsLoading: false,
    actionsError: new Error("some internal error"),
  }}
/>
