<script lang="ts">
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
      loading: "Installing icon pack...",
      success: "Installed icon pack",
      error: toastErrorMessage("Failed to install icon pack"),
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
  Import Pack
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
