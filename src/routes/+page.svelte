<script lang="ts">
  import FolderEditor from "$lib/components/folders/FolderEditor.svelte";
  import ActionsSidebar from "$lib/components/actions/ActionsSidebar.svelte";
  import ProfileSidebar from "$lib/components/profiles/ProfileSidebar.svelte";
</script>

<div class="layout">
  <ProfileSidebar />
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
