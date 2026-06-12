<script>
  // SETTINGS — "Small dials, steady burrow."
  // Menu bar monitor controls. Config lives in localStorage and is
  // mirrored to the Rust tray state via set_tray_config.
  import { loadTrayConfig, saveTrayConfig, applyTrayConfig } from "../lib/tray.js";

  let config = $state(loadTrayConfig());

  async function update(patch) {
    config = { ...config, ...patch };
    saveTrayConfig(config);
    try {
      await applyTrayConfig(config);
    } catch {
      // Outside Tauri (browser dev) there is no tray to update.
    }
  }
</script>

<section aria-labelledby="settings-title">
  <header class="page-head">
    <h1 id="settings-title">Settings</h1>
    <p class="subtitle">Menu bar monitor and app behavior.</p>
  </header>

  <div class="glass panel">
    <h2 class="caption">Menu Bar</h2>

    <div class="row">
      <div>
        <h3>Menu Bar Monitor</h3>
        <p>Show Mole in the menu bar. Click the icon for live metrics.</p>
      </div>
      <button
        class="toggle"
        class:on={config.enabled}
        role="switch"
        aria-checked={config.enabled}
        aria-label="Menu bar monitor"
        onclick={() => update({ enabled: !config.enabled })}
      >
        <span class="knob"></span>
      </button>
    </div>

    <div class="row" class:disabled={!config.enabled}>
      <div>
        <h3>Display</h3>
        <p>How the menu bar item looks.</p>
      </div>
      <div class="segment" role="radiogroup" aria-label="Menu bar display">
        <button
          role="radio"
          aria-checked={config.display === "icon"}
          class:selected={config.display === "icon"}
          disabled={!config.enabled}
          onclick={() => update({ display: "icon" })}
        >
          Icon
        </button>
        <button
          role="radio"
          aria-checked={config.display === "metrics"}
          class:selected={config.display === "metrics"}
          disabled={!config.enabled}
          onclick={() => update({ display: "metrics" })}
        >
          CPU %
        </button>
      </div>
    </div>

    <div class="row" class:disabled={!config.enabled}>
      <div>
        <h3>Closing the window</h3>
        <p>
          While the menu bar monitor is on, closing the main window keeps Mole
          running in the menu bar. Use the menu bar item to quit.
        </p>
      </div>
    </div>
  </div>
</section>

<style>
  .page-head { margin-bottom: var(--space-5); }
  h1 { font-size: var(--text-xl); font-weight: var(--weight-bold); }
  .subtitle { font-size: var(--text-sm); color: var(--ink-secondary); margin-top: var(--space-1); }
  .panel {
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: 640px;
  }
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
  }
  .row.disabled { opacity: 0.45; }
  .row h3 { font-size: var(--text-md); font-weight: var(--weight-semibold); }
  .row p { font-size: var(--text-xs); color: var(--ink-secondary); margin-top: var(--space-1); }
  .toggle {
    width: 44px;
    height: 26px;
    border-radius: var(--radius-pill);
    background: var(--ink-tertiary);
    position: relative;
    flex-shrink: 0;
    transition: background var(--duration-fast) var(--ease-out);
  }
  .toggle.on { background: var(--accent); }
  .knob {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 20px;
    height: 20px;
    border-radius: var(--radius-pill);
    background: #fff;
    transition: transform var(--duration-fast) var(--ease-out);
  }
  .toggle.on .knob { transform: translateX(18px); }
  .segment {
    display: flex;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-pill);
    background: var(--accent-soft);
    flex-shrink: 0;
  }
  .segment button {
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    padding: var(--space-1) var(--space-3);
    border-radius: var(--radius-pill);
    color: var(--ink-secondary);
  }
  .segment button.selected {
    background: var(--accent);
    color: #fff;
  }
  .segment button:disabled { cursor: default; }
</style>
