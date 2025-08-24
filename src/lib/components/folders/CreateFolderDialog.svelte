<script lang="ts">
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { FolderId, FolderConfig } from "$lib/api/types/folders";

  import { toast } from "svelte-sonner";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { toastErrorMessage } from "$lib/api/utils/error";
  import SolarAddFolderBold from "~icons/solar/add-folder-bold";
  import { createCreateFolderMutation } from "$lib/api/folders";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import TextInput from "../input/TextInput.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    /** Current profile ID */
    profileId: ProfileId;

    /** Base current folder config to inherit from */
    baseConfig: FolderConfig;

    /** Order to insert the next folder as */
    order: number;

    /** Handler for folder creation completed */
    onCreated: (folderId: FolderId) => void;
  };

  const { profileId, baseConfig, order, onCreated }: Props = $props();

  const i18n = i18nContext.get();

  const createFolderMutation = createCreateFolderMutation();

  let open = $state(false);
  let name = $state("");

  function onCreate(event: Event) {
    event.preventDefault();
    if (name.length < 1) return;

    const createPromise = createFolderMutation.mutateAsync(
      {
        create: {
          name,
          default: false,
          config: {
            rows: baseConfig.rows,
            columns: baseConfig.columns,
          },
          profile_id: profileId,
          order,
        },
      },
      {
        onSuccess: (folder) => {
          reset();
          open = false;

          onCreated(folder.id);
        },
      },
    );

    toast.promise(createPromise, {
      loading: i18n.f("folder_creating"),
      success: i18n.f("folder_created"),
      error: toastErrorMessage(i18n.f("folder_create_error")),
    });
  }

  function reset() {
    name = "";
  }
</script>

<Tooltip title={i18n.f("create_folder")}>
  {#snippet trigger({ props: triggerProps })}
    <Dialog {triggerProps} bind:open>
      {#snippet button({ props })}
        <div class="wrapper">
          <Button variant="secondary" {...props} class="button">
            <SolarAddFolderBold width="1.25rem" height="1.25rem" />
          </Button>
        </div>
      {/snippet}

      {#snippet title()}
        {i18n.f("create_folder")}
      {/snippet}

      <form onsubmit={onCreate}>
        <div class="content">
          <TextInput
            autocomplete="off"
            bind:value={name}
            required
            minlength={1}
            class="input"
            placeholder={i18n.f("name")}
          />
        </div>

        <div class="actions">
          <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
          <Button type="submit">{i18n.f("create")}</Button>
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

  .wrapper:global(> .button) {
    border-top-left-radius: 0;
    border-bottom-left-radius: 0;
  }
</style>
