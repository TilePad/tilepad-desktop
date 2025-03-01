<script module lang="ts">
  export type MovableAction = Action & {
    id: string;
    [SHADOW_ITEM_MARKER_PROPERTY_NAME]?: boolean;
  };
</script>

<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import { watch } from "runed";
  import { getPluginAssetPath } from "$lib/api/utils/url";
  import {
    dndzone,
    TRIGGERS,
    type DndEvent,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
  } from "svelte-dnd-action";

  type Props = {
    actions: Action[];
  };

  const { actions }: Props = $props();

  let items: MovableAction[] = $state([]);
  let shouldIgnoreDndEvents = $state(false);

  function handleDndConsider(e: CustomEvent<DndEvent<MovableAction>>) {
    const { trigger, id } = e.detail.info;
    if (trigger === TRIGGERS.DRAG_STARTED) {
      console.warn(`copying ${id}`);
      const idx = items.findIndex((item) => item.id === id);
      const newId = `${id}_copy_${Math.round(Math.random() * 100000)}`;
      e.detail.items = e.detail.items.filter(
        (item) => !item[SHADOW_ITEM_MARKER_PROPERTY_NAME],
      );
      e.detail.items.splice(idx, 0, { ...items[idx], id: newId });
      items = e.detail.items;
      shouldIgnoreDndEvents = true;
    } else if (!shouldIgnoreDndEvents) {
      items = e.detail.items;
    } else {
      items = [...items];
    }
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<MovableAction>>) {
    if (!shouldIgnoreDndEvents) {
      items = e.detail.items;
    } else {
      items = [...items];
      shouldIgnoreDndEvents = false;
    }
  }

  watch(
    () => actions,
    (actions) => {
      items = actions.map((action) => ({ id: action.action_id, ...action }));
    },
  );
</script>

<section
  class="list"
  use:dndzone={{
    items,
    flipDurationMs: 0,
    dropTargetStyle: {},
    morphDisabled: true,
  }}
  onconsider={handleDndConsider}
  onfinalize={handleDndFinalize}
>
  {#each items as action (action.id)}
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
  {/each}
</section>

<style>
  .list {
    width: 100%;
    height: 100%;
  }

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
