<script lang="ts">
  import type { TileId } from "$lib/api/types/tiles";
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";

  import { Dialog } from "bits-ui";
  import { fade, slide } from "svelte/transition";

  import TileEditorContent from "./TileEditorContent.svelte";

  type Props = {
    profileId: ProfileId;
    folderId: FolderId;
    tileId: TileId | null;
    onClose: VoidFunction;
  };

  const { profileId, folderId, tileId, onClose }: Props = $props();
</script>

<Dialog.Root
  open={tileId !== null}
  onOpenChange={(value) => {
    if (!value) onClose();
  }}
>
  <Dialog.Portal>
    <Dialog.Overlay forceMount>
      {#snippet child({ props })}
        {#if tileId}
          <div
            {...props}
            class="overlay"
            transition:fade={{ duration: 150 }}
          ></div>
        {/if}
      {/snippet}
    </Dialog.Overlay>
    <Dialog.Content trapFocus={false} forceMount>
      {#snippet child({ props })}
        {#if tileId}
          <div
            {...props}
            class="content"
            transition:slide={{ axis: "x", duration: 150 }}
          >
            <div class="content-inner">
              <TileEditorContent {profileId} {folderId} {tileId} {onClose} />
            </div>
          </div>
        {/if}
      {/snippet}
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>

<style>
  .overlay {
    position: fixed;
    left: 0;
    top: 0;
    width: 100vw;
    height: 100vh;
    background-color: #f4f6f8;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 19;
  }

  .content {
    position: fixed;
    right: 0;
    top: 0;
    width: 30rem;
    height: 100%;

    overflow: hidden;

    background-color: #18161b;
    border: 1px solid #222;
    border-radius: 0.25rem;

    z-index: 20;
  }

  .content-inner {
    display: flex;
    flex-flow: column;

    overflow: hidden;

    width: 30rem;
    height: 100%;
  }
</style>
