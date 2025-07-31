<script lang="ts">
  import { t } from "svelte-i18n";

  import Select from "../input/Select.svelte";

  type Props = {
    fonts: string[];
    value: string | null;
    onChangeValue: (value: string) => void;
  };

  const { fonts, value, onChangeValue }: Props = $props();
  const options = $derived([
    { value: "Roboto", name: "Roboto" },
    ...fonts
      .filter((font) => font !== "Roboto")
      .map((font) => ({ value: font, name: font })),
  ]);
</script>

<Select {options} {value} {onChangeValue} placeholder={$t("select_font")}>
  {#snippet item({ option })}
    <span style="font-family: Roboto;">{option.name}</span>
  {/snippet}
</Select>
