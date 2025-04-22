<script lang="ts">
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
      toast("An update is available v" + newVersion, {
        duration: Infinity,
        action: {
          label: "Update",
          onClick: () => installUpdate(update),
        },
      });
    }
  }

  async function installUpdate(update: Update) {
    const updatePromise = update.download();

    toast.promise(updatePromise, {
      loading: `Downloading update v${update.version}...`,
      success: "Update downloaded",
      error: toastErrorMessage("Failed to download update"),
    });

    await updatePromise;

    toast("Install the update (Will restart TilePad)", {
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
