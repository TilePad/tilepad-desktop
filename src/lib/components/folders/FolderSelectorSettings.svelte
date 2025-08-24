<script lang="ts">
  import type { ProfileId } from "$lib/api/types/profiles";
  import type { FolderModel } from "$lib/api/types/folders";

  import { useDebounce } from "runed";
  import { mergeProps } from "bits-ui";
  import { queryClient } from "$lib/api/client";
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import SettingsIcon from "~icons/solar/settings-bold";
  import {
    updateFolderRows,
    createSetFolderConfigMutation,
  } from "$lib/api/folders";

  import Tooltip from "../Tooltip.svelte";
  import Button from "../input/Button.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import EditFolderDialog from "./EditFolderDialog.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";
  import DeleteFolderDialog from "./DeleteFolderDialog.svelte";

  type Props = {
    profileId: ProfileId;
    folder: FolderModel;
  };

  const { profileId, folder }: Props = $props();

  const i18n = i18nContext.get();

  const setFolderConfigMutation = createSetFolderConfigMutation();

  const debounceUpdateFolder = useDebounce(({ rows, columns }) => {
    setFolderConfigMutation.mutateAsync({
      folderId: folder.id,
      config: {
        ...folder.config,
        rows,
        columns,
      },
    });
  }, 100);

  function onChangeRows(rows: number) {
    // Update the data on the UI ahead of time
    updateFolderRows(queryClient, folder.profile_id, folder.id, rows);

    debounceUpdateFolder({ rows, columns: folder.config.columns });
  }

  function onChangeColumns(columns: number) {
    // Update the data on the UI ahead of time
    updateFolderRows(queryClient, folder.profile_id, folder.id, columns);

    debounceUpdateFolder({ rows: folder.config.rows, columns });
  }
</script>

<Tooltip title={i18n.f("folder_settings")}>
  {#snippet trigger({ props: triggerProps })}
    <PopoverButton {triggerProps}>
      {#snippet button({ props })}
        <div class="wrapper">
          <Button
            variant="secondary"
            {...mergeProps(triggerProps, props)}
            class="button"
          >
            <SettingsIcon width="1.25rem" height="1.25rem" />
          </Button>
        </div>
      {/snippet}

      {#snippet content()}
        <div>
          <label for="rows">{i18n.f("rows")}</label>
          <NumberInput
            id="rows"
            type="number"
            value={folder.config.rows}
            onchange={(event) => {
              onChangeRows(event.currentTarget.valueAsNumber);
            }}
          />
        </div>
        <div>
          <label for="columns">{i18n.f("columns")}</label>
          <NumberInput
            id="columns"
            type="number"
            value={folder.config.columns}
            onchange={(event) => {
              onChangeColumns(event.currentTarget.valueAsNumber);
            }}
          />
        </div>

        <EditFolderDialog {folder} />
        {#if !folder.default}
          <DeleteFolderDialog {profileId} {folder} />
        {/if}
      {/snippet}
    </PopoverButton>
  {/snippet}
</Tooltip>

<style>
  .wrapper:global(> .button) {
    border-radius: 0;
  }
</style>
