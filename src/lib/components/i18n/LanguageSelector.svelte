<script lang="ts">
  import { t } from "svelte-i18n";
  import { Select } from "bits-ui";
  import { slide } from "svelte/transition";
  import TwemojiFlagSpain from "~icons/twemoji/flag-spain";
  import TwemojiFlagFrance from "~icons/twemoji/flag-france";
  import TwemojiFlagGermany from "~icons/twemoji/flag-germany";
  import SolarAltArrowUpBold from "~icons/solar/alt-arrow-up-bold";
  import SolarAltArrowDownBold from "~icons/solar/alt-arrow-down-bold";
  import TwemojiFlagUnitedStates from "~icons/twemoji/flag-united-states";

  import Tooltip from "../Tooltip.svelte";

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
  ];

  const language = $derived(
    languages.find((language) => language.value === value),
  );

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
          <span class="label">
            {#if language}
              <language.icon />
              {language.label}
            {:else}
              {$t("select_language")}
            {/if}
          </span>

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
              {#each languages as language}
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

  .trigger {
    padding: 0.5rem;
    border: none;
    background-color: #141316;
    border: 1px solid #3a3542;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    justify-content: space-between;
    width: 100%;
  }

  .label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .trigger-wrapper {
    display: flex;
  }

  .auto-label {
    padding: 0.25rem 0.5rem;
    background-color: #141316;
    border-radius: 0.25rem;
  }
</style>
