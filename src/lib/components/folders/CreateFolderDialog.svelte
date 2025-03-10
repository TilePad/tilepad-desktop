<script lang="ts">
  import { toast } from "svelte-sonner";
  import { createFolder } from "$lib/api/folders";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  type Props = DialogProps & {
    order: number;
  };

  let { order, buttonLabel }: Props = $props();

  const { profile } = getProfileContext();
  const { folder } = getFolderContext();

  const currentProfile = $derived.by(profile);
  const currentFolder = $derived.by(folder);

  let open = $state(false);
  let name = $state("");

  async function onCreate(event: Event) {
    event.preventDefault();
    if (name.length < 1) return;

    const createPromise = createFolder({
      name,
      default: false,
      config: {
        rows: currentFolder.config.rows,
        columns: currentFolder.config.columns,
      },
      profile_id: currentProfile.id,
      order,
    });

    toast.promise(createPromise, {
      loading: "Creating folder",
      success: "Created folder",
      error: toastErrorMessage("Failed to create folder"),
    });

    open = false;
  }
</script>

<Dialog bind:open {buttonLabel}>
  {#snippet children()}
    <form onsubmit={onCreate}>
      <div class="content">
        <h2>Create Folder</h2>
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
        <DialogCloseButton
          buttonLabel={{ text: "Create" }}
          onclick={onCreate}
          type="submit"
        />
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
