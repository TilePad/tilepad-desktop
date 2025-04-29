<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import TileDraggingProvider from "$lib/components/tiles/TileDraggingProvider.svelte";
</script>

<TileDraggingProvider>
  <div class="layout">
    <FolderEditor />
    <ActionsSidebar />
  </div>
</TileDraggingProvider>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
    overflow: hidden;
  }
</style>
