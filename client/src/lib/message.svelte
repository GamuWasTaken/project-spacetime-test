<script lang="ts">
  import { useTable, useReducer } from "spacetimedb/svelte";
  import { tables, reducers } from "$lib/module_bindings";
  import { Identity, Timestamp } from "spacetimedb";
  import { untrack } from "svelte";

  interface Props {
    id: Identity;
    fps: number;
  }
  const { id, fps }: Props = $props();
  let dt = $derived(1000 / fps);

  const [_message] = useTable(tables.message);
  let message: string | undefined = $state(undefined);
  $effect(() => {
    if (
      id.toHexString() == $_message[0]?.author.toHexString() &&
      message != undefined
    ) {
    } else {
      untrack(() => (message = $_message[0]?.message));
    }
  });

  const [_position] = useTable(tables.message_position);
  let [x, y] = $derived([$_position[0]?.x, $_position[0]?.y]);

  const updateMessage = useReducer(reducers.updateMessage);
  const updatePosition = useReducer(reducers.updateMessagePosition);

  let interval: number | undefined;

  let drag = $state({ grabbed: false, offsetX: 0, offsetY: 0 });

  function onmousemove(e: MouseEvent) {
    if (!drag.grabbed) return;

    x = e.x - drag.offsetX;
    y = e.y - drag.offsetY;
  }

  function onmousedown(e: MouseEvent) {
    const t = (e.target as HTMLElement).getBoundingClientRect();
    drag = {
      grabbed: true,
      offsetX: e.x - t.x,
      offsetY: e.y - t.y,
    };

    interval = setInterval(() => {
      updatePosition({ x, y });
    }, dt);
  }
  function onmouseup(_: MouseEvent) {
    drag.grabbed = false;
    clearInterval(interval);
  }
</script>

<svelte:window {onmouseup} />

{#if $_message[0]?.message != undefined && x != undefined}
  <div
    class="animate"
    style:--x={`${x}px`}
    style:--y={`${y}px`}
    style:--dt={`${dt}ms`}
    role="button"
    tabindex="0"
    {onmousemove}
    {onmousedown}
  >
    <textarea
      bind:value={
        () => message,
        (v) => {
          message = v;
          if (message != undefined) {
            updateMessage({
              message,
              timestamp: Timestamp.now(),
            });
          }
        }
      }
    ></textarea>
  </div>
{/if}

<style>
  textarea {
    width: 100%;
    height: 100%;
    background: transparent;
    border: none;
    line-height: 1.4;
    font-size: 14px;
    font-family: monospace;
    resize: none;
    outline: none;
    padding: 15px;
  }

  div {
    position: relative;
    width: 230px;
    height: 230px;

    top: var(--y);
    left: var(--x);

    background-color: #ffeb3b;
    box-shadow: 1px 1px 3px rgba(0, 0, 0, 0.2);

    overflow: hidden;

    transition:
      top max(var(--dt), 150ms) linear,
      left max(var(--dt), 150ms) linear,
      transform 0.2s ease,
      box-shadow 0.2s ease,
      border-radius 0.2s ease-in-out;
  }
  div:hover {
    box-shadow: 1px 1px 5px rgba(0, 0, 0, 0.3);
    transform: rotate(0.4deg);
    border-radius: 0 0 20px 0;
  }

  div::before {
    content: "";
    position: absolute;
    bottom: 0;
    right: 0;

    display: block;
    width: 40px;
    height: 40px;

    background: linear-gradient(
      135deg,
      rgba(0, 0, 0, 0) 0%,
      rgba(0, 0, 0, 0) 50%,
      rgba(0, 0, 0, 0.15) 100%
    );
    transition: background 0.2s ease;

    clip-path: polygon(100% 0%, 100% 100%, 0% 100%);

    pointer-events: none;
  }

  div:hover::before {
    background: linear-gradient(
      135deg,
      rgba(0, 0, 0, 0) 0%,
      rgba(0, 0, 0, 0) 50%,
      rgba(0, 0, 0, 0.3) 100%
    );
  }
</style>
