<script lang="ts">
  import { createSpacetimeDBProvider } from "spacetimedb/svelte";
  import { DbConnection } from "$lib/module_bindings";
  import type { Identity } from "spacetimedb";

  import Cursor from "$lib/cursor.svelte";
  import Message from "$lib/message.svelte";

  const HOST = "https://maincloud.spacetimedb.com";
  // const HOST = "ws://localhost:3000";

  const DB_NAME = "simple-test-gamuwastaken";

  let id: Identity | undefined = $state(undefined);

  const conn = DbConnection.builder()
    .withUri(HOST)
    .withDatabaseName(DB_NAME)
    .onConnect((_conn, identity, _token) => {
      id = identity;
    })
    .onConnectError((_ctx, _error) => {
      console.log(_error, HOST);
    })
    .onDisconnect(() => {});

  createSpacetimeDBProvider(conn);
  const fps = 15;
</script>

{#if id}
  <Message {id} {fps} />

  <Cursor {id} {fps} />
{/if}

<style>
  :global(body) {
    width: max(100vw, 100%);
    height: 100vh;
    background-color: #4f43;
    overflow: hidden;
    margin: 0;
  }
  :global(.animate) {
    transition:
      top max(var(--dt), 150ms) linear,
      left max(var(--dt), 150ms) linear;
  }
</style>
