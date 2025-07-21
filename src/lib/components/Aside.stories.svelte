<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Aside from "./Aside.svelte";

  const { Story } = defineMeta({
    title: "Custom/Aside",
    component: Aside,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Success"
  args={{
    severity: "success",
    title: "Success",
  }}
>
  This is a success message
</Story>

<Story
  name="Info"
  args={{
    severity: "info",
    title: "Info",
  }}
>
  This is a info message
</Story>

<Story
  name="Warning"
  args={{
    severity: "warning",
    title: "Warning",
  }}
>
  This is a warning message
</Story>

<Story
  name="Error"
  args={{
    severity: "error",
    title: "Error",
  }}
>
  This is a error message
</Story>

<Story
  name="Tip"
  args={{
    severity: "tip",
    title: "Tip",
  }}
>
  This is a tip message
</Story>
