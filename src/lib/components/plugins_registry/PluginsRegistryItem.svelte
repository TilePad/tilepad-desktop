<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  import { t } from "svelte-i18n";

  type Props = {
    item: PluginRegistryEntry;
    selected: boolean;
    installed: boolean;
    onClick: VoidFunction;
  };

  const { item, selected, installed, onClick }: Props = $props();
</script>

<button onclick={onClick} class="item" class:item--selected={selected}>
  <p class="name">{item.name}</p>
  <p class="description">{item.description}</p>

  <p class="authors">{item.authors.join(", ")}</p>

  <p class="repo">{item.repo}</p>

  {#if installed}
    <p class="installed">{$t("installed")}</p>
  {/if}
</button>

<style>
  .item {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
    background-color: #28252e;
    color: #fff;
    border: none;
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 0.9rem;
    border-radius: 0.25rem;
  }

  .item--selected {
    background-color: #453f4e;
  }

  .authors {
    font-size: 0.9rem;
  }

  .repo {
    font-size: 0.9rem;
    color: #ccc;
  }

  .name {
    flex: auto;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
    font-weight: bold;
  }

  .installed {
    color: #aaa;
    font-style: italic;
  }
</style>
