<script lang="ts">
  import type { UpdateState } from "$lib/components/update/UpdaterPrompt.svelte";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { check, Update } from "@tauri-apps/plugin-updater";
  import UpdaterPrompt from "$lib/components/update/UpdaterPrompt.svelte";

  const i18n = i18nContext.get();

  interface UpdaterState {
    update: Update;
    state: UpdateState;
  }

  let currentState: UpdaterState | null = $state(null);

  async function checkUpdate() {
    currentState = null;

    let update: Update | null = null;
    try {
      update = await check();
    } catch (err) {
      console.error("failed to check for update:", err);
      return;
    }

    if (!update) return;
    currentState = { update, state: "ready" };
  }

  async function downloadUpdate(update: Update) {
    currentState = { update, state: "downloading" };

    try {
      await update.download();
      currentState = { update, state: "installable" };
    } catch (err) {
      toast.error(toastErrorMessage(i18n.f("update_download_error"))(err));
      currentState = null;
    }
  }

  async function installUpdate(update: Update) {
    currentState = { update, state: "installing" };

    try {
      await update.install();
    } catch (err) {
      toast.error(toastErrorMessage(i18n.f("update_download_error"))(err));
    } finally {
      currentState = null;
    }
  }

  function onClose() {
    currentState = null;
  }

  $effect(() => {
    if (!import.meta.env.DEV) {
      checkUpdate();
    }
  });
</script>

{#if currentState}
  {@const update = currentState.update}
  <UpdaterPrompt
    update={currentState.update}
    state={currentState.state}
    onUpdate={() => {
      if (currentState && currentState.state === "ready") {
        downloadUpdate(update);
      } else if (currentState && currentState.state === "installable") {
        installUpdate(update);
      }
    }}
    {onClose}
  />
{/if}
