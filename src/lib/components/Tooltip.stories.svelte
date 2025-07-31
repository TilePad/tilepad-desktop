<script lang="ts" module>
  import type { ComponentProps } from "svelte";

  import { Tooltip as BitsTooltip } from "bits-ui";
  import { defineMeta } from "@storybook/addon-svelte-csf";

  import Tooltip from "./Tooltip.svelte";

  const { Story } = defineMeta({
    title: "Custom/Tooltip",
    component: Tooltip,
    render: template,
    tags: ["autodocs"],
    argTypes: {},
    decorators: [],
  });
</script>

{#snippet template(args: ComponentProps<typeof Tooltip>)}
  <BitsTooltip.Provider>
    <Tooltip {...args} />
  </BitsTooltip.Provider>
{/snippet}

{#snippet trigger({ props }: { props: Record<string, unknown> })}
  <p style="display: inline;" {...props}>Test Tooltip Target</p>
{/snippet}

<Story name="Default" args={{ title: "Example Tooltip", trigger }} />
