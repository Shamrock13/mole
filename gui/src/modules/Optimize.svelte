<script>
  // OPTIMIZE — "Closest orbit, swiftest run."
  // Streams `mo optimize --dry-run`: real health checks and the
  // maintenance recommendations Mole would apply. Applying them stays
  // in the terminal, where the interactive task picker (and sudo,
  // when needed) lives.
  import { onMount, onDestroy } from "svelte";
  import { moleCliPath, createTaskRunner, inTauri } from "../lib/mole.js";
  import TaskConsole from "../lib/components/TaskConsole.svelte";
  import CliNotice from "../lib/components/CliNotice.svelte";

  const runner = createTaskRunner();
  const { lines, running } = runner;

  let cliPath = $state(undefined);

  onMount(async () => {
    cliPath = inTauri() ? await moleCliPath() : null;
  });
  onDestroy(() => runner.destroy());
</script>

<section aria-labelledby="optimize-title">
  <header class="page-head">
    <h1 id="optimize-title">Optimize</h1>
    <p class="subtitle">
      System health checks and maintenance recommendations. The preview is
      read-only; apply tasks with <code class="mono">mo optimize</code> in a
      terminal.
    </p>
  </header>

  {#if cliPath === undefined}
    <p class="quiet">Checking for the Mole CLI…</p>
  {:else if cliPath === null}
    <CliNotice feature="Optimization" />
  {:else}
    <article class="glass panel">
      <div class="actions">
        <button class="primary" disabled={$running} onclick={() => runner.start("optimize-preview")}>
          {$running ? "Checking…" : "Check system health"}
        </button>
        {#if $running}
          <button class="stop" onclick={() => runner.cancel()}>Stop</button>
        {/if}
      </div>
      <p class="hint">
        Runs <code class="mono">mo optimize --dry-run</code>: nothing is
        modified. Whitelist paths with
        <code class="mono">mo optimize --whitelist</code>.
      </p>
      <TaskConsole lines={$lines} running={$running} />
    </article>
  {/if}
</section>

<style>
  .page-head {
    margin-bottom: var(--space-5);
  }
  h1 {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
  }
  .subtitle {
    font-size: var(--text-sm);
    color: var(--ink-secondary);
    margin-top: var(--space-1);
    max-width: 56ch;
  }
  .quiet {
    color: var(--ink-tertiary);
    font-size: var(--text-sm);
  }
  .panel {
    padding: var(--space-4);
  }
  .actions {
    display: flex;
    gap: var(--space-3);
    align-items: center;
  }
  .primary {
    background: var(--accent);
    color: #fff;
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-pill);
  }
  .primary:disabled {
    opacity: 0.4;
    cursor: default;
  }
  .stop {
    color: var(--danger);
    font-size: var(--text-sm);
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-3);
  }
  .hint {
    margin-top: var(--space-3);
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
  }
  .hint code {
    user-select: text;
    -webkit-user-select: text;
  }
</style>
