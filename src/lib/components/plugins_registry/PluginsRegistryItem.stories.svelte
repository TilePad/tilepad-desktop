<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import PluginsRegistryItem from "./PluginsRegistryItem.svelte";

  const { Story } = defineMeta({
    title: "PluginsRegistry/PluginsRegistryItem",
    component: PluginsRegistryItem,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    name: "Example Plugin",
    authors: ["Author 1", "Author 2"],
    description:
      "An example plugin description from a plugin in the repository",
    repo: "example/example",
    selected: false,
    installed: false,
  }}
/>

<Story
  name="Selected"
  args={{
    name: "Example Plugin",
    authors: ["Author 1", "Author 2"],
    description:
      "An example plugin description from a plugin in the repository",
    repo: "example/example",
    selected: true,
    installed: false,
  }}
/>

<Story
  name="Installed"
  args={{
    name: "Example Plugin",
    authors: ["Author 1", "Author 2"],
    description:
      "An example plugin description from a plugin in the repository",
    repo: "example/example",
    selected: false,
    installed: true,
  }}
/>

<Story
  name="Selected & Installed"
  args={{
    name: "Example Plugin",
    authors: ["Author 1", "Author 2"],
    description:
      "An example plugin description from a plugin in the repository",
    repo: "example/example",
    selected: true,
    installed: true,
  }}
/>
