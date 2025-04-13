<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  type Props = {
    item: PluginRegistryEntry;
    installed: boolean;
    onClick: VoidFunction;
  };

  const { item, installed, onClick }: Props = $props();
</script>

<button onclick={onClick} class="item">
  <p class="name">{item.name}</p>
  <p class="description"></p>

  <p class="authors">{item.authors.join(", ")}</p>

  <p>{item.repo}</p>

  <p>Installed: {installed}</p>
</button>

<style>
  .item {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
    background-color: #544d5e;
    color: #fff;
    border: none;
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .name {
    flex: auto;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-weight: bold;
  }
</style>
