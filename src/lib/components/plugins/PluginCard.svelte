<!-- Card for a known device -->
<script lang="ts">
  import type { PluginWithState } from "$lib/api/types/plugin";
  type Props = {
    plugin: PluginWithState;
  };

  const { plugin }: Props = $props();
  const { manifest, state } = plugin;
</script>

<div class="plugin">
  <span class="plugin__version">{manifest.plugin.version}</span>

  <h2 class="plugin__name">
    {manifest.plugin.name}
  </h2>

  <p class="plugin__description">{manifest.plugin.description}</p>

  <span class="state">{state}</span>
</div>

<style>
  .plugin {
    display: flex;
    flex-flow: column;
    gap: 0.5rem;
    align-items: flex-start;

    padding: 1rem;
    border-radius: 0.5rem;
    background-color: #2f2c36;
  }

  .plugin__description {
    color: #ccc;
    font-size: 0.8rem;
  }
  .plugin__version {
    color: #ccc;
    font-size: 0.8rem;
  }

  .plugin__name {
    font-size: 1.2rem;
  }

  .state {
    padding: 0.5rem;
    display: inline-flex;
    gap: 0.5rem;
    font-size: 0.8rem;
    vertical-align: middle;
  }
</style>
