<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  import { Popover, type PopoverCloseProps } from "bits-ui";

  import Button from "../input/Button.svelte";

  type Props = {
    children?: Snippet;

    closeProps?: Omit<PopoverCloseProps, "asChild">;
  } & HTMLButtonAttributes;

  const { children, closeProps, ...buttonProps }: Props = $props();
</script>

<Popover.Close {...closeProps}>
  {#snippet child({ props })}
    <Button {...props} {...buttonProps} type="button">
      {@render children?.()}
    </Button>
  {/snippet}
</Popover.Close>
