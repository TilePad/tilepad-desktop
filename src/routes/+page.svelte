<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import CurrentFolder from "$lib/components/profiles/CurrentFolder.svelte";
  import ProfileFolders from "$lib/components/profiles/ProfileFolders.svelte";
  import ProfileSelector from "$lib/components/profiles/ProfileSelector.svelte";
</script>

<div class="layout">
  <div>
    <ProfileSelector />
    <CurrentFolder />
    <ProfileFolders />
  </div>

  <FolderEditor />
</div>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
  }
</style>
