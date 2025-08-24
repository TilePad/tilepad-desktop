<script lang="ts">
  import type { ProfileModel } from "$lib/api/types/profiles";

  import { watch } from "runed";
  import { toast } from "svelte-sonner";
  import { setProfileName } from "$lib/api/profiles";
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
  let name = $state(profile.name);

  async function onSave(event: Event) {
    event.preventDefault();

    const updatePromise = setProfileName(profile.id, name);

    toast.promise(updatePromise, {
      loading: i18n.f("profile_updating"),
      success: i18n.f("profile_updated"),
      error: toastErrorMessage(i18n.f("profile_update_error")),
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
    <Button {...props}>{i18n.f("edit_profile")}</Button>
  {/snippet}

  {#snippet title()}
    {i18n.f("edit_profile")}
  {/snippet}

  <form onsubmit={onSave}>
    <div class="content">
      <input
        autocomplete="off"
        bind:value={name}
        required
        minlength="1"
        class="input"
        placeholder={i18n.f("name")}
      />
    </div>

    <div class="actions">
      <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
      <Button type="submit">{i18n.f("save")}</Button>
    </div>
  </form>
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
