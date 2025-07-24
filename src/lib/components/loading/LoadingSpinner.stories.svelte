<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import LoadingSpinner from "./LoadingSpinner.svelte";

  const { Story } = defineMeta({
    title: "Custom/LoadingSpinner",
    component: LoadingSpinner,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" />
