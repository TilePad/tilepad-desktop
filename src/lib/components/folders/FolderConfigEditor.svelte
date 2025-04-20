<!-- Editor for the config of a folder -->
<script lang="ts">
  import SolarAltArrowRightLinear from "~icons/solar/alt-arrow-right-linear";

  import FolderSelector from "./FolderSelector.svelte";
  import ProfileSelector from "../profiles/ProfileSelector.svelte";
</script>

<div class="current">
  <div>
    <ProfileSelector />
  </div>

  <SolarAltArrowRightLinear />

  <div>
    <FolderSelector />
  </div>
</div>

<style>
  .current {
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    overflow: hidden;
    background-color: #29262e;

    display: flex;
    gap: 0.5rem;
    align-items: center;
  }
</style>
