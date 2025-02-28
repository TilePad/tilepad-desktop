<script lang="ts">
  import type { FolderModel } from "$lib/api/types/folders";

  import { useDebounce } from "runed";
  import { queryClient } from "$lib/api/client";
  import { foldersKeys, updateFolder } from "$lib/api/folders";

  import { getFolderContext } from "./FolderProvider.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);

  const debounceUpdateFolder = useDebounce(({ rows, columns }) => {
    updateFolder(currentFolder.id, {
      config: {
        ...currentFolder.config,
        rows,
        columns,
      },
    });
  }, 100);

  function onChangeRows(rows: number) {
    queryClient.setQueryData<FolderModel>(
      foldersKeys.specific(currentFolder.profile_id, currentFolder.id),
      (data) => {
        if (data === undefined) return data;
        return { ...data, config: { ...data.config, rows } };
      },
    );

    debounceUpdateFolder({ rows, columns: currentFolder.config.columns });
  }

  function onChangeColumns(columns: number) {
    queryClient.setQueryData<FolderModel>(
      foldersKeys.specific(currentFolder.profile_id, currentFolder.id),
      (data) => {
        if (data === undefined) return data;
        return { ...data, config: { ...data.config, columns } };
      },
    );

    debounceUpdateFolder({ rows: currentFolder.config.rows, columns });
  }
</script>

<div>
  {currentFolder.name}

  <div>
    <label for="">Rows</label>
    <input
      type="number"
      value={currentFolder.config.rows}
      onchange={(event) => {
        onChangeRows(event.currentTarget.valueAsNumber);
      }}
    />
  </div>
  <div>
    <label for="">Columns</label>
    <input
      type="number"
      value={currentFolder.config.columns}
      onchange={(event) => {
        onChangeColumns(event.currentTarget.valueAsNumber);
      }}
    />
  </div>
</div>
