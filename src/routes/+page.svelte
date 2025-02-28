<script lang="ts">
  import ProfileEditor from "$lib/components/profiles/ProfileEditor.svelte";
  import ProfileFolders from "$lib/components/profiles/ProfileFolders.svelte";
  import ProfileSelector from "$lib/components/profiles/ProfileSelector.svelte";
</script>

<div class="layout">
  <div>
    <ProfileSelector />
    <ProfileFolders />
  </div>

  <ProfileEditor />
</div>

<style>
  .layout {
    display: flex;
    flex-flow: row;
    align-items: flex-start;
    height: 100%;
  }
</style>
