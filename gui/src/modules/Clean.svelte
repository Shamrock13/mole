<script>
  // CLEAN — "Rainwater clears the soil."
  // Ten cache categories sorted by safety tier. Every destructive
  // action goes through a review sheet listing exact paths and byte
  // counts; categories default to the conservative selection (only
  // "safe" tier pre-checked). Protected system paths render as
  // blocked rows that cannot be selected.
  import { formatBytes } from "../lib/format.js";

  const TIERS = {
    safe: { label: "Safe", color: "var(--ok)" },
    review: { label: "Review", color: "var(--warn)" },
    caution: { label: "Caution", color: "var(--danger)" },
  };

  // Mock inventory shaped like `mo clean --dry-run` output. The Tauri
  // backend will populate this from the real scanners.
  let categories = $state([
    cat("Xcode build products", "safe", [
      f("~/Library/Developer/Xcode/DerivedData/MyApp-abcd", 4.2e9),
      f("~/Library/Developer/Xcode/DerivedData/Tool-ef01", 1.9e9),
    ]),
    cat("Homebrew downloads", "safe", [f("~/Library/Caches/Homebrew/downloads", 1.4e9)]),
    cat("npm / pnpm cache", "safe", [f("~/.npm/_cacache", 2.1e9)]),
    cat("Browser temp files", "safe", [
      f("~/Library/Caches/com.apple.Safari/WebKitCache", 6.4e8),
      f("~/Library/Caches/Google/Chrome/Default/Cache", 8.9e8),
    ]),
    cat("System log archives", "safe", [f("~/Library/Logs/DiagnosticReports", 2.2e8)]),
    cat("App caches (inactive apps)", "review", [
      f("~/Library/Caches/com.spotify.client", 7.1e8),
      f("~/Library/Caches/us.zoom.xos", 3.3e8),
    ]),
    cat("iOS device support", "review", [f("~/Library/Developer/Xcode/iOS DeviceSupport/17.5", 3.8e9)]),
    cat("Old simulator runtimes", "review", [f("~/Library/Developer/CoreSimulator/Caches", 2.7e9)]),
    cat("Mail attachment cache", "caution", [f("~/Library/Mail/V10/MailData/Attachments", 1.1e9)]),
    cat("Time Machine local snapshots", "caution", [f("/Volumes/.timemachine (local snapshots)", 9.6e9)]),
  ]);

  // Path-refusal demo: anything Mole's should_protect_path() would
  // reject is shown but visually blocked.
  const blockedPaths = [
    "/System/Library/Caches",
    "/Library/Apple",
  ];

  let confirming = $state(false);

  function cat(name, tier, files) {
    return {
      name,
      tier,
      files: files.map((file) => ({ ...file, selected: tier === "safe" })),
    };
  }
  function f(path, bytes) {
    return { path, bytes };
  }

  let selected = $derived(
    categories.flatMap((c) => c.files.filter((file) => file.selected))
  );
  let selectedBytes = $derived(selected.reduce((sum, file) => sum + file.bytes, 0));

  let tierOrder = { safe: 0, review: 1, caution: 2 };
  let sorted = $derived([...categories].sort((a, b) => tierOrder[a.tier] - tierOrder[b.tier]));

  function runClean() {
    // Hook point: invoke("mole_clean", { paths: selected.map(f => f.path) })
    confirming = false;
  }
</script>

<section aria-labelledby="clean-title">
  <header class="page-head">
    <div>
      <h1 id="clean-title">Clean</h1>
      <p class="subtitle">Review every path before anything is removed. Cache purges are permanent; safest tiers are pre-selected.</p>
    </div>
  </header>

  {#each sorted as category (category.name)}
    <article class="glass group" aria-label={category.name}>
      <header class="group-head">
        <h2>{category.name}</h2>
        <span class="tier" style:color={TIERS[category.tier].color}>
          {TIERS[category.tier].label}
        </span>
      </header>
      <ul>
        {#each category.files as file (file.path)}
          <li>
            <label>
              <input type="checkbox" bind:checked={file.selected} />
              <span class="path mono">{file.path}</span>
              <span class="bytes mono">{formatBytes(file.bytes)}</span>
            </label>
          </li>
        {/each}
      </ul>
    </article>
  {/each}

  <article class="glass group blocked-group" aria-label="Protected paths">
    <header class="group-head">
      <h2>Protected by Mole</h2>
      <span class="tier" style:color="var(--danger)">Blocked</span>
    </header>
    <ul>
      {#each blockedPaths as path (path)}
        <li class="blocked">
          <span aria-hidden="true">⌀</span>
          <span class="path mono">{path}</span>
          <span class="bytes">system-critical, never touched</span>
        </li>
      {/each}
    </ul>
  </article>

  <footer class="actionbar glass-raised">
    <p>
      <strong class="mono">{selected.length}</strong> items ·
      <strong class="mono">{formatBytes(selectedBytes)}</strong> reclaimable
    </p>
    <button
      class="primary"
      disabled={selected.length === 0}
      onclick={() => (confirming = true)}
    >
      Review &amp; Clean…
    </button>
  </footer>

  {#if confirming}
    <div class="scrim" role="presentation">
      <div
        class="sheet glass-raised"
        role="alertdialog"
        aria-modal="true"
        aria-labelledby="confirm-title"
        aria-describedby="confirm-desc"
      >
        <h2 id="confirm-title">Permanently remove {selected.length} items?</h2>
        <p id="confirm-desc">
          These caches are deleted permanently, not moved to Trash.
          Exact paths and sizes:
        </p>
        <ul class="confirm-list glass-inset">
          {#each selected as file (file.path)}
            <li>
              <span class="path mono">{file.path}</span>
              <span class="bytes mono">{formatBytes(file.bytes)}</span>
            </li>
          {/each}
        </ul>
        <div class="sheet-actions">
          <button onclick={() => (confirming = false)}>Cancel</button>
          <button class="primary danger" onclick={runClean}>
            Remove {formatBytes(selectedBytes)}
          </button>
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
  .group {
    padding: var(--space-4);
    margin-bottom: var(--space-3);
  }
  .group-head {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: var(--space-3);
  }
  .group-head h2 {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
  }
  .tier {
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-caps);
    text-transform: uppercase;
  }
  ul {
    list-style: none;
    padding: 0;
  }
  li label {
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: var(--space-3);
    align-items: center;
    padding: var(--space-2) var(--space-2);
    border-radius: var(--radius-sm);
    cursor: pointer;
  }
  li label:hover {
    background: var(--glass-inset);
  }
  input[type="checkbox"] {
    accent-color: var(--accent);
  }
  .path {
    font-size: var(--text-xs);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .bytes {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
  }
  .blocked {
    display: grid;
    grid-template-columns: auto 1fr auto;
    gap: var(--space-3);
    padding: var(--space-2);
    color: var(--ink-tertiary);
  }
  .actionbar {
    position: sticky;
    bottom: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--space-3) var(--space-4);
    margin-top: var(--space-4);
  }
  .primary {
    background: var(--accent);
    color: #fff;
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-pill);
  }
  .primary:disabled {
    opacity: 0.4;
    cursor: default;
  }
  .primary.danger {
    background: var(--danger);
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
    width: min(560px, calc(100vw - 64px));
    max-height: 70vh;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-5);
  }
  .sheet h2 {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
  }
  .confirm-list {
    overflow-y: auto;
    padding: var(--space-3);
  }
  .confirm-list li {
    display: flex;
    justify-content: space-between;
    gap: var(--space-3);
    padding: var(--space-1) 0;
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
