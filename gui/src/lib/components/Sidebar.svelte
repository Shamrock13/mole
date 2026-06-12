<script>
  // Floating glass sidebar. Semantic <nav>, full keyboard support
  // (buttons are natively tabbable; aria-current marks the page).
  let { modules = [], current, onNavigate } = $props();
</script>

<nav class="glass sidebar" aria-label="Mole modules">
  <div class="brand">
    <span class="logo" aria-hidden="true">⛰</span>
    <span class="name">Mole</span>
  </div>

  <ul>
    {#each modules as mod (mod.id)}
      <li>
        <button
          class="item"
          class:active={current === mod.id}
          aria-current={current === mod.id ? "page" : undefined}
          onclick={() => onNavigate(mod.id)}
        >
          <span class="icon" aria-hidden="true">{mod.icon}</span>
          <span class="label">
            <span class="title">{mod.title}</span>
            <span class="tagline">{mod.tagline}</span>
          </span>
        </button>
      </li>
    {/each}
  </ul>

  <footer class="caption">100% local · no telemetry</footer>
</nav>

<style>
  .sidebar {
    width: 232px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    padding: var(--space-4) var(--space-3);
    margin: var(--space-4);
    margin-right: 0;
  }
  .brand {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    /* Top padding clears the traffic lights, which the overlay
       titlebar places over the sidebar's top-left corner. */
    padding: 36px var(--space-3) var(--space-5);
    font-weight: var(--weight-bold);
    font-size: var(--text-md);
  }
  .logo {
    filter: saturate(0);
  }
  ul {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .item {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-md);
    text-align: left;
    color: var(--ink-secondary);
    transition: background var(--duration-fast) var(--ease-out);
  }
  .item:hover {
    background: var(--glass-inset);
  }
  .item.active {
    background: var(--accent-soft);
    color: var(--ink-primary);
  }
  .item.active .icon {
    color: var(--accent);
  }
  .icon {
    font-size: var(--text-md);
    width: 20px;
    text-align: center;
  }
  .label {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .title {
    font-weight: var(--weight-medium);
    font-size: var(--text-sm);
  }
  .tagline {
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
  footer {
    margin-top: auto;
    padding: var(--space-3);
  }
</style>
