<script lang="ts">
  import { toast } from "svelte-sonner";
  import { installIconPack } from "$lib/api/icons";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import ImportIcon from "~icons/solar/import-bold-duotone";

  import Button from "../input/Button.svelte";

  const i18n = i18nContext.get();

  let inputElm: HTMLInputElement | undefined = $state();

  function onChangeFile() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);
    if (!file) return;

    const createPromise = installIconPack(file);

    toast.promise(createPromise, {
      loading: i18n.f("icon_packs_installing"),
      success: i18n.f("icon_packs_installed"),
      error: toastErrorMessage(i18n.f("icon_packs_install_error")),
    });
  }
</script>

<Button
  variant="secondary"
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  <ImportIcon width="1.5rem" height="1.5rem" />
  {i18n.f("import_pack")}
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
