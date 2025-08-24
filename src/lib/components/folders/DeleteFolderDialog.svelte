<script lang="ts">
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
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

  const i18n = i18nContext.get();

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
      loading: i18n.f("folder_deleting"),
      success: i18n.f("folder_deleted"),
      error: toastErrorMessage(i18n.f("folder_delete_error")),
    });
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>{i18n.f("delete_folder")}</Button>
  {/snippet}

  {#snippet title()}
    {i18n.f("delete_folder")}
  {/snippet}

  {#snippet description()}
    {i18n.f("confirm_delete_folder")}
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
    <Button type="submit" variant="error" onclick={onDelete}>
      {i18n.f("delete")}
    </Button>
  {/snippet}
</Dialog>
