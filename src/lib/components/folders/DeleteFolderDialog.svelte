<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { toast } from "svelte-sonner";
  import { deleteFolder } from "$lib/api/folders";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  type Props = DialogProps & {
    folder: FolderModel;
  };

  let { folder }: Props = $props();

  const { profile } = getProfileContext();

  const currentProfile = $derived.by(profile);

  let open = $state(false);

  async function onDelete(event: Event) {
    event.preventDefault();

    const createPromise = deleteFolder(currentProfile.id, folder.id);

    toast.promise(createPromise, {
      loading: "Deleting folder",
      success: "Deleted folder",
      error: toastErrorMessage("Failed to delete folder"),
    });

    open = false;
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>Delete</Button>
  {/snippet}

  {#snippet children()}
    <form onsubmit={onDelete}>
      <div class="content">
        <h2>Delete Folder</h2>

        <p>Are you sure you want to delete this folder?</p>
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: "Close" }} />
        <Button type="submit" variant="error">Delete</Button>
      </div>
    </form>
  {/snippet}
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
