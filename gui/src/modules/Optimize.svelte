<script>
  // OPTIMIZE — "Closest orbit, swiftest run."
  // Single-tap maintenance tasks mapped to lib/optimize/tasks.sh.
  // Running tasks show the gentle pulse; nothing here deletes files.
  const TASKS = [
    { id: "quicklook", name: "Rebuild Quick Look", detail: "Reset thumbnail and preview generation" },
    { id: "caches", name: "Repair caches & metadata", detail: "Spotlight, LaunchServices, dyld caches" },
    { id: "login", name: "Audit login items", detail: "Find slow or orphaned startup entries" },
    { id: "batch", name: "Batch admin workflow", detail: "Run all maintenance tasks in safe order" },
  ];

  let running = $state({});

  function run(id) {
    // Hook point: invoke("mole_optimize", { task: id })
    running[id] = true;
    setTimeout(() => (running[id] = false), 2600);
  }
</script>

<section aria-labelledby="optimize-title">
  <header class="page-head">
    <h1 id="optimize-title">Optimize</h1>
    <p class="subtitle">Maintenance only. These tasks rebuild and repair; they never remove your data.</p>
  </header>

  <div class="tasks">
    {#each TASKS as task (task.id)}
      <article class="glass task">
        <div>
          <h2>{task.name}</h2>
          <p>{task.detail}</p>
        </div>
        <button
          class="go"
          class:pulse={running[task.id]}
          disabled={running[task.id]}
          onclick={() => run(task.id)}
          aria-label={running[task.id] ? `${task.name} running` : `Run ${task.name}`}
        >
          {running[task.id] ? "Running" : "Run"}
        </button>
      </article>
    {/each}
  </div>
</section>

<style>
  .page-head { margin-bottom: var(--space-5); }
  h1 { font-size: var(--text-xl); font-weight: var(--weight-bold); }
  .subtitle { font-size: var(--text-sm); color: var(--ink-secondary); margin-top: var(--space-1); }
  .tasks {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: var(--space-4);
  }
  .task {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-4);
  }
  .task h2 { font-size: var(--text-md); font-weight: var(--weight-semibold); }
  .task p { font-size: var(--text-xs); color: var(--ink-secondary); margin-top: var(--space-1); }
  .go {
    background: var(--accent);
    color: #fff;
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-pill);
    flex-shrink: 0;
  }
  .go:disabled { cursor: default; }
</style>
