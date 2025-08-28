<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import IconPackCard from "./IconPackCard.svelte";

  const { Story } = defineMeta({
    title: "Icons/IconPackCard",
    component: IconPackCard,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    id: "00000000-0000-0000-0000-000000000000",
    version: "0.0.0",
    name: "Example Icon pack",
    description: "Example icon pack description for the example icon pack",
    authors: ["Example User", "Example User 1"],
    onUninstall: () => {},
  }}
/>
