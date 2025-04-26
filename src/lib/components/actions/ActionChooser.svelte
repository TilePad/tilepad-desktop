<script lang="ts">
  import type { Action } from "$lib/api/types/actions";

  import { createActionsQuery } from "$lib/api/actions";
  import { getErrorMessage } from "$lib/api/utils/error";
  import { getPluginAssetPath } from "$lib/api/utils/url";

  import type { DialogProps } from "../dialog/Dialog.svelte";

  import Dialog from "../dialog/Dialog.svelte";
  import { getServerContext } from "../ServerProvider.svelte";
  import DialogCloseButton from "../dialog/DialogCloseButton.svelte";

  type Props = DialogProps & {
    onSelect: (action: Action) => void;
  };

  let { onSelect, ...restProps }: Props = $props();

  const serverContext = getServerContext();

  let activeAction: Action | null = $state(null);

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
                        serverContext.serverURL,
                        category.plugin_id,
                        category.icon,
                      )}
                      alt="Action Icon"
                    />
                  {/if}

                  <h3>{category.label}</h3>

                  {#each category.actions as action}
                    <button
                      onclick={() => {
                        activeAction = action;
                      }}
                    >
                      {#if action.icon !== null}
                        <img
                          src={getPluginAssetPath(
                            serverContext.serverURL,
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
                    </button>
                  {/each}
                </div>
              {/each}
            {/if}
          </div>
        </div>
      </div>

      <div class="actions">
        <DialogCloseButton buttonLabel={{ text: "Cancel" }} />
        <DialogCloseButton
          disabled={activeAction === null}
          buttonLabel={{ text: "Create" }}
          onclick={() => {
            if (activeAction === null) return;
            onSelect(activeAction);
          }}
        />
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
