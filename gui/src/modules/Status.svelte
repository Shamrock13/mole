<script>
  // STATUS — "Every heartbeat, in its light."
  // Real-time dashboard fed by the metrics store (mo status --json
  // through Tauri, simulator in the browser). Ten metrics, each with
  // a 60-second sparkline; fan mode control on the thermals tile.
  import { metrics } from "../lib/metrics.js";
  import MetricCard from "../lib/components/MetricCard.svelte";
  import FanModeControl from "../lib/components/FanModeControl.svelte";
  import {
    formatBytes,
    formatPercent,
    formatRate,
    formatUptime,
  } from "../lib/format.js";

  let m = $derived($metrics);
</script>

<section aria-labelledby="status-title">
  <header class="page-head">
    <div>
      <h1 id="status-title">Status</h1>
      <p class="subtitle">
        {m.hardware.model}{m.hardware.cpu_model ? ` · ${m.hardware.cpu_model}` : ""}
      </p>
    </div>
    <p class="live caption" aria-live="polite">
      <span class="dot pulse" class:idle={!m.live} aria-hidden="true"></span>
      {m.live ? `live · ${m.source}` : "connecting"}
    </p>
  </header>

  <div class="grid">
    <MetricCard
      title="CPU"
      value={formatPercent(m.cpu.usage)}
      detail={`load ${m.cpu.load1.toFixed(2)}`}
      history={m.cpu.history}
      active={m.cpu.usage > 1}
    />

    <MetricCard
      title="Memory"
      value={formatPercent(m.memory.usedPercent)}
      detail={`${formatBytes(m.memory.used)} of ${formatBytes(m.memory.total)}`}
      history={m.memory.history}
      active
    />

    <MetricCard
      title="GPU"
      value={formatPercent(m.gpu.usage)}
      detail={m.gpu.name}
      history={m.gpu.history}
      active={m.gpu.usage > 5}
    />

    <MetricCard
      title="Disk"
      value={formatPercent(m.disk.usedPercent)}
      detail={`${formatBytes(m.disk.used)} used · R ${formatRate(m.disk.readRate)} W ${formatRate(m.disk.writeRate)}`}
      history={m.disk.history}
      max={Math.max(20, ...m.disk.history.filter((v) => v != null))}
      active={m.disk.readRate + m.disk.writeRate > 0.5}
    />

    <MetricCard
      title="Network"
      value={formatRate(m.network.rxRate + m.network.txRate)}
      detail={`↓ ${formatRate(m.network.rxRate)} · ↑ ${formatRate(m.network.txRate)}`}
      history={m.network.history}
      max={Math.max(2, ...m.network.history.filter((v) => v != null))}
      active={m.network.rxRate + m.network.txRate > 0.05}
    />

    <MetricCard
      title="Battery"
      value={m.battery.percent != null ? formatPercent(m.battery.percent) : "—"}
      detail={m.battery.percent == null
        ? "No battery"
        : m.battery.charging
          ? "Charging"
          : (m.battery.health ?? "On battery")}
      history={m.battery.history}
      active={m.battery.charging}
    />

    <MetricCard
      title="Thermals · Fans"
      value={m.thermal.cpuTemp != null ? `${Math.round(m.thermal.cpuTemp)}°` : "—"}
      detail={`pressure ${m.thermal.pressure}`}
      history={m.thermal.history}
      min={20}
      max={100}
      active={m.thermal.pressure !== "nominal"}
    >
      {#snippet children()}
        <FanModeControl supported={m.fans.supported} rpm={m.fans.rpm} />
      {/snippet}
    </MetricCard>

    <MetricCard
      title="Uptime"
      value={formatUptime(m.uptimeSeconds)}
      detail={m.hardware.os_version || ""}
      history={m.cpu.history}
      active={false}
    />

    <article class="glass card processes" aria-labelledby="proc-title">
      <header class="proc-head">
        <h3 id="proc-title" class="caption">Processes · {m.procs}</h3>
        <span class="dot pulse" aria-hidden="true"></span>
      </header>
      <ul>
        {#each m.topProcesses as proc (proc.pid)}
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
  </div>
</section>

<style>
  .page-head {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    margin-bottom: var(--space-5);
  }
  h1 {
    font-size: var(--text-xl);
    font-weight: var(--weight-bold);
    letter-spacing: var(--tracking-tight);
  }
  .subtitle {
    font-size: var(--text-sm);
    color: var(--ink-secondary);
    margin-top: var(--space-1);
  }
  .live {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding-top: var(--space-2);
  }
  .dot {
    width: 7px;
    height: 7px;
    border-radius: var(--radius-pill);
    background: var(--accent);
    box-shadow: 0 0 8px var(--accent-glow);
  }
  .dot.idle {
    background: var(--ink-tertiary);
    box-shadow: none;
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(248px, 1fr));
    gap: var(--space-4);
  }
  .processes {
    grid-column: span 2;
    padding: var(--space-4);
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .proc-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .processes ul {
    list-style: none;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .processes li {
    display: grid;
    grid-template-columns: 1fr 56px 56px;
    gap: var(--space-2);
    font-size: var(--text-xs);
  }
  .proc-name {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .proc-cpu {
    color: var(--accent);
    text-align: right;
  }
  .proc-mem {
    color: var(--ink-secondary);
    text-align: right;
  }
  .empty {
    color: var(--ink-tertiary);
  }
  @media (max-width: 720px) {
    .processes {
      grid-column: span 1;
    }
  }
</style>
