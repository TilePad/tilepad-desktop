<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
</script>

<div class="layout">
  <FolderEditor />
  <ActionsSidebar />
</div>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
    overflow: hidden;
  }
</style>
