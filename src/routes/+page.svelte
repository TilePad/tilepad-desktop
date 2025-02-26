<script lang="ts">
  import { deviceRequestsQuery } from "$lib/api/devices";

  const requestsQuery = deviceRequestsQuery();
</script>

{#if $requestsQuery.isLoading}
  Loading requests...
{:else if $requestsQuery.isError}
  Failed to load device requests {$requestsQuery.error}
{:else if $requestsQuery.isSuccess}
  <div>
    {#each $requestsQuery.data as device}
      <div>
        <b>Name</b>: {device.device_name}
        <b>Address</b>: {device.socket_addr}

        <button>Approve</button>
        <button>Decline</button>
      </div>
    {/each}
  </div>
{/if}
