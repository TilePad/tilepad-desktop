<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { t } from "svelte-i18n";
  import { useDebounce } from "runed";
  import { mergeProps } from "bits-ui";
  import { queryClient } from "$lib/api/client";
  import SolarSettingsBold from "~icons/solar/settings-bold";
  import { foldersKeys, updateFolder } from "$lib/api/folders";

  import Tooltip from "../Tooltip.svelte";
  import NumberInput from "../input/NumberInput.svelte";
  import EditFolderDialog from "./EditFolderDialog.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";
  import DeleteFolderDialog from "./DeleteFolderDialog.svelte";

  type Props = {
    folder: FolderModel;
  };

  const { folder }: Props = $props();

  const debounceUpdateFolder = useDebounce(({ rows, columns }) => {
    updateFolder(folder.id, {
      config: {
        ...folder.config,
        rows,
        columns,
      },
    });
  }, 100);

  function onChangeRows(rows: number) {
    // Update the data on the UI ahead of time
    queryClient.setQueryData<FolderModel>(
      foldersKeys.specific(folder.profile_id, folder.id),
      (data) => {
        if (data === undefined) return data;
        return { ...data, config: { ...data.config, rows } };
      },
    );

    debounceUpdateFolder({ rows, columns: folder.config.columns });
  }

  function onChangeColumns(columns: number) {
    // Update the data on the UI ahead of time
    queryClient.setQueryData<FolderModel>(
      foldersKeys.specific(folder.profile_id, folder.id),
      (data) => {
        if (data === undefined) return data;
        return { ...data, config: { ...data.config, columns } };
      },
    );

    debounceUpdateFolder({ rows: folder.config.rows, columns });
  }
</script>

<Tooltip title={$t("folder_settings")}>
  {#snippet trigger({ props: triggerProps })}
    <PopoverButton {triggerProps}>
      {#snippet button({ props })}
        <button {...mergeProps(triggerProps, props)} class="button">
          <SolarSettingsBold width="1.25rem" height="1.25rem" />
        </button>
      {/snippet}

      {#snippet content()}
        <div>
          <label for="rows">{$t("rows")}</label>
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
          <label for="columns">{$t("columns")}</label>
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
        <DeleteFolderDialog {folder} />
      {/snippet}
    </PopoverButton>
  {/snippet}
</Tooltip>

<style>
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

    justify-content: space-between;
  }
</style>
