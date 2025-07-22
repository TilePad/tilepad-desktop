<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Button from "./Button.svelte";

  const { Story } = defineMeta({
    title: "Custom/Button",
    component: Button,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}}>Button</Story>
