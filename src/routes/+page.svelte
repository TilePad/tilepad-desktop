<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import FolderProvider from "$lib/components/folders/FolderProvider.svelte";
  import ProfilesProvider from "$lib/components/profiles/ProfilesProvider.svelte";
  import TileDraggingProvider from "$lib/components/tiles/TileDraggingProvider.svelte";
</script>

<ProfilesProvider>
  <FolderProvider>
    <TileDraggingProvider>
      <div class="layout">
        <FolderEditor />
        <ActionsSidebar />
      </div>
    </TileDraggingProvider>
  </FolderProvider>
</ProfilesProvider>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
    overflow: hidden;
  }
</style>
