<script lang="ts">
  import ActionItem, { type ActionItemData } from "./ActionItem.svelte";

  type Props = {
    actions: ActionItemData[];
  };

  const { actions }: Props = $props();
</script>

<section class="list action-list">
  {#each actions as action (action.pluginId + action.actionId)}
    <ActionItem {...action} />
  {/each}
</section>

<style>
  .list {
    width: 100%;
    height: 100%;
  }
</style>
