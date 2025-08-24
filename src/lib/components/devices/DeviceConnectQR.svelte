<script lang="ts">
  import type { ServerConnectionInfo } from "$lib/api/types/server";

  import QRCode from "@castlenine/svelte-qrcode";
  import Aside from "$lib/components/Aside.svelte";
  import { i18nContext } from "$lib/i18n/i18n.svelte";

  type Props = {
    connectInfo: ServerConnectionInfo;
  };

  const { connectInfo }: Props = $props();

  const i18n = i18nContext.get();

  const encodedInterfaces = $derived(
    JSON.stringify({
      addr: connectInfo.interfaces.map((it) => it.addr),
      port: connectInfo.port,
    }),
  );
</script>

<div class="column">
  {#if connectInfo.interfaces.length > 0}
    <div class="qr">
      <QRCode size={250} data={encodedInterfaces} />
    </div>

    <div class="port">
      <b>{i18n.f("port")}</b>
      {connectInfo.port}
    </div>

    <ul class="interfaces">
      {#each connectInfo.interfaces as int, index (index)}
        <li class="interface">
          <b class="interface__name">{int.name}</b>
          <span class="interface__addr">{int.addr}</span>
        </li>
      {/each}
    </ul>
  {:else}
    <Aside severity="error" style="margin: 1rem;">
      {i18n.f("no_interfaces")}
    </Aside>
  {/if}
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
