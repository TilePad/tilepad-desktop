<script lang="ts">
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createDeleteFolderMutation } from "$lib/api/folders";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    profileId: ProfileId;
    folder: FolderModel;
  };

  const { profileId, folder }: Props = $props();

  const deleteFolderMutation = createDeleteFolderMutation();

  let open = $state(false);

  function onDelete(event: Event) {
    event.preventDefault();

    const createPromise = deleteFolderMutation.mutateAsync(
      {
        profileId,
        folderId: folder.id,
      },
      {
        onSettled() {
          open = false;
        },
      },
    );

    toast.promise(createPromise, {
      loading: $t("folder_deleting"),
      success: $t("folder_deleted"),
      error: toastErrorMessage($t("folder_delete_error")),
    });
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>{$t("delete_folder")}</Button>
  {/snippet}

  {#snippet title()}
    {$t("delete_folder")}
  {/snippet}

  {#snippet description()}
    {$t("confirm_delete_folder")}
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: $t("close") }} />
    <Button type="submit" variant="error" onclick={onDelete}>
      {$t("delete")}
    </Button>
  {/snippet}
</Dialog>
