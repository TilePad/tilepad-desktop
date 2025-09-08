<script lang="ts">
  import FolderProvider from "$lib/components/folders/FolderProvider.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";

  import type { LayoutProps } from "./$types";

  const { children }: LayoutProps = $props();
</script>

<ProfilesProvider>
  <FolderProvider>
    {@render children?.()}
  </FolderProvider>
</ProfilesProvider>
