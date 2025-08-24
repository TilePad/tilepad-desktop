<script lang="ts">
  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { check, Update } from "@tauri-apps/plugin-updater";

  const i18n = i18nContext.get();

  async function checkUpdate(automatic: boolean) {
    let update: Update | null = null;
    try {
      update = await check();
    } catch (err) {
      console.error("failed to check for update:", err);
      return;
    }

    if (!update) return;

    const newVersion = update.version;

    if (automatic) {
      installUpdate(update);
    } else {
      toast(
        i18n.f("update_available", {
          values: { version: newVersion },
        }),
        {
          duration: Infinity,
          action: {
            label: "Update",
            onClick: () => installUpdate(update),
          },
        },
      );
    }
  }

  async function installUpdate(update: Update) {
    const updatePromise = update.download();

    toast.promise(updatePromise, {
      loading: i18n.f("update_downloading", {
        values: {
          version: update.version,
        },
      }),
      success: i18n.f("update_downloaded"),
      error: toastErrorMessage(i18n.f("update_download_error")),
    });

    await updatePromise;

    toast(i18n.f("install_update"), {
      duration: Infinity,
      action: {
        label: "Install",
        onClick: () => {
          update.install();
        },
      },
    });
  }

  $effect(() => {
    checkUpdate(false);
  });
</script>
