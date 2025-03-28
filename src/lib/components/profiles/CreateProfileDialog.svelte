<script lang="ts">
  import { toast } from "svelte-sonner";
  import { createProfile } from "$lib/api/profiles";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

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
      loading: "Creating profile",
      success: "Created profile",
      error: toastErrorMessage("Failed to create profile"),
    });
  }
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <form onsubmit={onCreate}>
      <div class="content">
        <h2>Create Profile</h2>
        <input
          required
          minlength="1"
          class="input"
          bind:value={name}
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
