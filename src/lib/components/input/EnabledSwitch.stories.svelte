<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import EnabledSwitch from "./EnabledSwitch.svelte";

  const { Story } = defineMeta({
    title: "Custom/EnabledSwitch",
    component: EnabledSwitch,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}} />
