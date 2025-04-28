<script lang="ts">
  import { t } from "svelte-i18n";
  import { getVersion } from "@tauri-apps/api/app";
</script>

<div class="block">
  <img
    class="avatar"
    width="64"
    height="64"
    src="/avatar-64x64.png"
    alt="Jacob Avatar"
  />
  <div class="creator-text">
    <p>
      {$t("created_by")}
      <a class="creator" href="https://github.com/jacobtread" target="_blank">
        Jacobtread
      </a>
    </p>
    <p class="version">
      {$t("version")}:
      {#await getVersion() then version}
        {version}
      {/await}
    </p>
  </div>
</div>

<style>
  .avatar {
    display: inline-block;
    vertical-align: middle;
  }

  .block {
    display: flex;
    align-items: center;
  }

  .version {
    font-size: 0.8rem;
    color: #999;
  }

  .creator {
    color: #6c8de6;
    font-weight: bold;
  }
</style>
