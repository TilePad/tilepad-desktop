<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { installIconPack } from "$lib/api/icons";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarImportBoldDuotone from "~icons/solar/import-bold-duotone";

  import Button from "../input/Button.svelte";

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeFile() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);
    if (!file) return;

    const createPromise = installIconPack(file);

    toast.promise(createPromise, {
      loading: $t("icon_packs_installing"),
      success: $t("icon_packs_installed"),
      error: toastErrorMessage($t("icon_packs_install_error")),
    });
  }
</script>

<Button
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  <SolarImportBoldDuotone width="1.5rem" height="1.5rem" />
  {$t("icon_packs_manual_import")}
</Button>

<input
  bind:this={inputElm}
  hidden
  multiple
  style="display: none;"
  type="file"
  onchange={onChangeFile}
  accept=".tilepadIcons"
/>
