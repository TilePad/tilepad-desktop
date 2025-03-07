<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import { getPluginAssetPath } from "$lib/api/utils/url";

  type Props = {
    action: Action;
  };
  const { action }: Props = $props();
</script>

<div class="action">
  {#if action.icon !== null}
    <img
      src={getPluginAssetPath(action.plugin_id, action.icon)}
      alt="Action Icon"
    />
  {/if}

  <div class="action__text">
    <span class="label">{action.label}</span>

    {#if action.description !== null}
      <span class="description">{action.description}</span>
    {/if}
  </div>
</div>

<style>
  .action {
    padding: 0.5rem;
    display: flex;
    flex-flow: row;
    gap: 0.5rem;
    width: 20rem;
    background-color: #35303b;
  }

  .action__text {
    display: flex;
    flex-flow: column;
  }

  .label {
    font-weight: bold;
  }
</style>
