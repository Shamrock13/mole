<script>
  // Shown when no system-wide `mo` exists. The DMG only bundles the
  // status collector; everything that cleans or changes the system
  // runs through the audited CLI.
  let { feature = "This module" } = $props();
</script>

<article class="glass notice">
  <h2>Mole CLI required</h2>
  <p>
    {feature} runs through Mole's command-line pipeline, with its path
    protection, whitelists, and operation log. Install it once:
  </p>
  <code class="mono">brew install mole</code>
  <p class="alt">
    or
    <code class="mono inline">curl -fsSL https://raw.githubusercontent.com/tw93/mole/main/install.sh | bash</code>
  </p>
</article>

<style>
  .notice {
    padding: var(--space-5);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    max-width: 640px;
  }
  .notice h2 {
    font-size: var(--text-md);
    font-weight: var(--weight-semibold);
  }
  .notice p {
    font-size: var(--text-sm);
    color: var(--ink-secondary);
  }
  code {
    display: inline-block;
    background: var(--glass-inset);
    border-radius: var(--radius-sm);
    padding: var(--space-2) var(--space-3);
    font-size: var(--text-xs);
    user-select: text;
    -webkit-user-select: text;
  }
  code.inline {
    padding: 2px var(--space-2);
  }
  .alt {
    font-size: var(--text-xs);
    color: var(--ink-tertiary);
  }
</style>
