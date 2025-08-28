<script module lang="ts">
  import { PluginTaskState } from "$lib/api/types/plugin";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import PluginCard from "./PluginCard.svelte";
  import type { ComponentProps } from "svelte";
  import { range } from "$lib/api/utils/svelte.svelte";

  const { Story } = defineMeta({
    title: "Plugins/PluginCard",
    render: template,
    component: PluginCard,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

{#snippet template(args: ComponentProps<typeof PluginCard>)}
  <div
    style="width: 3400px; display:grid; grid-template-columns:repeat(auto-fit, minmax(min(400px, max(200px, 100%)), 1fr)); gap: var(--tp-space-4); padding: var(--tp-space-4);"
  >
    {#each range(0, 20) as index (index)}
      <PluginCard {...args} />
    {/each}
  </div>
{/snippet}

<Story
  name="Default"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,
    name: "Example Plugin",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="New Version"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,

    name: "Example Plugin",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    latestVersion: {
      version: "1.2.0",
      remotePlugin: {
        id: "",
        name: "",
        authors: [],
        description: "",
        repo: "",
      },
    },
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="Unavailable"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,

    name: "Example Plugin",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Unavailable,
    developerMode: true,
  }}
/>

<Story
  name="Developer Mode"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,

    name: "Example Plugin",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
    developerMode: true,
  }}
/>

<Story
  name="Internal"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,

    name: "Example Plugin",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: true,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="Long Name"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,
    name: "Example Plugin With A long name that spans multiple lines",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="Long Name Unbroken"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,
    name: "ExamplePluginwithalongnamethatisunbroken",
    description: "Example plugin description for example plugin",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="Long Description"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,
    name: "Example Plugin",
    description:
      "Example plugin description for example plugin, here is a long description that might overflow and needs to be handled properly so that text does not break the layout",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>

<Story
  name="Long Text"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    icon: null,
    name: "Example Plugin With A long name that spans multiple lines",
    description:
      "Example plugin description for example plugin, here is a long description that might overflow and needs to be handled properly so that text does not break the layout",
    version: "1.0.0",
    internal: false,
    authors: ["Example User", "Example User 1"],
    state: PluginTaskState.Running,
  }}
/>
