<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Select from "./Select.svelte";

  const { Story } = defineMeta({
    title: "Input/Select",
    component: Select,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    value: null,
    options: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Option",
      },
    ],
    onChangeValue: () => {},
  }}
/>

<Story
  name="Selected"
  args={{
    value: "00000000-0000-0000-0000-000000000000",
    options: [
      {
        value: "00000000-0000-0000-0000-000000000000",
        name: "Default Option",
      },
    ],
    onChangeValue: () => {},
  }}
/>

<Story
  name="No Options"
  args={{
    value: null,
    options: [],
    onChangeValue: () => {},
  }}
/>
