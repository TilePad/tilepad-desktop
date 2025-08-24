<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import jacobAvatar from "$lib/assets/avatar-64x64.png";

  type Props = {
    version?: string;
  };

  const { version }: Props = $props();

  const i18n = i18nContext.get();
</script>

<div class="block">
  <img
    class="avatar"
    width="64"
    height="64"
    src={jacobAvatar}
    alt="Jacob Avatar"
  />
  <div class="creator-text">
    <p>
      {i18n.f("created_by")}
      <a class="creator" href="https://github.com/jacobtread" target="_blank">
        Jacobtread
      </a>
    </p>

    {#if version}
      <p class="version">
        {i18n.f("version")}:
        {version}
      </p>
    {/if}
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
