<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  import Button from "../input/Button.svelte";

  type Props = {
    version: string;
    name: string;
    description: string | null;
    authors: string[];
    onUninstall: VoidFunction;
  };

  const { version, name, description, authors, onUninstall }: Props = $props();

  const i18n = i18nContext.get();
</script>

<div class="card">
  <div class="head">
    <span class="version">{version}</span>
  </div>

  <h2 class="name">
    {name}
  </h2>

  <p class="description">{description}</p>

  {#if authors.length > 0}
    <span class="authors">
      By {authors.join(", ")}
    </span>
  {/if}

  <div class="actions">
    <Button variant="secondary" size="small" onclick={onUninstall}>
      {i18n.f("uninstall")}
    </Button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-flow: column;
    gap: var(--tp-space-2);
    align-items: flex-start;

    padding: var(--tp-space-3);
    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-secondary);
    border: 1px solid var(--tp-border-secondary);
  }

  .head {
    display: flex;
    flex-flow: row;
    align-items: center;
    justify-content: space-between;
    gap: var(--tp-space-2);

    width: 100%;
  }

  .description {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
    max-width: 100%;
  }

  .version {
    color: var(--tp-text-tertiary);
    font-size: var(--tp-text-xs);
  }

  .name {
    font-size: var(--tp-text-lg);
    line-height: var(--tp-leading-tight);
  }

  .actions {
    display: flex;
    gap: var(--tp-space-3);
    flex-shrink: 0;
  }

  .authors {
    font-size: var(--tp-text-xs);
    color: var(--tp-text-tertiary);
  }
</style>
