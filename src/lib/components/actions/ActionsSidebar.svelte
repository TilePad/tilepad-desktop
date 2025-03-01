<script lang="ts">
  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getPluginAssetPath } from "$lib/api/utils/url";

  import ActionsSidebarList from "./ActionsSidebarList.svelte";

  const actionsQuery = createActionsQuery();
</script>

<div class="list">
  <h2>Actions</h2>
  <input type="text" placeholder="Search..." />

  <div>
    {#if $actionsQuery.isLoading}
      Loading actions...
    {:else if $actionsQuery.isError}
      Failed to load actions: {getErrorMessage($actionsQuery.error)}
    {:else if $actionsQuery.isSuccess}
      {#each $actionsQuery.data as category}
        <div>
          {#if category.icon !== null}
            <img
              src={getPluginAssetPath(category.plugin_id, category.icon)}
              alt="Action Icon"
            />
          {/if}

          <h3>{category.label}</h3>

          <ActionsSidebarList actions={category.actions} />
        </div>
      {/each}
    {/if}
  </div>
</div>

<style>
  .list {
    flex-shrink: 0;
    max-width: 15rem;
  }
</style>
