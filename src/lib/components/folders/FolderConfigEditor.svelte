<!-- Editor for the config of a folder -->
<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { useDebounce } from "runed";
  import { queryClient } from "$lib/api/client";
  import { foldersKeys, updateFolder } from "$lib/api/folders";
  import SolarSettingsBoldDuotone from "~icons/solar/settings-bold-duotone";

  import NumberInput from "../input/NumberInput.svelte";
  import PopoverButton from "../popover/PopoverButton.svelte";
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

<div class="current">
  {folder.name}

  <PopoverButton>
    {#snippet children()}<SolarSettingsBoldDuotone />{/snippet}

    {#snippet content()}
      <div>
        <label for="">Rows</label>
        <NumberInput
          type="number"
          value={folder.config.rows}
          onchange={(event) => {
            onChangeRows(event.currentTarget.valueAsNumber);
          }}
        />
      </div>
      <div>
        <label for="">Columns</label>
        <NumberInput
          type="number"
          value={folder.config.columns}
          onchange={(event) => {
            onChangeColumns(event.currentTarget.valueAsNumber);
          }}
        />
      </div>
    {/snippet}
  </PopoverButton>
</div>

<style>
  .current {
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    overflow: hidden;
    background-color: #29262e;

    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: space-between;
  }
</style>
