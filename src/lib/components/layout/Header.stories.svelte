<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Header from "./Header.svelte";

  const { Story } = defineMeta({
    title: "Custom/Header",
    component: Header,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}} />
