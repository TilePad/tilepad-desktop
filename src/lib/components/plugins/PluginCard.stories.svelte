<script module>
  import { PluginTaskState } from "$lib/api/types/plugin";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import PluginCard from "./PluginCard.svelte";

  const { Story } = defineMeta({
    title: "Plugins/PluginCard",
    component: PluginCard,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

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
