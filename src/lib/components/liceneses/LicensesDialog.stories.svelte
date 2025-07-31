<script module>
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import LicensesDialog from "./LicensesDialog.svelte";

  const { Story } = defineMeta({
    title: "Licenses/LicensesDialog",
    component: LicensesDialog,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

<Story name="Default" />
