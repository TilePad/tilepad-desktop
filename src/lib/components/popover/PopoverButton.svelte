<script lang="ts">
  import type { Snippet } from "svelte";
  import type { HTMLButtonAttributes } from "svelte/elements";

  import { scale } from "svelte/transition";
  import {
    Popover,
    type PopoverRootProps,
    type PopoverContentProps,
    type PopoverTriggerProps,
  } from "bits-ui";

  import Button from "../input/Button.svelte";

  type Props = {
    button?: PopoverTriggerProps["child"];
    children?: Snippet;
    content?: Snippet;
    transparent?: boolean;

    rootProps?: PopoverRootProps;
    triggerProps?: Omit<PopoverTriggerProps, "asChild">;
    contentProps?: Omit<PopoverContentProps, "asChild">;
  } & HTMLButtonAttributes;

  const {
    button,
    content,
    children,
    transparent,
    rootProps,
    triggerProps,
    contentProps,
    ...buttonProps
  }: Props = $props();
</script>

<Popover.Root {...rootProps}>
  <Popover.Trigger {...triggerProps}>
    {#snippet child({ props })}
      {#if button}
        {@render button({ props })}
      {:else if transparent}
        <button
          class="transparent"
          {...props}
          {...buttonProps}
          onclick={(event) => {
            const p = props as HTMLButtonAttributes;
            if (buttonProps.onclick) buttonProps.onclick(event);
            if (p.onclick) p.onclick(event);
          }}
          type="button"
        >
          {@render children?.()}
        </button>
      {:else}
        <Button
          {...props}
          {...buttonProps}
          onclick={(event) => {
            const p = props as HTMLButtonAttributes;
            if (buttonProps.onclick) buttonProps.onclick(event);
            if (p.onclick) p.onclick(event);
          }}
          type="button"
        >
          {@render children?.()}
        </Button>
      {/if}
    {/snippet}
  </Popover.Trigger>
  <Popover.Portal>
    <Popover.Content sideOffset={8} {...contentProps}>
      {#snippet child({ props, wrapperProps, open })}
        <div {...wrapperProps} class="wrapper">
          {#if open}
            <div
              {...props}
              transition:scale={{ duration: 200 }}
              class="content"
            >
              {@render content?.()}
            </div>
          {/if}

          <Popover.Arrow>
            {#snippet child({ props })}
              <span {...props} class="arrow">
                <svg
                  height={12}
                  width={24}
                  viewBox="0 0 30 10"
                  preserveAspectRatio="none"
                  data-arrow=""
                >
                  <polygon points="0,0 30,0 15,10" fill="#453f4d" />
                </svg>
              </span>
            {/snippet}
          </Popover.Arrow>
        </div>
      {/snippet}
    </Popover.Content>
  </Popover.Portal>
</Popover.Root>

<style>
  .content {
    z-index: 30;
    min-width: 12rem;
    border-radius: 8px;
    padding: 0.5rem;
    background-color: #453f4d;
    box-shadow: 4px 0 10px #000;

    display: flex;
    flex-flow: column;
    gap: 0.75rem;
  }

  .arrow {
    color: #222;
  }

  .transparent {
    border: none;
    background: transparent;
    color: #fff;
    cursor: pointer;
  }
</style>
