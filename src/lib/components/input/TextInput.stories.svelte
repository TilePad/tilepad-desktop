<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import TextInput from "./TextInput.svelte";

  const { Story } = defineMeta({
    title: "Input/TextInput",
    component: TextInput,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{}} />

<Story name="Placeholder" args={{ placeholder: "Enter text" }} />

<Story name="Password" args={{ type: "password", placeholder: "Password" }} />
