<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import SolarTextBoldBoldDuotone from "~icons/solar/text-bold-bold-duotone";
  import SolarTextItalicBoldDuotone from "~icons/solar/text-italic-bold-duotone";

  import ToggleButton from "./ToggleButton.svelte";

  const { Story } = defineMeta({
    title: "Custom/ToggleButton",
    component: ToggleButton,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" args={{ active: false }}>
  <SolarTextBoldBoldDuotone />
</Story>

<Story name="Active" args={{ active: true }}>
  <SolarTextItalicBoldDuotone />
</Story>
