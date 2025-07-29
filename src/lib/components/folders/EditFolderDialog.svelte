<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { watch } from "runed";
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createSetFolderNameMutation } from "$lib/api/folders";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import TextInput from "../input/TextInput.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    folder: FolderModel;
  };

  const { folder }: Props = $props();

  const setFolderNameMutation = createSetFolderNameMutation();

  let open = $state(false);
  let name = $state(folder.name);

  function onSave(event: Event) {
    event.preventDefault();

    const updatePromise = $setFolderNameMutation.mutateAsync(
      {
        folderId: folder.id,
        name,
      },
      {
        onSettled: () => {
          open = false;
          reset();
        },
      },
    );

    toast.promise(updatePromise, {
      loading: $t("folder_updating"),
      success: $t("folder_updated"),
      error: toastErrorMessage($t("folder_update_error")),
    });
  }

  function reset() {
    name = "";
  }

  watch(
    () => folder,
    (folder) => {
      name = folder.name;
    },
  );
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button {...props}>{$t("edit_folder")}</Button>
  {/snippet}

  {#snippet title()}
    {$t("edit_folder")}
  {/snippet}

  <form onsubmit={onSave}>
    <div class="content">
      <TextInput
        autocomplete="off"
        bind:value={name}
        required
        minlength={1}
        class="input"
        placeholder={$t("name")}
      />
    </div>

    <div class="actions">
      <DialogCloseButton buttonLabel={{ text: $t("close") }} />
      <Button type="submit" loading={$setFolderNameMutation.isPending}>
        {$t("save")}
      </Button>
    </div>
  </form>
</Dialog>

<style>
  .content {
    max-width: 100%;
    width: 30rem;
    max-height: 90vh;
    display: flex;
    flex-flow: column;
    padding: 1rem;
    padding-bottom: 0;
  }

  .actions {
    display: flex;
    flex-flow: row;
    gap: 1rem;
    padding: 1rem;
    justify-content: flex-end;
  }
</style>
