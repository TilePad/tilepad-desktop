<script lang="ts">
  import { toast } from "svelte-sonner";
  import { installPlugin } from "$lib/api/plugins";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import Button from "../input/Button.svelte";

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeFile() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);
    if (!file) return;

    const createPromise = installPlugin(file);

    toast.promise(createPromise, {
      loading: "Installing plugin...",
      success: "Installed plugin",
      error: toastErrorMessage("Failed to install plugin"),
    });
  }
</script>

<Button
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
>
  Import Plugin
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
