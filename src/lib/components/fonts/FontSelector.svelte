<script lang="ts">
  import { t } from "svelte-i18n";
  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import { createFontsQuery } from "$lib/api/fonts";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";

  type Props = {
    value?: string;
    onChangeValue: (value: string) => void;
  };

  const { value, onChangeValue }: Props = $props();

  const fontsQuery = createFontsQuery();

  let open = $state(false);
</script>

<Select.Root
  allowDeselect={false}
  type="single"
  onOpenChange={(value) => (open = value)}
  {value}
  onValueChange={(value) => onChangeValue(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      <div class="trigger-wrapper">
        <button class="trigger" {...props}>
          {value ?? $t("select_font")}

          {#if open}
            <SolarAltArrowUpBold />
          {:else}
            <SolarAltArrowDownBold />
          {/if}
        </button>
      </div>
    {/snippet}
  </Select.Trigger>

  <Select.Portal>
    <Select.Content sideOffset={8} forceMount align="start">
      {#snippet child({ props, open, wrapperProps })}
        <div {...wrapperProps} class="content-wrapper">
          {#if open}
            <div
              {...props}
              class="content"
              transition:slide={{ duration: 100 }}
            >
              <!-- Roboto is a font always provided by tilepad -->
              <Select.Item value="Roboto" label="Roboto">
                {#snippet child({ props, selected, highlighted })}
                  <div
                    {...props}
                    class="item"
                    class:item--selected={selected}
                    class:item--highlighted={highlighted}
                    style="font-family: Roboto;"
                  >
                    Roboto
                  </div>
                {/snippet}
              </Select.Item>

              {#if $fontsQuery.isSuccess}
                {#each $fontsQuery.data as font, index (index)}
                  <Select.Item value={font} label={font}>
                    {#snippet child({ props, selected, highlighted })}
                      <div
                        {...props}
                        class="item"
                        class:item--selected={selected}
                        class:item--highlighted={highlighted}
                        style="font-family: {font};"
                      >
                        {font}
                      </div>
                    {/snippet}
                  </Select.Item>
                {/each}
              {/if}
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

  .trigger {
    padding: 0.5rem;
    border: none;
    background-color: #1f1d22;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    width: 100%;
    justify-content: space-between;
  }

  .trigger-wrapper {
    display: flex;
  }

  .content-wrapper {
    z-index: 999 !important;
  }
</style>
