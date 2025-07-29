<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import IconPackCategory from "./IconPackCategory.svelte";

  const { Story } = defineMeta({
    title: "Icons/IconPackCategory",
    component: IconPackCategory,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    name: "Example Category",
    icons: [
      {
        name: "Example Icon",
        path: "#",
        src: "",
      },
    ],
    onClickIcon: () => {},
  }}
/>
