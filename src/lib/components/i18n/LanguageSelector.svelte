<script lang="ts">
  import { t } from "svelte-i18n";
  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import TwemojiFlagSpain from "~icons/twemoji/flag-spain";
  import DownArrow from "~icons/solar/alt-arrow-down-bold";
  import TwemojiFlagFrance from "~icons/twemoji/flag-france";
  import TwemojiFlagGermany from "~icons/twemoji/flag-germany";
  import TwemojiFlagCzechia from "~icons/twemoji/flag-czechia";
  import TwemojiFlagUnitedStates from "~icons/twemoji/flag-united-states";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";

  type Props = {
    value: string;
    onChangeValue: (value: string) => void;
  };

  const { value, onChangeValue }: Props = $props();

  const languages = [
    { icon: TwemojiFlagUnitedStates, label: "English", value: "en" },
    { icon: TwemojiFlagGermany, label: "Deutsch", value: "de", auto: true },
    { icon: TwemojiFlagFrance, label: "Français", value: "fr", auto: true },
    { icon: TwemojiFlagSpain, label: "Español", value: "es", auto: true },
    { icon: TwemojiFlagCzechia, label: "Čeština", value: "cs", auto: true },
  ];

  const language = $derived(
    languages.find((language) => language.value === value),
  );

  let open = $state(false);
</script>

<Select.Root
  bind:open
  allowDeselect={false}
  type="single"
  {value}
  onValueChange={(value) => onChangeValue(value)}
>
  <Select.Trigger>
    {#snippet child({ props })}
      <div class="wrapper" data-open={open}>
        <Button class="trigger" variant="secondary" {...props}>
          <span class="label">
            {#if language}
              <language.icon />
              {language.label}
            {:else}
              {$t("select_language")}
            {/if}
          </span>
          <DownArrow class="trigger__icon" />
        </Button>
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
              {#each languages as language (language.value)}
                <Select.Item value={language.value} label={language.label}>
                  {#snippet child({ props, selected, highlighted })}
                    <div
                      {...props}
                      class="item"
                      class:item--selected={selected}
                      class:item--highlighted={highlighted}
                    >
                      <span class="label">
                        <language.icon />
                        {language.label}
                      </span>

                      {#if language.auto}
                        <Tooltip title={$t("auto_generated")}>
                          {#snippet trigger({ props })}
                            <span {...props} class="auto-label">
                              {$t("auto")}
                            </span>
                          {/snippet}
                        </Tooltip>
                      {/if}
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
    gap: 0.5rem;
    border-radius: 0.25rem;
    padding: 0.5rem;
    max-width: 500px;
    align-items: center;
    justify-content: space-between;
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

  .label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .auto-label {
    padding: 0.25rem 0.5rem;
    background-color: #141316;
    border-radius: 0.25rem;
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
</style>
