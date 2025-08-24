<script lang="ts">
  import { i18nContext } from "$lib/i18n/i18n.svelte";
  import { getErrorMessage } from "$lib/api/utils/error";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Aside from "../Aside.svelte";
  import Dialog from "../dialog/Dialog.svelte";
  import Markdown from "../markdown/Markdown.svelte";
  import SkeletonList from "../skeleton/SkeletonList.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {};

  const { ...restProps }: Props = $props();

  const i18n = i18nContext.get();

  const licenseMarkdownPromise = import(
    "../../../../THIRD_PARTY_LICENSES.md?raw"
  );
</script>

<Dialog {...restProps}>
  <div class="content">
    <div class="header">
      <h2>{i18n.f("third_party_licenses")}</h2>
      <DialogCloseButton buttonLabel={{ text: i18n.f("close") }} />
    </div>

    <div class="viewer">
      {#await licenseMarkdownPromise}
        <SkeletonList />
      {:then markdown}
        <Markdown source={markdown.default} />
      {:catch error}
        <Aside severity="error" style="margin: 1rem;">
          {i18n.f("readme_error", {
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
