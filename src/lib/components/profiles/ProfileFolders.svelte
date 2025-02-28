<script>
  import { createFoldersQuery } from "$lib/api/folders";

  import { getProfileContext } from "./ProfilesProvider.svelte";

  const { profile } = getProfileContext();
  const currentProfile = $derived.by(profile);

  const foldersQuery = createFoldersQuery(() => currentProfile.id);
  const folders = $derived($foldersQuery.data ?? []);
</script>

{#if $foldersQuery.isLoading}{:else if $foldersQuery.isError}{:else if $foldersQuery.isSuccess}
  <div>
    {#each folders as folder}
      <button>
        {folder.name}
      </button>
    {/each}
  </div>
{/if}
