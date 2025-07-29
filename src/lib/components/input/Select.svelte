<script lang="ts" module>
  export interface Option {
    value: string;
    name: string;
  }
</script>

<script lang="ts" generics="T extends Option">
  import type { Snippet } from "svelte";

  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import DownArrow from "~icons/solar/alt-arrow-down-bold";
  import Button from "$lib/components/input/Button.svelte";

  type Props = {
    options: T[];
    value: string | null;
    onChangeValue: (value: string) => void;
    icon?: Snippet;
    placeholder?: string;
    item?: Snippet<[ItemSlotProps]>;
    trigger?: Snippet<[TriggerProps]>;
  };

  type ItemSlotProps = {
    option: T;
  };

  type TriggerProps = {
    currentOption: T | undefined;
    props: Record<string, unknown>;
    open: boolean;
  };

  const {
    options,
    value,
    onChangeValue,
    icon,
    placeholder,
    item = defaultItem,
    trigger = defaultTrigger,
  }: Props = $props();

  const currentOption = $derived(
    options.find((option) => option.value === value),
  );

  let open = $state(false);
</script>

{#snippet defaultTrigger({ currentOption, open, props }: TriggerProps)}
  <div class="wrapper" data-open={open}>
    <Button class="trigger" variant="secondary" {...props}>
      {@render icon?.()}

      {#if currentOption}
        {currentOption.name}
      {:else}
        {placeholder ?? "Select option"}
      {/if}
      <DownArrow class="trigger__icon" />
    </Button>
  </div>
{/snippet}

{#snippet defaultItem({ option }: { option: Option })}
  <span>{option.name}</span>
{/snippet}

<Select.Root
  bind:open
  allowDeselect={false}
  type="single"
  value={currentOption?.value}
  onValueChange={(value) => onChangeValue(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      {@render trigger({ currentOption, props, open })}
    {/snippet}
  </Select.Trigger>

  <Select.Portal>
    <Select.Content sideOffset={8} forceMount>
      {#snippet child({ props, open, wrapperProps })}
        <div {...wrapperProps} class="content-wrapper">
          {#if open}
            <div
              {...props}
              class="content"
              transition:slide={{ duration: 100 }}
            >
              {#each options as option (option.value)}
                <Select.Item value={option.value} label={option.name}>
                  {#snippet child({ props, selected, highlighted })}
                    <div
                      {...props}
                      class="item"
                      class:item--selected={selected}
                      class:item--highlighted={highlighted}
                    >
                      {@render item({ option })}
                    </div>
                  {/snippet}
                </Select.Item>
              {/each}
            </div>
          {/if}
        </div>
      {/snippet}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  .content {
    border-radius: 0.5rem;
    background-color: #2a2631;
    padding: 0.25rem;
    max-height: 40vh;
    min-width: 20rem;
    overflow: auto;
    box-shadow: 2px 10px 20px rgba(0, 0, 0, 0.1);
  }

  .form-input {
    display: inline-flex;
    flex-flow: column;
  }

  .item {
    cursor: pointer;
    display: flex;
    border-radius: 0.25rem;
    padding: 0.5rem;
    max-width: 500px;
  }

  .item:hover {
    background-color: #706580;
  }

  .item--selected {
    background-color: #675d75;
  }

  .item--highlighted {
    outline: 1px solid white;
  }

  .wrapper {
    width: 100%;
  }

  .wrapper:global(> .trigger) {
    width: 100%;
    justify-content: space-between;
  }

  .wrapper:global(> .trigger > .trigger__icon) {
    transition: all var(--tp-transition-fast);
    transform-origin: center;
  }

  .wrapper[data-open="true"]:global(> .trigger > .trigger__icon) {
    transform: rotate(-180deg);
  }

  .content-wrapper {
    z-index: 999 !important;
  }
</style>
