<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  import Select from "../input/Select.svelte";

  type Props = {
    fonts: string[];
    value: string | null;
    onChangeValue: (value: string) => void;
  };

  const { fonts, value, onChangeValue }: Props = $props();

  const i18n = i18nContext.get();

  const options = $derived([
    { value: "Roboto", name: "Roboto" },
    ...fonts
      .filter((font) => font !== "Roboto")
      .map((font) => ({ value: font, name: font })),
  ]);
</script>

<Select {options} {value} {onChangeValue} placeholder={i18n.f("select_font")}>
  {#snippet item({ option })}
    <span style="font-family: {option.value};">{option.name}</span>
  {/snippet}
</Select>
