<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import CreatorSection from "./CreatorSection.svelte";

  const { Story } = defineMeta({
    title: "Custom/CreatorSection",
    component: CreatorSection,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{ version: "1.0.0" }} />
