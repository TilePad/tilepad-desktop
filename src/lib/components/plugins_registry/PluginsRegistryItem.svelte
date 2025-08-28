<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  type Props = {
    name: string;
    authors: string[];
    description: string;
    repo: string;

    selected: boolean;
    installed: boolean;
    onClick: VoidFunction;
  };

  const {
    name,
    authors,
    description,
    repo,
    selected,
    installed,
    onClick,
  }: Props = $props();

  const i18n = i18nContext.get();
</script>

<button onclick={onClick} class="item" class:item--selected={selected}>
  <p class="name">{name}</p>
  <p class="description">{description}</p>

  <p class="authors">{authors.join(", ")}</p>

  <p class="repo">{repo}</p>

  {#if installed}
    <p class="installed">{i18n.f("installed")}</p>
  {/if}
</button>

<style>
  .item {
    display: flex;
    flex-flow: column;
    gap: 0.25rem;
    color: #fff;
    border: none;
    width: 100%;
    text-align: left;
    padding: 0.5rem;
    cursor: pointer;
    font-size: 0.9rem;

    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-secondary);
    border: 1px solid var(--tp-border-secondary);
    overflow: hidden;
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
