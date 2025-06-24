<script lang="ts">
  import { t } from "svelte-i18n";
  import { getLicenses } from "$lib/api/server";
  import { getErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Aside from "../Aside.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import Markdown from "../markdown/Markdown.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();
</script>

<Dialog {...restProps}>
  <div class="content">
    <div class="header">
      <h2>{$t("third_party_licenses")}</h2>
      <DialogCloseButton buttonLabel={{ text: $t("close") }} />
    </div>

    <div class="viewer">
      {#await getLicenses()}
        <SkeletonList />
      {:then markdown}
        <Markdown source={markdown} />
      {:catch error}
        <Aside severity="error" style="margin: 1rem;">
          {$t("readme_error", {
            values: { error: getErrorMessage(error) },
          })}
        </Aside>
      {/await}
    </div>
  </div>
</Dialog>

<style>
  .content {
    max-width: 100%;
    width: 90vw;
    height: 90vh;

    display: flex;
    flex-flow: column;
    padding-bottom: 0;
  }

  .viewer {
    flex: auto;
    overflow: auto;
  }

  .header {
    display: flex;
    flex-flow: row;
    flex-shrink: 0;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
    border-bottom: 1px solid #333;
    background-color: #29262e;
  }
</style>
