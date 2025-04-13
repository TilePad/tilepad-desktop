<script lang="ts">
  import type { PluginRegistryEntry } from "$lib/api/types/plugins_registry";

  type Props = {
    item: PluginRegistryEntry;
    onClick: VoidFunction;
  };

  const { item, onClick }: Props = $props();
</script>

<button onclick={onClick}>
  {item.name}
  {item.description}
  {item.authors.join(", ")}
  {item.repo}
</button>
