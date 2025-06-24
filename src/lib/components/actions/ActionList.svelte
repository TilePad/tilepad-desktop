<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import ActionItem from "./ActionItem.svelte";

  type Props = {
    actions: Action[];
  };

  const { actions }: Props = $props();
</script>

<section class="list action-list">
  {#each actions as action (action.plugin_id + action.action_id)}
    <ActionItem {action} />
  {/each}
</section>

<style>
  .list {
    width: 100%;
    height: 100%;
  }
</style>
