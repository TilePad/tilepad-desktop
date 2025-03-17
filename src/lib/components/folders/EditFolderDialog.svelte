<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { watch } from "runed";
  import { toast } from "svelte-sonner";
  import { updateFolder } from "$lib/api/folders";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    folder: FolderModel;
  };

  let { folder }: Props = $props();

  let open = $state(false);
  let name = $state(folder.name);

  async function onSave(event: Event) {
    event.preventDefault();

    const updatePromise = updateFolder(folder.id, {
      name,
    });

    toast.promise(updatePromise, {
      loading: "Updating folder",
      success: "Updated folder",
      error: toastErrorMessage("Failed to update folder"),
    });

    open = false;

    reset();
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
    <Button {...props}>Edit</Button>
  {/snippet}

  {#snippet children()}
    <form onsubmit={onSave}>
      <div class="content">
        <h2>Edit Folder</h2>

        <input
          autocomplete="off"
          bind:value={name}
          required
          minlength="1"
          class="input"
          placeholder="Name"
        />
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: "Close" }} />
        <Button type="submit">Save</Button>
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

  .input {
    margin-top: 1rem;
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }
</style>
