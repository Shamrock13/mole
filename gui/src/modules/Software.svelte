<script>
  // SOFTWARE — "Red dust covers what you've outgrown."
  // Real installed-app inventory from `mo uninstall --list` (a
  // guaranteed read-only path). Uninstalling stays in the terminal
  // where Mole's interactive confirmation flow lives; each row shows
  // the exact command.
  import { onMount } from "svelte";
  import { moleCliPath, listInstalledApps, inTauri } from "../lib/mole.js";
  import CliNotice from "../lib/components/CliNotice.svelte";

  let cliPath = $state(undefined);
  let apps = $state([]);
  let loading = $state(false);
  let error = $state(null);
  let query = $state("");
  let expanded = $state(null);

  let filtered = $derived(
    query.trim()
      ? apps.filter((app) =>
          `${app.name} ${app.bundle_id}`.toLowerCase().includes(query.trim().toLowerCase())
        )
      : apps
  );

  async function refresh() {
    loading = true;
    error = null;
    try {
      const list = await listInstalledApps();
      apps = [...list].sort((a, b) => a.name.localeCompare(b.name));
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    cliPath = inTauri() ? await moleCliPath() : null;
    if (cliPath) refresh();
  });
</script>

<section aria-labelledby="software-title">
  <header class="page-head">
    <div>
      <h1 id="software-title">Software</h1>
      <p class="subtitle">
        Every installed app with its size and source. Uninstall from the
        terminal to keep Mole's interactive leftover review.
      </p>
    </div>
    {#if cliPath}
      <div class="tools">
        <input
          type="search"
          placeholder="Search apps"
          bind:value={query}
          aria-label="Search installed apps"
        />
        <button class="pill" disabled={loading} onclick={refresh}>
          {loading ? "Scanning…" : "Refresh"}
        </button>
      </div>
    {/if}
  </header>

  {#if cliPath === undefined}
    <p class="quiet">Checking for the Mole CLI…</p>
  {:else if cliPath === null}
    <CliNotice feature="The app inventory" />
  {:else if error}
    <article class="glass panel">
      <p class="quiet">Could not list applications: {error}</p>
    </article>
  {:else if loading && apps.length === 0}
    <p class="quiet">Scanning /Applications…</p>
  {:else}
    <article class="glass panel" aria-label="Installed applications">
      <header class="list-head caption">
        <span>{filtered.length} of {apps.length} apps</span>
        <span>Size</span>
      </header>
      <ul>
        {#each filtered as app (app.path)}
          <li>
            <button
              class="row"
              class:open={expanded === app.path}
              onclick={() => (expanded = expanded === app.path ? null : app.path)}
              aria-expanded={expanded === app.path}
            >
              <span class="name">{app.name}</span>
              <span class="source" class:brew={app.source === "Homebrew"}>{app.source}</span>
              <span class="size mono">{app.size}</span>
            </button>
            {#if expanded === app.path}
              <div class="detail glass-inset">
                {#if app.bundle_id}
                  <p><span class="caption">Bundle</span> <span class="mono sel">{app.bundle_id}</span></p>
                {/if}
                <p><span class="caption">Path</span> <span class="mono sel">{app.path}</span></p>
                <p>
                  <span class="caption">Uninstall</span>
                  <code class="mono sel">mo uninstall "{app.uninstall_name || app.name}"</code>
                </p>
              </div>
            {/if}
          </li>
        {:else}
          <li class="quiet empty">No apps match “{query}”.</li>
        {/each}
      </ul>
    </article>
  {/if}
</section>

<style>
  .page-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: var(--space-4);
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
    max-width: 48ch;
  }
  .tools {
    display: flex;
    gap: var(--space-2);
    align-items: center;
    flex-shrink: 0;
  }
  input[type="search"] {
    background: var(--glass-inset);
    border: none;
    border-radius: var(--radius-pill);
    padding: var(--space-2) var(--space-3);
    font: inherit;
    font-size: var(--text-sm);
    color: var(--ink-primary);
    width: 200px;
  }
  .pill {
    background: var(--accent-soft);
    color: var(--accent);
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-pill);
  }
  .pill:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .quiet {
    color: var(--ink-tertiary);
    font-size: var(--text-sm);
  }
  .panel {
    padding: var(--space-4);
  }
  .list-head {
    display: flex;
    justify-content: space-between;
    padding: 0 var(--space-2) var(--space-2);
  }
  ul {
    list-style: none;
    padding: 0;
  }
  .row {
    display: grid;
    grid-template-columns: 1fr auto 84px;
    gap: var(--space-3);
    align-items: center;
    width: 100%;
    text-align: left;
    padding: var(--space-2);
    border-radius: var(--radius-sm);
  }
  .row:hover,
  .row.open {
    background: var(--glass-inset);
  }
  .name {
    font-weight: var(--weight-medium);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .source {
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
  }
  .source.brew {
    color: var(--accent);
  }
  .size {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
    text-align: right;
  }
  .detail {
    margin: var(--space-1) var(--space-2) var(--space-2);
    padding: var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .detail p {
    display: flex;
    gap: var(--space-3);
    align-items: baseline;
    font-size: var(--text-xs);
  }
  .detail .caption {
    width: 72px;
    flex-shrink: 0;
  }
  .sel {
    user-select: text;
    -webkit-user-select: text;
    word-break: break-all;
  }
  .empty {
    padding: var(--space-3);
  }
</style>
