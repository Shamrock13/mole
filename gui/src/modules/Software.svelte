<script>
  // SOFTWARE — "Red dust covers what you've outgrown."
  // Shell view: update vectors (Sparkle / Homebrew / App Store),
  // startup items, and an uninstall inspector that segregates each
  // leftover class. Uninstalls route to Trash (recoverable).
  const updates = [
    { app: "iTerm2", from: "3.5.1", to: "3.5.4", vector: "Sparkle" },
    { app: "ripgrep", from: "14.1.0", to: "14.1.1", vector: "Homebrew" },
    { app: "Pixelmator Pro", from: "3.6.2", to: "3.6.4", vector: "App Store" },
  ];

  const startup = [
    { name: "Raycast", kind: "Login Item", enabled: true },
    { name: "com.docker.vmnetd", kind: "Launch Daemon", enabled: true },
    { name: "Adobe Updater", kind: "Launch Agent", enabled: false },
  ];

  let inspecting = $state(null);

  const leftovers = {
    app: "OldApp.app",
    groups: [
      { label: "Preferences", paths: ["~/Library/Preferences/com.oldapp.plist"] },
      { label: "Support files", paths: ["~/Library/Application Support/OldApp"] },
      { label: "Launch agents", paths: ["~/Library/LaunchAgents/com.oldapp.helper.plist"] },
      { label: "Dock entries", paths: ["com.apple.dock persistent-apps"] },
    ],
    personalData: ["~/Documents/OldApp Exports (contains personal data, kept by default)"],
  };
</script>

<section aria-labelledby="software-title">
  <header class="page-head">
    <h1 id="software-title">Software</h1>
    <p class="subtitle">Updates, startup items, and clean uninstalls. Removals go to Trash and are recoverable.</p>
  </header>

  <article class="glass group" aria-label="Available updates">
    <h2 class="caption">Updates</h2>
    <ul>
      {#each updates as u (u.app)}
        <li class="row">
          <span class="name">{u.app}</span>
          <span class="vector">{u.vector}</span>
          <span class="ver mono">{u.from} → {u.to}</span>
          <button class="pill">Update</button>
        </li>
      {/each}
    </ul>
  </article>

  <article class="glass group" aria-label="Startup items">
    <h2 class="caption">Startup items</h2>
    <ul>
      {#each startup as s (s.name)}
        <li class="row">
          <span class="name">{s.name}</span>
          <span class="vector">{s.kind}</span>
          <label class="switch">
            <input type="checkbox" checked={s.enabled} aria-label={`${s.name} at login`} />
            <span>{s.enabled ? "On" : "Off"}</span>
          </label>
        </li>
      {/each}
    </ul>
  </article>

  <article class="glass group" aria-label="Uninstall">
    <h2 class="caption">Uninstall</h2>
    <ul>
      <li class="row">
        <span class="name">{leftovers.app}</span>
        <span class="vector">2.4 GB total</span>
        <button class="pill" onclick={() => (inspecting = leftovers)}>Inspect…</button>
      </li>
    </ul>

    {#if inspecting}
      <div class="inspector glass-inset">
        {#each inspecting.groups as group (group.label)}
          <h3 class="caption">{group.label}</h3>
          <ul class="paths">
            {#each group.paths as path (path)}
              <li class="mono">{path}</li>
            {/each}
          </ul>
        {/each}
        <h3 class="caption warn">Personal data — kept unless you opt in</h3>
        <ul class="paths">
          {#each inspecting.personalData as path (path)}
            <li class="mono">{path}</li>
          {/each}
        </ul>
        <p class="trash-note">Everything above moves to Trash, never deleted directly.</p>
      </div>
    {/if}
  </article>
</section>

<style>
  .page-head { margin-bottom: var(--space-5); }
  h1 { font-size: var(--text-xl); font-weight: var(--weight-bold); }
  .subtitle { font-size: var(--text-sm); color: var(--ink-secondary); margin-top: var(--space-1); }
  .group { padding: var(--space-4); margin-bottom: var(--space-3); }
  .group h2 { margin-bottom: var(--space-3); }
  ul { list-style: none; padding: 0; }
  .row {
    display: grid;
    grid-template-columns: 1fr auto auto auto;
    gap: var(--space-4);
    align-items: center;
    padding: var(--space-2);
    border-radius: var(--radius-sm);
  }
  .row:hover { background: var(--glass-inset); }
  .name { font-weight: var(--weight-medium); }
  .vector, .ver { font-size: var(--text-xs); color: var(--ink-secondary); }
  .pill {
    background: var(--accent-soft);
    color: var(--accent);
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
  }
  .switch { display: flex; gap: var(--space-2); font-size: var(--text-xs); align-items: center; }
  .switch input { accent-color: var(--accent); }
  .inspector { margin-top: var(--space-3); padding: var(--space-4); display: flex; flex-direction: column; gap: var(--space-2); }
  .paths li { font-size: var(--text-xs); padding: var(--space-1) 0; color: var(--ink-secondary); }
  .warn { color: var(--warn); }
  .trash-note { font-size: var(--text-xs); color: var(--ink-tertiary); }
</style>
