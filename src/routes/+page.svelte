<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import CurrentFolder from "$lib/components/folders/CurrentFolder.svelte";
  import ProfileFolders from "$lib/components/profiles/ProfileFolders.svelte";
  import ProfileSelector from "$lib/components/profiles/ProfileSelector.svelte";
</script>

<div class="layout">
  <div class="left">
    <ProfileSelector />
    <CurrentFolder />
    <ProfileFolders />
  </div>

  <div class="tiles">
    <FolderEditor />
  </div>
</div>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
  }

  .left {
    width: 200px;
    flex-shrink: 0;
  }

  .tiles {
    display: flex;
    flex: auto;
    overflow: hidden;
    height: 100%;
  }
</style>
