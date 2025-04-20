<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { toast } from "svelte-sonner";
  import { deleteProfile } from "$lib/api/profiles";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    profile: ProfileModel;
  };

  let { profile }: Props = $props();

  let open = $state(false);

  async function onDelete(event: Event) {
    event.preventDefault();

    const createPromise = deleteProfile(profile.id);

    toast.promise(createPromise, {
      loading: "Deleting profile",
      success: "Deleted profile",
      error: toastErrorMessage("Failed to delete profile"),
    });

    open = false;
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>Delete Profile</Button>
  {/snippet}

  {#snippet children()}
    <form onsubmit={onDelete}>
      <div class="content">
        <h2>Delete Profile</h2>

        <p>Are you sure you want to delete this profile?</p>
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
