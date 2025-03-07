<script lang="ts">
  import ProfileFolders from "./ProfileFolders.svelte";
  import ProfileSelector from "./ProfileSelector.svelte";
</script>

<div class="sidebar">
  <ProfileSelector />
  <ProfileFolders />
</div>

<style>
  .sidebar {
    width: 200px;
    flex-shrink: 0;
    background-color: #29262e;
    border-right: 1px solid #333;

    height: 100%;
  }
</style>
