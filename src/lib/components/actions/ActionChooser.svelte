<script lang="ts">
  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getPluginAssetPath } from "$lib/api/utils/url";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps;

  let { ...restProps }: Props = $props();

  const actionsQuery = createActionsQuery();
</script>

<Dialog {...restProps}>
  {#snippet children()}
    <div class="content">
      <div class="split">
        <div class="list">
          <h2>Actions</h2>
          <input type="text" placeholder="Search..." />

          <div>
            {#if $actionsQuery.isLoading}
              Loading actions...
            {:else if $actionsQuery.isError}
              Failed to load actions: {getErrorMessage($actionsQuery.error)}
            {:else if $actionsQuery.isSuccess}
              {#each $actionsQuery.data as category}
                <div>
                  {#if category.icon !== null}
                    <img
                      src={getPluginAssetPath(
                        category.plugin_id,
                        category.icon,
                      )}
                      alt="Action Icon"
                    />
                  {/if}

                  <h3>{category.label}</h3>

                  {#each category.actions as action}
                    <div>
                      {#if action.icon !== null}
                        <img
                          src={getPluginAssetPath(
                            action.plugin_id,
                            action.icon,
                          )}
                          alt="Action Icon"
                        />
                      {/if}

                      <span>{action.label}</span>

                      {#if action.description !== null}
                        <span>{action.description}</span>
                      {/if}
                    </div>
                  {/each}
                </div>
              {/each}
            {/if}
          </div>
        </div>
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: "Cancel" }} />
        <DialogCloseButton buttonLabel={{ text: "Select" }} />
      </div>
    </div>
  {/snippet}
</Dialog>

<style>
  .content {
    width: 100%;
    max-width: 30rem;
    max-height: 90vh;
  }

  .split {
    display: flex;
    flex: auto;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    padding: 1rem;
    gap: 1rem;
  }
</style>
