<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import ProfileFolders from "$lib/components/profiles/ProfileFolders.svelte";
  import ProfileSelector from "$lib/components/profiles/ProfileSelector.svelte";
  import { getFolderContext } from "$lib/components/folders/FolderProvider.svelte";
  import FolderConfigEditor from "$lib/components/folders/FolderConfigEditor.svelte";

  const { folder } = getFolderContext();
  const currentFolder = $derived.by(folder);
</script>

<div class="layout">
  <div class="sidebar">
    <ProfileSelector />
    <FolderConfigEditor folder={currentFolder} />
    <ProfileFolders />
  </div>

  <div class="tiles">
    <FolderEditor />
  </div>

  <ActionsSidebar />
</div>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
  }

  .sidebar {
    width: 200px;
    flex-shrink: 0;
    background-color: #29262e;

    height: 100%;
  }

  .tiles {
    display: flex;
    flex: auto;
    overflow: hidden;
    height: 100%;
  }
</style>
