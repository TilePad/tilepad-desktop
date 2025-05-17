<script lang="ts">
  import type { ServerConnectionInfo } from "$lib/api/types/server";

  import { t } from "svelte-i18n";
  import QRCode from "@castlenine/svelte-qrcode";
  import { getConnectionInfo } from "$lib/api/server";

  import SkeletonList from "../skeleton/SkeletonList.svelte";

  function encodeInterfaces(info: ServerConnectionInfo) {
    return {
      addr: info.interfaces.map((it) => it.addr),
      port: info.port,
    };
  }
</script>

<div class="column">
  {#await getConnectionInfo()}
    <SkeletonList />
  {:then connectInfo}
    {JSON.stringify(encodeInterfaces(connectInfo))}
    <div class="qr">
      <QRCode size={250} data={JSON.stringify(encodeInterfaces(connectInfo))} />
    </div>

    <div class="port">
      <b>{$t("port")}</b>
      {connectInfo.port}
    </div>

    <ul class="interfaces">
      {#each connectInfo.interfaces as int}
        <li class="interface">
          <b class="interface__name">{int.name}</b>
          <span class="interface__addr">{int.addr}</span>
        </li>
      {/each}
    </ul>
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
    overflow: hidden;
    height: 100%;
  }

  .interfaces {
    margin-left: 0;
    padding-inline-start: 0;
    flex: auto;
    overflow: auto;
    width: 100%;
  }

  .interface {
    padding: 0;
    display: flex;
    flex-flow: column;
    background-color: #2e2938;
    padding: 0.5rem;
  }

  .port {
    width: 100%;
    padding: 0.5rem;
    background-color: #443c55;
  }
</style>
