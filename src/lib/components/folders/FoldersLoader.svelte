<script lang="ts">
  import type { Snippet } from "svelte";
  import type { FolderModel } from "$lib/api/types/folders";

  import { createFoldersQuery } from "$lib/api/folders";

  type Props = {
    profileId: string;
    content?: Snippet<[{ folders: FolderModel[] }]>;
  };

  const { profileId, content }: Props = $props();

  const foldersQuery = createFoldersQuery(() => profileId);
  const folders = $derived($foldersQuery.data ?? []);
</script>

{@render content?.({ folders })}
