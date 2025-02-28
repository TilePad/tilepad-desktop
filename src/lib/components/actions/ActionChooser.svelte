<script lang="ts">
  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps;

  let { ...restProps }: Props = $props();
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <div class="content">
      <div class="split">
        <div class="list">
          <h2>Actions</h2>
          <input type="text" placeholder="Search..." />
        </div>
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: "Cancel" }} />
        <DialogCloseButton buttonLabel={{ text: "Select" }} />
      </div>
    </div>
  {/snippet}
</Dialog>

<style>
  .content {
    width: 100%;
    max-width: 30rem;
    max-height: 90vh;
  }

  .split {
    display: flex;
    flex: auto;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding: 1rem;
    gap: 1rem;
  }
</style>
