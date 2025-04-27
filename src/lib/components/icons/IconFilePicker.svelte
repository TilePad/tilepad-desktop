<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { uploadUserIcon } from "$lib/api/icons";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { TileIconType, type TileIcon } from "$lib/api/types/tiles";

  import Button from "../input/Button.svelte";

  type Props = {
    onSelectIcon: (icon: TileIcon) => void;
  };

  const { onSelectIcon }: Props = $props();

  let inputElm: HTMLInputElement | undefined = $state();

  async function onChangeFile() {
    if (!inputElm) return;

    const files = inputElm.files;
    if (!files) return;

    const file = files.item(0);
    if (!file) return;

    try {
      const path = await uploadUserIcon(file);
      onSelectIcon({ type: TileIconType.Uploaded, path });
    } catch (err) {
      toast.error(toastErrorMessage($t("file_upload_failed"))(err));
    }
  }
</script>

<Button
  type="button"
  onclick={() => {
    inputElm?.click();
  }}
  style="width: 100%"
>
  {$t("select_icon_file")}
</Button>

<input
  bind:this={inputElm}
  hidden
  multiple
  style="display: none;"
  type="file"
  onchange={onChangeFile}
  accept="image/*"
/>
