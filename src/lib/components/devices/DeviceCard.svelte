<!-- Card for a known device -->
<script lang="ts">
  import type { DeviceId } from "$lib/api/types/devices";
  import type { FolderId } from "$lib/api/types/folders";
  import type { ProfileId } from "$lib/api/types/profiles";

  import { t } from "svelte-i18n";
  import { fingerprint } from "$lib/utils/fingerprint";
  import SolarTrashBin2BoldDuotone from "~icons/solar/trash-bin-2-bold-duotone";
  import SolarTranslationBoldDuotone from "~icons/solar/translation-bold-duotone";

  import Button from "../input/Button.svelte";
  import Select, { type Option } from "../input/Select.svelte";

  type Props = {
    id: DeviceId;
    name: string;
    publicKey: number[];
    profileId: ProfileId;
    folderId: FolderId | null;
    connected: boolean;

    profiles: Option[];
    folders: Option[];

    onRevoke: VoidFunction;
    onChangeProfile: (profileId: ProfileId) => void;
    onChangeFolder: (folderId: FolderId) => void;
  };

  const {
    id,
    name,
    publicKey,
    profileId,
    folderId,
    connected,

    profiles,
    folders,

    onRevoke,
    onChangeProfile,
    onChangeFolder,
  }: Props = $props();

  const fingerprintPromise = $derived(fingerprint(new Uint8Array(publicKey)));
</script>

<div class="card">
  <div class="head">
    <span class="identifier">{id}</span>

    <p class="state" data-connected={connected}>
      {#if connected}
        <SolarTranslationBoldDuotone />
        {$t("connected")}
      {:else}
        <SolarTranslationBoldDuotone />
        {$t("not_connected")}
      {/if}
    </p>
  </div>

  <h2 class="name">
    {name}
  </h2>

  {#await fingerprintPromise then print}
    <p class="fingerprint">{print}</p>
  {/await}

  <div class="actions">
    <Select
      value={profileId}
      options={profiles}
      onChangeValue={onChangeProfile}
      placeholder={$t("choose_profile")}
    />

    <Select
      value={folderId}
      options={folders}
      onChangeValue={onChangeFolder}
      placeholder={$t("choose_folder")}
    />

    <Button variant="error" onclick={onRevoke}>
      <SolarTrashBin2BoldDuotone />
      {$t("revoke")}
    </Button>
  </div>
</div>

<style>
  .card {
    display: flex;
    flex-flow: column;
    gap: var(--tp-space-3);
    align-items: flex-start;

    padding: var(--tp-space-4);
    border-radius: var(--tp-radius-md);
    background-color: var(--tp-bg-secondary);
    border: 1px solid var(--tp-border-secondary);
  }

  .head {
    display: flex;
    flex-flow: row;
    align-items: center;
    justify-content: space-between;
    gap: var(--tp-space-2);

    width: 100%;
  }

  .identifier {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
  }

  .name {
    font-size: var(--tp-text-lg);
    line-height: var(--tp-leading-tight);
  }

  .state {
    display: inline-flex;
    align-items: center;
    gap: var(--tp-space-2);
    font-size: var(--tp-text-sm);
    vertical-align: middle;
  }

  .state[data-connected="false"] {
    color: var(--tp-error-500);
  }

  .state[data-connected="true"] {
    color: var(--tp-success-500);
  }

  .actions {
    display: flex;
    align-items: center;
    width: 100%;
    gap: var(--tp-space-2);
    justify-content: flex-start;
  }

  .actions:global(> .wrapper) {
    max-width: 200px;
  }

  .fingerprint {
    color: var(--tp-text-secondary);
    font-size: var(--tp-text-xs);
  }
</style>
