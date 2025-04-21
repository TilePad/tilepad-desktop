<script lang="ts">
  import { Tooltip } from "bits-ui";
  import { type Snippet } from "svelte";
  import { fade } from "svelte/transition";

  type Props = Tooltip.RootProps & {
    trigger: Snippet<[{ props: Record<string, unknown> }]>;
    triggerProps?: Tooltip.TriggerProps;
  };

  let {
    open = $bindable(false),
    children,
    onOpenChange,
    trigger,
    triggerProps = {},
    ...restProps
  }: Props = $props();
</script>

<Tooltip.Root
  ignoreNonKeyboardFocus
  delayDuration={150}
  {...restProps}
  bind:open
  {onOpenChange}
>
  <Tooltip.Trigger {...triggerProps}>
    {#snippet child({ props })}
      {@render trigger({ props })}
    {/snippet}
  </Tooltip.Trigger>
  <Tooltip.Portal>
    <Tooltip.Content forceMount>
      {#snippet child({ open, props, wrapperProps })}
        {#if open}
          <div {...wrapperProps}>
            <div transition:fade={{ duration: 100 }}>
              <div {...props} class="content">
                {@render children?.()}
              </div>

              <Tooltip.Arrow>
                {#snippet child({ props })}
                  <span {...props} class="arrow">
                    <svg
                      height={12}
                      width={24}
                      viewBox="0 0 30 10"
                      preserveAspectRatio="none"
                      data-arrow
                    >
                      <polygon points="0,0 30,0 15,10" fill="#131316" />
                    </svg>
                  </span>
                {/snippet}
              </Tooltip.Arrow>
            </div>
          </div>
        {/if}
      {/snippet}
    </Tooltip.Content>
  </Tooltip.Portal>
</Tooltip.Root>

<style>
  .content {
    z-index: 90;
    border-radius: 8px;
    padding: 0.25rem 0.5rem;
    font-size: 0.9rem;
    background-color: #1a1a1f;
    box-shadow: 4px 0 10px #000;
    border: 1px solid #333;

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
