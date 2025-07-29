<script module lang="ts">
  import type { ComponentProps } from "svelte";

  import { Tooltip } from "bits-ui";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import LanguageSelector from "./LanguageSelector.svelte";

  const { Story } = defineMeta({
    title: "I18n/LanguageSelector",
    component: LanguageSelector,
    render: template,
    tags: ["autodocs"],
    argTypes: {},
  });
</script>

{#snippet template(args: ComponentProps<typeof LanguageSelector>)}
  <Tooltip.Provider>
    <LanguageSelector {...args} />
  </Tooltip.Provider>
{/snippet}

<Story
  name="Default"
  args={{
    value: "en",
    onChangeValue: () => {},
  }}
/>
