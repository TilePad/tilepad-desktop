<script lang="ts">
  import type { IconPack } from "$lib/api/types/icons";

  import { toast } from "svelte-sonner";
  import { uninstallIconPack } from "$lib/api/icons";
  import { toastErrorMessage } from "$lib/api/utils/error";

  import Button from "../input/Button.svelte";

  type Props = {
    pack: IconPack;
  };

  const { pack }: Props = $props();
  const manifest = $derived(pack.manifest);

  function handleUninstall() {
    const revokePromise = uninstallIconPack(manifest.icons.id);

    toast.promise(revokePromise, {
      loading: "Uninstalling icon pack",
      success: "Uninstalled icon pack",
      error: toastErrorMessage("Failed to uninstall icon pack"),
    });
  }
</script>

<div class="card">
  <div class="top">
    <span class="version">{manifest.icons.version}</span>

    <div class="actions">
      <Button size="small" onclick={handleUninstall}>Uninstall</Button>
    </div>
  </div>

  <h2 class="name">
    {manifest.icons.name}
  </h2>

  <p class="description">{manifest.icons.description}</p>
</div>

<style>
  .top {
    display: flex;
    flex-flow: row;
    justify-content: space-between;
    align-items: center;
    width: 100%;
  }

  .card {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
  }

  .description {
    color: #ccc;
    font-size: 0.8rem;
  }

  .version {
    color: #ccc;
    font-size: 0.8rem;
  }

  .name {
    font-size: 1.2rem;
  }

  .actions {
    display: flex;
    gap: 1rem;
  }
</style>
