<script lang="ts">
  import type { ServerConnectionInfo } from "$lib/api/types/server";

  import QRCode from "@castlenine/svelte-qrcode";
  import { getConnectionInfo } from "$lib/api/server";

  function encodeInterfaces(info: ServerConnectionInfo) {
    return {
      addr: info.interfaces.map((it) => it.addr),
      port: info.port,
    };
  }
</script>

<div class="column">
  {#await getConnectionInfo()}
    Loading...
  {:then connectInfo}
    <div class="qr">
      <QRCode size={250} data={JSON.stringify(encodeInterfaces(connectInfo))} />
    </div>

    <ul class="interfaces">
      {#each connectInfo.interfaces as int}
        <li class="interface">
          <b class="interface__name">{int.name}</b>
          <span class="interface__addr">{int.addr}</span>
        </li>
      {/each}
    </ul>

    <div class="port">
      <b>Port</b>
      {connectInfo.port}
    </div>
  {/await}
</div>

<style>
  .qr {
    width: 250px;
    height: 250px;
  }

  .column {
    display: flex;
    flex-flow: column;
    justify-content: flex-end;
    align-items: center;
    max-width: 250px;
  }

  .interfaces {
    margin-left: 0;
    padding-inline-start: 0;
    max-height: 200px;
    overflow: auto;
    width: 100%;
    border: 2px solid #443c55;
  }

  .interface {
    padding: 0;
    display: flex;
    flex-flow: column;
    background-color: #2e2938;
    padding: 0.5rem;
  }

  .interface__name {
  }

  .interface__addr {
  }

  .port {
    width: 100%;
    padding: 0.5rem;
    background-color: #443c55;
  }
</style>
