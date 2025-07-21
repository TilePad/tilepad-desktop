<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { check, Update } from "@tauri-apps/plugin-updater";

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
        $t("update_available", {
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
      loading: $t("update_downloading", {
        values: {
          version: update.version,
        },
      }),
      success: $t("update_downloaded"),
      error: toastErrorMessage($t("update_download_error")),
    });

    await updatePromise;

    toast($t("install_update"), {
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
