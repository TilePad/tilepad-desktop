<script lang="ts">
  import type { TileModel } from "$lib/api/types/tiles";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
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

  const i18n = i18nContext.get();

  const deleteTile = createDeleteTileMutation();

  let open = $state(false);

  function onDelete() {
    if (!tile) return;

    const deletePromise = deleteTile.mutateAsync({
      tileId: tile.id,
      folderId: tile.folder_id,
    });

    toast.promise(deletePromise, {
      loading: i18n.f("tile_deleting"),
      success: i18n.f("tile_deleted"),
      error: toastErrorMessage(i18n.f("tile_delete_error")),
    });

    open = false;
    onClose();
  }
</script>

<Dialog bind:open>
  {#snippet button({ props })}
    <Button transparent variant="error" size="icon" {...props}>
      <SolarTrashBinTrashBoldDuotone width={24} height={24} />
    </Button>
  {/snippet}

  {#snippet title()}
    {i18n.f("confirm_delete")}
  {/snippet}

  {#snippet description()}
    {i18n.f("confirm_delete_tile")}
  {/snippet}

  {#snippet actions()}
    <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
    <Button type="submit" variant="error" onclick={onDelete}>
      {i18n.f("delete")}
    </Button>
  {/snippet}
</Dialog>
