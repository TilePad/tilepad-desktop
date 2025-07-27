<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import IconsRegistryItem from "./IconsRegistryItem.svelte";

  const { Story } = defineMeta({
    title: "IconsRegistry/IconsRegistryItem",
    component: IconsRegistryItem,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    name: "Example Icon Pack",
    authors: ["Author 1", "Author 2"],
    description:
      "An example icon pack description from a icon pack in the repository",
    repo: "example/example",
    selected: false,
    installed: false,
  }}
/>

<Story
  name="Selected"
  args={{
    name: "Example Icon Pack",
    authors: ["Author 1", "Author 2"],
    description:
      "An example icon pack description from a icon pack in the repository",
    repo: "example/example",
    selected: true,
    installed: false,
  }}
/>

<Story
  name="Installed"
  args={{
    name: "Example Icon Pack",
    authors: ["Author 1", "Author 2"],
    description:
      "An example icon pack description from a icon pack in the repository",
    repo: "example/example",
    selected: false,
    installed: true,
  }}
/>

<Story
  name="Selected & Installed"
  args={{
    name: "Example Icon Pack",
    authors: ["Author 1", "Author 2"],
    description:
      "An example icon pack description from a icon pack in the repository",
    repo: "example/example",
    selected: true,
    installed: true,
  }}
/>
