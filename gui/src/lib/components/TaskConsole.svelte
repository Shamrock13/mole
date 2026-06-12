<script>
  // Live output pane for streamed CLI tasks. Sticks to the bottom as
  // lines arrive, like a terminal.
  let { lines = [], running = false } = $props();
  let pane = $state(null);

  $effect(() => {
    void lines.length;
    if (pane) pane.scrollTop = pane.scrollHeight;
  });
</script>

{#if lines.length || running}
  <div class="console glass-inset" bind:this={pane} role="log" aria-live="polite">
    {#each lines as line, i (i)}
      <p class="mono">{line}</p>
    {/each}
    {#if running}
      <p class="mono caret" aria-hidden="true">▌</p>
    {/if}
  </div>
{/if}

<style>
  .console {
    margin-top: var(--space-3);
    padding: var(--space-3) var(--space-4);
    max-height: 46vh;
    overflow-y: auto;
    user-select: text;
    -webkit-user-select: text;
  }
  .console p {
    font-size: var(--text-xs);
    line-height: 1.7;
    white-space: pre-wrap;
    word-break: break-word;
  }
  .caret {
    color: var(--accent);
    animation: gentle-pulse var(--pulse-period) var(--ease-out) infinite;
  }
</style>
