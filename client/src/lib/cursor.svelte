<script lang="ts">
  import { useTable, useReducer } from "spacetimedb/svelte";
  import { tables, reducers } from "$lib/module_bindings";
  import type { Identity } from "spacetimedb";
  import { untrack } from "svelte";

  interface Props {
    id: Identity;
    fps: number;
  }

  const { id, fps }: Props = $props();
  let dt = $derived(1000 / fps);

  const [self] = useTable(tables.cursor.where((c) => c.id.eq(id!)));
  const [cursors] = useTable(tables.cursor.where((c) => c.id.ne(id!)));

  const updatePosition = useReducer(reducers.updatePosition);

  let cursor = $state({ x: 0, y: 0 });
  let cursorMoved = $derived(!!cursor);

  let interval: number | undefined;
  $effect(() => {
    clearInterval(interval);

    if (!cursorMoved) return;

    interval = setInterval(() => {
      // Dont rerun on cursor move, just on `dt` change
      untrack(() => {
        updatePosition(cursor);
        cursorMoved = false;
      });
    }, dt);
  });
</script>

<svelte:window onmousemove={(e) => (cursor = { x: e.clientX, y: e.clientY })} />

{#each $cursors as cursor}
  <div
    class="cursor animate"
    style:--x={`${cursor.x}px`}
    style:--y={`${cursor.y}px`}
    style:--color={`#${cursor.id.toHexString().slice(10, 13)}`}
    style:--dt={`${dt}ms`}
  ></div>
{/each}

{#if $self[0]}
  <div
    class="cursor"
    style:--x={`${cursor.x}px`}
    style:--y={`${cursor.y}px`}
    style:--color={`#${$self[0].id.toHexString().slice(10, 13)}`}
  ></div>
{/if}

<style>
  .cursor {
    background-color: var(--color);
    display: block;
    width: 10px;
    aspect-ratio: 1 / 1;
    position: absolute;
    top: var(--y);
    left: var(--x);
    pointer-events: none;
  }
</style>
