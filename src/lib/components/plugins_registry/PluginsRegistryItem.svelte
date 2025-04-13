<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  type Props = {
    item: PluginRegistryEntry;
    onClick: VoidFunction;
  };

  const { item, onClick }: Props = $props();
</script>

<button onclick={onClick} class="item">
  <h2 class="name">{item.name}</h2>
  <p class="description"></p>

  <span class="authors"> {item.authors.join(", ")} </span>

  {item.repo}
</button>

<style>
  .item {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    background-color: #413c49;
    color: #fff;
    border: none;
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    cursor: pointer;
    background-color: #1a181d;
    font-size: 0.9rem;
  }

  .name {
    flex: auto;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>
