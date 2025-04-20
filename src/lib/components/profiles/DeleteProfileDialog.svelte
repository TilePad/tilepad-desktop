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

  {#snippet title()}
    Delete Profile
  {/snippet}

  {#snippet description()}
    Are you sure you want to delete this profile?
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: "Close" }} />
    <Button type="submit" variant="error" onclick={onDelete}>Delete</Button>
  {/snippet}
</Dialog>
