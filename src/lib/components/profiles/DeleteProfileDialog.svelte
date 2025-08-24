<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { toast } from "svelte-sonner";
  import { deleteProfile } from "$lib/api/profiles";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    profile: ProfileModel;
  };

  let { profile }: Props = $props();

  const i18n = i18nContext.get();

  let open = $state(false);

  async function onDelete(event: Event) {
    event.preventDefault();

    const createPromise = deleteProfile(profile.id);

    toast.promise(createPromise, {
      loading: i18n.f("profile_deleting"),
      success: i18n.f("profile_deleted"),
      error: toastErrorMessage(i18n.f("profile_delete_error")),
    });

    open = false;
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>{i18n.f("delete_profile")}</Button>
  {/snippet}

  {#snippet title()}
    {i18n.f("delete_profile")}
  {/snippet}

  {#snippet description()}
    {i18n.f("confirm_delete_profile")}
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
    <Button type="submit" variant="error" onclick={onDelete}>
      {i18n.f("delete")}
    </Button>
  {/snippet}
</Dialog>
