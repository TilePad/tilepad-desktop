<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { createProfile } from "$lib/api/profiles";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarUserPlusBold from "~icons/solar/user-plus-bold";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Tooltip from "../Tooltip.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & { order: number };

  let { order, ...restProps }: Props = $props();

  let name = $state("");

  async function onCreate(event: Event) {
    event.preventDefault();
    if (name.length < 1) return;

    const createPromise = createProfile({
      name,
      default: false,
      config: {},
      order,
    });

    toast.promise(createPromise, {
      loading: $t("profile_creating"),
      success: $t("profile_created"),
      error: toastErrorMessage($t("profile_create_error")),
    });
  }
</script>

<Tooltip title={$t("create_profile")}>
  {#snippet trigger({ props: triggerProps })}
    <Dialog {triggerProps} {...restProps}>
      {#snippet button({ props })}
        <button {...props} class="button">
          <SolarUserPlusBold width="1.5rem" height="1.5rem" />
        </button>
      {/snippet}

      {#snippet title()}
        {$t("create_profile")}
      {/snippet}

      {#snippet children()}
        <form onsubmit={onCreate}>
          <div class="content">
            <input
              required
              minlength="1"
              class="input"
              bind:value={name}
              placeholder={$t("name")}
            />
          </div>

          <div class="actions">
            <DialogCloseButton buttonLabel={{ text: $t("close") }} />
            <DialogCloseButton
              buttonLabel={{ text: $t("create") }}
              onclick={onCreate}
              type="submit"
            />
          </div>
        </form>
      {/snippet}
    </Dialog>
  {/snippet}
</Tooltip>

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
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
    justify-content: space-between;
  }
</style>
