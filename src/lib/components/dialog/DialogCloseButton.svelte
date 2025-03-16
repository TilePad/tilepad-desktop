<script module lang="ts">
  export type DialogCloseButtonProps = {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;
  } & DialogCloseProps &
    ButtonProps;
</script>

<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { Dialog, type DialogCloseProps } from "bits-ui";
  import Button, {
    type ButtonProps,
  } from "$lib/components/input/Button.svelte";

  const { button, buttonLabel, ...buttonProps }: DialogCloseButtonProps =
    $props();
</script>

<Dialog.Close {...buttonProps}>
  {#snippet child({ props })}
    {#if button}
      {@render button({ props })}
    {:else if buttonLabel}
      <Button type="button" {...props}>
        {#if buttonLabel.icon}
          <buttonLabel.icon />
        {/if}

        {buttonLabel.text}
      </Button>
    {/if}
  {/snippet}
</Dialog.Close>
