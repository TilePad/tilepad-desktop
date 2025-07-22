<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import NumberInput from "./NumberInput.svelte";

  const { Story } = defineMeta({
    title: "Custom/NumberInput",
    component: NumberInput,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}} />
