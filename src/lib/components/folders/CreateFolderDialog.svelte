<script lang="ts">
  import { toast } from "svelte-sonner";
  import { createFolder } from "$lib/api/folders";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarAddFolderBold from "~icons/solar/add-folder-bold";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";
  type Props = DialogProps & {
    order: number;
  };

  let { order }: Props = $props();

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

    reset();
  }

  function reset() {
    name = "";
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <button type="button" {...props} class="button">
      <SolarAddFolderBold />
    </button>
  {/snippet}
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
        <Button type="submit">Create</Button>
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

  .button {
    padding: 0rem 0.5rem;
    border: none;
    background-color: #141316;
    color: #fff;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;

    justify-content: space-between;
  }
</style>
