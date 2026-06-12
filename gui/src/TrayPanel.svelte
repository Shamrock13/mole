<script>
  // Menu bar popover: a compact, glanceable slice of the STATUS
  // dashboard. Shares the metrics store with the main window, so the
  // data path (mole-status --json --fast) is identical. Polling pauses
  // automatically while the popover is hidden (document.hidden).
  import { metrics } from "./lib/metrics.js";
  import Sparkline from "./lib/components/Sparkline.svelte";
  import {
    formatBytes,
    formatPercent,
    formatRate,
    formatUptime,
  } from "./lib/format.js";

  let m = $derived($metrics);

  async function invoke(cmd) {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke(cmd);
  }
</script>

<div class="panel">
  <header class="head">
    <p class="brand caption">Mole</p>
    <p class="live caption">
      <span class="dot pulse" class:idle={!m.live} aria-hidden="true"></span>
      {m.live ? "live" : "connecting"}
    </p>
  </header>

  <div class="chips">
    <span class="chip mono">{m.hardware.model || "Mac"}</span>
    {#if m.memory.total}
      <span class="chip mono">{formatBytes(m.memory.total)}</span>
    {/if}
    {#if m.hardware.os_version}
      <span class="chip mono">{m.hardware.os_version}</span>
    {/if}
    <span class="chip mono">up {formatUptime(m.uptimeSeconds)}</span>
  </div>

  <div class="grid">
    <article class="glass tile">
      <header><h3 class="caption">CPU</h3><span class="badge mono">{m.cpu.load1.toFixed(1)} load</span></header>
      <p class="value mono">{formatPercent(m.cpu.usage)}</p>
      <Sparkline values={m.cpu.history} label="CPU" width={150} height={24} />
    </article>

    <article class="glass tile">
      <header><h3 class="caption">GPU</h3></header>
      <p class="value mono">{formatPercent(m.gpu.usage)}</p>
      <Sparkline values={m.gpu.history} label="GPU" width={150} height={24} />
    </article>

    <article class="glass tile">
      <header><h3 class="caption">MEM</h3><span class="badge mono">{formatBytes(m.memory.used)}</span></header>
      <p class="value mono">{formatPercent(m.memory.usedPercent)}</p>
      <Sparkline values={m.memory.history} label="Memory" width={150} height={24} />
    </article>

    <article class="glass tile">
      <header><h3 class="caption">DISK</h3><span class="badge mono">{formatBytes(m.disk.total)}</span></header>
      <p class="value mono">{formatPercent(m.disk.usedPercent)}</p>
      <p class="detail">{formatBytes(Math.max(0, m.disk.total - m.disk.used))} free</p>
    </article>
  </div>

  <article class="glass wide">
    <header><h3 class="caption">NET</h3>
      <span class="badge mono">↓ {formatRate(m.network.rxRate)} · ↑ {formatRate(m.network.txRate)}</span>
    </header>
    <Sparkline
      values={m.network.history}
      max={Math.max(2, ...m.network.history.filter((v) => v != null))}
      label="Network"
      width={330}
      height={26}
    />
  </article>

  {#if m.battery.percent != null}
    <article class="glass wide battery">
      <header><h3 class="caption">Battery</h3>
        {#if m.battery.health}<span class="badge mono">{m.battery.health}</span>{/if}
      </header>
      <div class="battery-row">
        <p class="value mono">{formatPercent(m.battery.percent)}</p>
        <p class="detail">{m.battery.charging ? "Charging" : "On battery"}</p>
      </div>
    </article>
  {/if}

  <article class="glass wide procs">
    <header><h3 class="caption">Top Processes</h3>
      <span class="badge mono">CPU · Mem</span>
    </header>
    <ul>
      {#each m.topProcesses.slice(0, 5) as proc (proc.pid)}
        <li>
          <span class="proc-name">{proc.name}</span>
          <span class="proc-cpu mono">{proc.cpu.toFixed(1)}%</span>
          <span class="proc-mem mono">{proc.memory.toFixed(1)}%</span>
        </li>
      {:else}
        <li class="empty">Collecting…</li>
      {/each}
    </ul>
  </article>

  <footer class="actions">
    <button class="action" onclick={() => invoke("show_main_window")}>Open Mole</button>
    <button class="action quiet" onclick={() => invoke("quit_app")}>Quit</button>
  </footer>
</div>

<style>
  .panel {
    height: 100vh;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4);
    overflow-y: auto;
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .brand { letter-spacing: var(--tracking-tight); }
  .live { display: flex; align-items: center; gap: var(--space-2); }
  .dot {
    width: 7px;
    height: 7px;
    border-radius: var(--radius-pill);
    background: var(--accent);
    box-shadow: 0 0 8px var(--accent-glow);
  }
  .dot.idle { background: var(--ink-tertiary); box-shadow: none; }
  .chips { display: flex; flex-wrap: wrap; gap: var(--space-2); }
  .chip {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
    background: var(--accent-soft);
    padding: 2px var(--space-2);
    border-radius: var(--radius-pill);
  }
  .grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-3);
  }
  .tile, .wide {
    padding: var(--space-3);
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    min-width: 0;
  }
  .tile header, .wide header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
  }
  .badge {
    font-size: var(--text-xs);
    color: var(--ink-secondary);
    white-space: nowrap;
  }
  .value {
    font-size: var(--text-lg);
    font-weight: var(--weight-semibold);
    letter-spacing: var(--tracking-tight);
  }
  .detail { font-size: var(--text-xs); color: var(--ink-secondary); }
  .tile :global(svg), .wide :global(svg) { width: 100%; }
  .battery-row {
    display: flex;
    align-items: baseline;
    gap: var(--space-3);
  }
  .procs ul {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .procs li {
    display: grid;
    grid-template-columns: 1fr 52px 52px;
    gap: var(--space-2);
    font-size: var(--text-xs);
  }
  .proc-name { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
  .proc-cpu { color: var(--accent); text-align: right; }
  .proc-mem { color: var(--ink-secondary); text-align: right; }
  .empty { color: var(--ink-tertiary); }
  .actions {
    margin-top: auto;
    display: flex;
    gap: var(--space-2);
  }
  .action {
    flex: 1;
    background: var(--accent);
    color: #fff;
    font-size: var(--text-xs);
    font-weight: var(--weight-semibold);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-pill);
  }
  .action.quiet {
    background: var(--accent-soft);
    color: var(--ink-secondary);
  }
</style>
