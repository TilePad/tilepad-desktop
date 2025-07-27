<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import LinkButton from "./LinkButton.svelte";

  const { Story } = defineMeta({
    title: "Input/LinkButton",
    component: LinkButton,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}}>Link Button</Story>
