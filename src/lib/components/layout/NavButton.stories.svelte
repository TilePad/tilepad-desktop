<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";
  import SolarLayersMinimalisticBoldDuotone from "~icons/solar/layers-minimalistic-bold-duotone";

  import NavButton from "./NavButton.svelte";

  const { Story } = defineMeta({
    title: "Layout/NavButton",
    component: NavButton,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story
  name="Default"
  args={{
    icon: SolarLayersMinimalisticBoldDuotone,
    href: "#",
    title: "Example",
  }}
/>
