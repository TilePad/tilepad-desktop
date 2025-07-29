<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import FontSelector from "./FontSelector.svelte";

  const { Story } = defineMeta({
    title: "Fonts/FontSelector",
    component: FontSelector,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    fonts: ["Roboto", "Arial"],
    value: "Roboto",
    onChangeValue: () => {},
  }}
/>
