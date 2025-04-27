<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { watch } from "runed";
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { updateProfile } from "$lib/api/profiles";
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
  let name = $state(profile.name);

  async function onSave(event: Event) {
    event.preventDefault();

    const updatePromise = updateProfile(profile.id, {
      name,
    });

    toast.promise(updatePromise, {
      loading: $t("profile_updating"),
      success: $t("profile_updated"),
      error: toastErrorMessage($t("profile_update_error")),
    });

    open = false;
    reset();
  }

  function reset() {
    name = "";
  }

  watch(
    () => profile,
    (profile) => {
      name = profile.name;
    },
  );
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button {...props}>{$t("edit_profile")}</Button>
  {/snippet}

  {#snippet title()}
    {$t("edit_profile")}
  {/snippet}

  {#snippet children()}
    <form onsubmit={onSave}>
      <div class="content">
        <input
          autocomplete="off"
          bind:value={name}
          required
          minlength="1"
          class="input"
          placeholder={$t("name")}
        />
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: $t("close") }} />
        <Button type="submit">{$t("save")}</Button>
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
