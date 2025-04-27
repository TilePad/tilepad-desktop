<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
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
      loading: $t("profile_deleting"),
      success: $t("profile_deleted"),
      error: toastErrorMessage($t("profile_delete_error")),
    });

    open = false;
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button variant="error" {...props}>{$t("delete_profile")}</Button>
  {/snippet}

  {#snippet title()}
    {$t("delete_profile")}
  {/snippet}

  {#snippet description()}
    {$t("confirm_delete_profile")}
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: $t("close") }} />
    <Button type="submit" variant="error" onclick={onDelete}>
      {$t("delete")}
    </Button>
  {/snippet}
</Dialog>
