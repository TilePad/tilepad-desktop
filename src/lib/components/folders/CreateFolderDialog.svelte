<script lang="ts">
  import { t } from "svelte-i18n";
  import { toast } from "svelte-sonner";
  import { createFolder } from "$lib/api/folders";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarAddFolderBold from "~icons/solar/add-folder-bold";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import { getFolderContext } from "./FolderProvider.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";
  import { getProfileContext } from "../profiles/ProfilesProvider.svelte";

  type Props = DialogProps & {
    order: number;
  };

  let { order }: Props = $props();

  const { profile } = getProfileContext();
  const { folder, setFolderId } = getFolderContext();

  const currentProfile = $derived.by(profile);
  const currentFolder = $derived.by(folder);

  let open = $state(false);
  let name = $state("");

  async function onCreate(event: Event) {
    event.preventDefault();
    if (name.length < 1) return;

    const createPromise = createFolder({
      name,
      default: false,
      config: {
        rows: currentFolder.config.rows,
        columns: currentFolder.config.columns,
      },
      profile_id: currentProfile.id,
      order,
    });

    toast.promise(createPromise, {
      loading: $t("folder_creating"),
      success: $t("folder_created"),
      error: toastErrorMessage($t("folder_create_error")),
    });

    const folder = await createPromise;

    open = false;
    reset();

    setFolderId(folder.id);
  }

  function reset() {
    name = "";
  }
</script>

<Tooltip title={$t("create_folder")}>
  {#snippet trigger({ props: triggerProps })}
    <Dialog {triggerProps} bind:open>
      {#snippet button({ props })}
        <button type="button" {...props} class="button">
          <SolarAddFolderBold />
        </button>
      {/snippet}

      {#snippet title()}
        {$t("create_folder")}
      {/snippet}

      <form onsubmit={onCreate}>
        <div class="content">
          <input
            autocomplete="off"
            bind:value={name}
            required
            minlength="1"
            class="input"
            placeholder={$t("name")}
          />
        </div>

        <div class="actions">
          <DialogCloseButton buttonLabel={{ text: $t("close") }} />
          <Button type="submit">{$t("create")}</Button>
        </div>
      </form>
    </Dialog>
  {/snippet}
</Tooltip>

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
    padding: 0.5rem;
    background-color: #000;
    border: 1px solid #666;
    color: #fff;
    border-radius: 0.25rem;
    align-items: center;
    display: flex;
    gap: 0.5rem;
  }

  .button {
    padding: 0rem 0.5rem;
    border: none;
    background-color: #141316;
    color: #fff;
    align-items: center;
    display: flex;
    gap: 0.5rem;
    cursor: pointer;
    font-size: 1em;
    text-decoration: none;
    border-top-right-radius: 0.25rem;
    border-bottom-right-radius: 0.25rem;
    justify-content: space-between;
  }
</style>
