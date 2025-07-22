<!-- Button to allow manually importing a plugin -->
<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { installPlugin } from "$lib/api/plugins";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarImportBoldDuotone from "~icons/solar/import-bold-duotone";

  import Button from "../input/Button.svelte";

  let inputElm: HTMLInputElement | undefined = $state();

  function onChangeFile() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);
    if (!file) return;

    const createPromise = installPlugin(file);

    toast.promise(createPromise, {
      loading: $t("plugin_installing"),
      success: $t("plugin_installed"),
      error: toastErrorMessage($t("plugin_install_error")),
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
  {$t("import_plugin")}
</Button>

<input
  bind:this={inputElm}
  hidden
  multiple
  style="display: none;"
  type="file"
  onchange={onChangeFile}
  accept=".tilepadPlugin"
/>
