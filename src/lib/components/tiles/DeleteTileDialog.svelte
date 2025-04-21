<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { toast } from "svelte-sonner";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import { createDeleteTileMutation } from "$lib/api/tiles";
  import SolarTrashBinTrashBoldDuotone from "~icons/solar/trash-bin-trash-bold-duotone";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    tile: TileModel;
    onClose: VoidFunction;
  };

  let { tile, onClose }: Props = $props();

  const deleteTile = createDeleteTileMutation();

  let open = $state(false);

  function onDelete() {
    if (!tile) return;

    const deletePromise = $deleteTile.mutateAsync({
      tileId: tile.id,
      folderId: tile.folder_id,
    });

    toast.promise(deletePromise, {
      loading: "Deleting tile",
      success: "Deleted tile",
      error: toastErrorMessage("Failed to delete tile"),
    });

    open = false;
    onClose();
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button transparent variant="error" {...props}>
      <SolarTrashBinTrashBoldDuotone width={24} height={24} />
    </Button>
  {/snippet}

  {#snippet title()}
    Confirm Delete
  {/snippet}

  {#snippet description()}
    Are you sure you want to delete this tile?
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: "Close" }} />
    <Button type="submit" variant="error" onclick={onDelete}>Delete</Button>
  {/snippet}
</Dialog>
