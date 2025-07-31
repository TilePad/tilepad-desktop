<script lang="ts">
  import type { Snippet, Component } from "svelte";

  import { fly, fade } from "svelte/transition";
  import { Dialog, type WithoutChild } from "bits-ui";

  import Button from "../input/Button.svelte";

  export type DialogProps = Dialog.RootProps & {
    buttonLabel?: { text?: string; icon?: Component };
    button?: Snippet<[{ props: Record<string, unknown> }]>;

    title?: Snippet;
    description?: Snippet;
    actions?: Snippet;
    contentProps?: WithoutChild<Dialog.ContentProps>;
    triggerProps?: WithoutChild<Dialog.TriggerProps>;
    // ...other component props if you wish to pass them
  };

  let {
    open = $bindable(false),
    children,
    buttonLabel,
    button,
    contentProps,
    triggerProps,
    title,
    description,
    actions,
    ...restProps
  }: DialogProps = $props();
</script>

<Dialog.Root bind:open {...restProps}>
  <Dialog.Trigger {...triggerProps}>
    {#snippet child({ props })}
      {#if button}
        {@render button({ props })}
      {:else if buttonLabel}
        <Button variant="secondary" {...props} type="button">
          {#if buttonLabel.icon}
            <buttonLabel.icon
              width="1.5rem"
              height="1.5rem"
              style="flex-shrink: 0"
            />
          {/if}

          {buttonLabel.text}
        </Button>
      {/if}
    {/snippet}
  </Dialog.Trigger>

  <Dialog.Portal>
    <Dialog.Overlay forceMount>
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 250 }}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content {...contentProps} forceMount>
      {#snippet child({ props, open })}
        {#if open}
          <div
            {...props}
            class="content"
            transition:fly={{ y: 15, duration: 150 }}
          >
            {#if title}
              <Dialog.Title>
                {#snippet child({ props })}
                  <h3 {...props} class="title">{@render title()}</h3>
                {/snippet}
              </Dialog.Title>
            {/if}

            {#if description}
              <Dialog.Description>
                {#snippet child({ props })}
                  <p {...props} class="description">
                    {@render description()}
                  </p>
                {/snippet}
              </Dialog.Description>
            {/if}

            {@render children?.()}

            {#if actions}
              <div
                class="actions"
                class:actions--no-content={children === undefined}
              >
                {@render actions()}
              </div>
            {/if}
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 49;
  }

  .content {
    display: flex;
    flex-flow: column;

    position: fixed;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);

    background-color: #211e25;
    border: 1px solid #333;
    border-radius: 0.25rem;

    z-index: 50;
  }

  .title {
    background-color: #18161b;
    padding: 1rem;
    border-bottom: 1px solid #222;
    color: #fff;
    font-size: 1rem;
  }

  .description {
    color: #ccc;
    padding: 1rem;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding: 1rem;
    gap: 1rem;
  }

  .actions--no-content {
    padding-top: 0;
  }
</style>
