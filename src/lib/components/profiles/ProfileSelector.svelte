<script lang="ts">
  import type { ProfileId, ProfileModel } from "$lib/api/types/profiles";

  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import DownArrowIcon from "~icons/solar/alt-arrow-down-bold";
  import SolarUsersGroupRoundedBoldDuotone from "~icons/solar/users-group-rounded-bold-duotone";

  import type { Option } from "../input/Select.svelte";

  import Button from "../input/Button.svelte";
  import Select from "../input/Select.svelte";

  type Props = {
    options: ProfileModel[];
    value: ProfileId;
    onChangeValue: (value: ProfileId) => void;
  };

  type ProfileOption = Option & {
    default: boolean;
  };

  const { options, value, onChangeValue }: Props = $props();

  const i18n = i18nContext.get();

  const profileOptions: ProfileOption[] = $derived(
    options.map((option) => ({
      value: option.id,
      name: option.name,
      default: option.default,
    })),
  );
</script>

<Select options={profileOptions} {value} {onChangeValue}>
  {#snippet item({ option })}
    <div class="item">
      <span>{option.name}</span>

      {#if option.default}
        <span class="default-label">
          {i18n.f("default")}
        </span>
      {/if}
    </div>
  {/snippet}

  {#snippet trigger({ currentOption, open, props })}
    <div class="wrapper" data-open={open}>
      <Button variant="secondary" class="trigger" {...props}>
        <SolarUsersGroupRoundedBoldDuotone />

        {#if currentOption}
          {currentOption.name}
        {/if}

        <DownArrowIcon class="trigger__icon" />
      </Button>
    </div>
  {/snippet}
</Select>

<style>
  .item {
    display: inline-flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .default-label {
    padding: 0.25rem 0.5rem;
    background-color: #141316;
    border-radius: 0.25rem;
  }

  .wrapper {
    display: flex;
  }

  .wrapper:global(> .trigger) {
    justify-content: space-between;
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .wrapper:global(> .trigger > .trigger__icon) {
    transition: all var(--tp-transition-fast);
    transform-origin: center;
  }

  .wrapper[data-open="true"]:global(> .trigger > .trigger__icon) {
    transform: rotate(-180deg);
  }
</style>
