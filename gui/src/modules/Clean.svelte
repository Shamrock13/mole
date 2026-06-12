<script>
  // CLEAN — "Rainwater clears the soil."
  // Runs the real `mo clean` pipeline and streams its output. The GUI
  // never deletes anything itself: previews are `mo clean --dry-run`,
  // and the actual clean is the same audited shell flow the terminal
  // uses (whitelists, protected paths, operation log included).
  import { onMount, onDestroy } from "svelte";
  import { moleCliPath, createTaskRunner, inTauri } from "../lib/mole.js";
  import TaskConsole from "../lib/components/TaskConsole.svelte";
  import CliNotice from "../lib/components/CliNotice.svelte";

  const runner = createTaskRunner();
  const { lines, running } = runner;

  let cliPath = $state(undefined); // undefined = checking, null = missing
  let confirming = $state(false);
  let lastTask = $state(null);

  onMount(async () => {
    cliPath = inTauri() ? await moleCliPath() : null;
  });
  onDestroy(() => runner.destroy());

  function preview() {
    lastTask = "preview";
    runner.start("clean-preview");
  }

  function cleanNow() {
    confirming = false;
    lastTask = "clean";
    runner.start("clean");
  }
</script>

<section aria-labelledby="clean-title">
  <header class="page-head">
    <div>
      <h1 id="clean-title">Clean</h1>
      <p class="subtitle">
        Deep cleanup of caches, logs, and leftovers through Mole's safety
        pipeline. Preview first; nothing is removed until you confirm.
      </p>
    </div>
  </header>

  {#if cliPath === undefined}
    <p class="checking">Checking for the Mole CLI…</p>
  {:else if cliPath === null}
    <CliNotice feature="Cleaning" />
  {:else}
    <article class="glass panel">
      <div class="actions">
        <button class="primary ghost" disabled={$running} onclick={preview}>
          {$running && lastTask === "preview" ? "Previewing…" : "Preview (dry run)"}
        </button>
        <button class="primary" disabled={$running} onclick={() => (confirming = true)}>
          Clean now…
        </button>
        {#if $running}
          <button class="stop" onclick={() => runner.cancel()}>Stop</button>
        {/if}
      </div>
      <p class="hint">
        Runs <code class="mono">{cliPath}</code>. System-level caches need
        admin rights and are skipped here; run
        <code class="mono">sudo mo clean</code> in a terminal for those.
      </p>
      <TaskConsole lines={$lines} running={$running} />
    </article>
  {/if}

  {#if confirming}
    <div class="scrim" role="presentation">
      <div
        class="sheet glass-raised"
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="confirm-title"
        aria-describedby="confirm-desc"
      >
        <h2 id="confirm-title">Run cleanup now?</h2>
        <p id="confirm-desc">
          This runs <code class="mono">mo clean</code>: user-level caches and
          logs are removed permanently, honoring your whitelist and Mole's
          protected paths. Every removal is recorded in the operation log
          (<code class="mono">mo history</code>).
        </p>
        <div class="sheet-actions">
          <button onclick={() => (confirming = false)}>Cancel</button>
          <button class="primary" onclick={cleanNow}>Clean</button>
        </div>
      </div>
    </div>
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
  .checking {
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
  .primary.ghost {
    background: var(--accent-soft);
    color: var(--accent);
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
  .scrim {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.25);
    display: grid;
    place-items: center;
    z-index: 10;
  }
  .sheet {
    width: min(520px, calc(100vw - 64px));
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-5);
  }
  .sheet h2 {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
  }
  .sheet p {
    font-size: var(--text-sm);
    color: var(--ink-secondary);
  }
  .sheet-actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
  }
  .sheet-actions button {
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-pill);
  }
</style>
