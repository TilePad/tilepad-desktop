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
      class="icon"
      src={getPluginAssetPath(action.plugin_id, action.icon)}
      alt="Action Icon"
    />
  {/if}

  <div class="action__text">
    <span class="label">{action.label}</span>
  </div>
</div>

<style>
  /* 	This is the css way of styling the dragged element */
  :global(#dnd-action-dragged-el .action) {
    transform: rotate(7deg);
  }

  .action {
    width: 15rem;
    padding: 0.5rem;
    display: flex;
    flex-flow: row;
    gap: 0.5rem;
    background-color: #1a181d;
  }

  .icon {
    max-width: 1.5rem;
  }

  .action__text {
    display: flex;
    flex-flow: column;
  }

  .label {
    font-weight: bold;
    font-size: 0.9rem;
  }
</style>
