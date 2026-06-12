// Live metrics store for the STATUS dashboard.
//
// Source of truth is Mole's own Go collector: the Tauri backend runs
// `mo status --json` and this module adapts that payload. When the app
// runs outside Tauri (plain `vite dev` in a browser) a lightweight
// simulator produces the same shape so the UI stays workable.
//
// Each metric keeps a 60-sample ring buffer (1 sample per 2-second
// tick) that feeds the sparklines. Polling pauses automatically when
// the window is hidden to keep idle CPU near zero.

import { readable, writable } from "svelte/store";

const HISTORY = 60;
// Matches the --watch sampling interval in the Rust shell: the
// backend snapshot only changes every 2s, so polling faster just
// re-renders identical data.
const TICK_MS = 2000;

function ring() {
  return new Array(HISTORY).fill(null);
}

function push(buf, value) {
  buf.push(value);
  if (buf.length > HISTORY) buf.shift();
}

function emptyState() {
  return {
    live: false,
    source: "simulated",
    uptimeSeconds: 0,
    procs: 0,
    hardware: { model: "Mac", cpu_model: "", os_version: "" },
    cpu: { usage: 0, load1: 0, history: ring() },
    memory: { usedPercent: 0, used: 0, total: 0, pressure: 0, history: ring() },
    gpu: { usage: 0, name: "GPU", history: ring() },
    disk: { usedPercent: 0, used: 0, total: 0, readRate: 0, writeRate: 0, history: ring() },
    network: { rxRate: 0, txRate: 0, history: ring() },
    battery: { percent: null, charging: false, health: null, history: ring() },
    thermal: { cpuTemp: null, pressure: "nominal", history: ring() },
    fans: { supported: false, rpm: 0, mode: "auto", history: ring() },
    topProcesses: [],
  };
}

// ---------------------------------------------------------------
// Tauri bridge
// ---------------------------------------------------------------
function inTauri() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function fetchMoleStatus() {
  const { invoke } = await import("@tauri-apps/api/core");
  const raw = await invoke("mole_status");
  return JSON.parse(raw);
}

// Field names follow cmd/status/metrics.go json tags exactly.
function adaptMolePayload(state, j) {
  state.live = true;
  state.source = "mo status";
  state.uptimeSeconds = j.uptime_seconds ?? 0;
  state.procs = j.procs ?? 0;
  if (j.hardware) state.hardware = j.hardware;

  state.cpu.usage = j.cpu?.usage ?? 0;
  state.cpu.load1 = j.cpu?.load1 ?? 0;

  const mem = j.memory ?? {};
  state.memory.total = mem.total ?? 0;
  state.memory.used = mem.used ?? 0;
  state.memory.usedPercent =
    mem.used_percent ?? (mem.total ? (mem.used / mem.total) * 100 : 0);

  // The collector reports -1 while it has no GPU reading yet;
  // surface that as "no data", never as a negative percentage.
  const gpu = Array.isArray(j.gpu) && j.gpu.length ? j.gpu[0] : null;
  const gpuUsage = gpu?.usage;
  state.gpu.usage = gpuUsage != null && gpuUsage >= 0 ? gpuUsage : null;
  state.gpu.name = gpu?.name ?? "GPU";

  const disks = Array.isArray(j.disks) ? j.disks : [];
  const disk = disks.find((d) => d.mount === "/") ?? disks[0];
  if (disk) {
    state.disk.total = disk.total ?? 0;
    state.disk.used = disk.used ?? 0;
    state.disk.usedPercent =
      disk.used_percent ?? (disk.total ? (disk.used / disk.total) * 100 : 0);
  }
  state.disk.readRate = j.disk_io?.read_rate ?? 0;
  state.disk.writeRate = j.disk_io?.write_rate ?? 0;

  const nets = Array.isArray(j.network) ? j.network : [];
  state.network.rxRate = nets.reduce((sum, n) => sum + (n.rx_rate_mbs ?? 0), 0);
  state.network.txRate = nets.reduce((sum, n) => sum + (n.tx_rate_mbs ?? 0), 0);

  const bat = Array.isArray(j.batteries) && j.batteries.length ? j.batteries[0] : null;
  state.battery.percent = bat?.percent ?? null;
  // pmset status tokens: charging / discharging / charged / finishing
  state.battery.charging = /^(charging|charged|finishing)/i.test(bat?.status ?? "");
  state.battery.health = bat?.health || null;

  const cpuTemp = j.thermal?.cpu_temp ?? 0;
  state.thermal.cpuTemp = cpuTemp > 0 ? cpuTemp : null;
  state.thermal.pressure =
    cpuTemp >= 85 ? "serious" : cpuTemp >= 70 ? "warm" : "nominal";
  state.fans.supported = (j.thermal?.fan_count ?? 0) > 0;
  state.fans.rpm = j.thermal?.fan_speed ?? 0;

  // Keep everything the collector sends (the GUI requests 20); the
  // views decide how many to show collapsed vs expanded.
  state.topProcesses = j.top_processes ?? [];
}

// ---------------------------------------------------------------
// Browser-only simulator (same shape, plausible rhythms)
// ---------------------------------------------------------------
function drift(prev, target, jitter, min = 0, max = 100) {
  const next = prev + (target - prev) * 0.05 + (Math.random() - 0.5) * jitter;
  return Math.min(max, Math.max(min, next));
}

function simulate(state, t) {
  state.live = true;
  state.source = "simulated";
  state.uptimeSeconds = 86400 * 3 + t;
  state.procs = 412 + Math.round(Math.sin(t / 9) * 6);
  state.hardware = {
    model: "MacBook Pro 14-inch",
    cpu_model: "Apple M3 Pro",
    os_version: "macOS",
  };
  state.cpu.usage = drift(state.cpu.usage || 18, 22 + 14 * Math.sin(t / 17), 6);
  state.cpu.load1 = (state.cpu.usage / 100) * 8;
  state.memory.total = 18 * 1024 ** 3;
  state.memory.usedPercent = drift(state.memory.usedPercent || 58, 62, 1.5);
  state.memory.used = (state.memory.usedPercent / 100) * state.memory.total;
  state.gpu.usage = drift(state.gpu.usage || 8, 10 + 18 * Math.max(0, Math.sin(t / 31)), 5);
  state.gpu.name = "Apple M3 Pro GPU";
  state.disk.total = 994 * 1024 ** 3;
  state.disk.usedPercent = 71.4;
  state.disk.used = state.disk.usedPercent * 0.01 * state.disk.total;
  state.disk.readRate = Math.max(0, drift(state.disk.readRate || 2, 3, 4, 0, 400));
  state.disk.writeRate = Math.max(0, drift(state.disk.writeRate || 1, 1.5, 2.5, 0, 400));
  state.network.rxRate = Math.max(0, drift(state.network.rxRate || 0.4, 0.8, 1.2, 0, 120));
  state.network.txRate = Math.max(0, drift(state.network.txRate || 0.1, 0.2, 0.4, 0, 120));
  state.battery.percent = Math.max(5, 84 - t / 600);
  state.battery.charging = false;
  state.battery.health = "Normal";
  state.thermal.cpuTemp = drift(state.thermal.cpuTemp || 46, 48 + state.cpu.usage / 8, 1.2, 30, 100);
  state.thermal.pressure = state.thermal.cpuTemp > 80 ? "serious" : "nominal";
  state.fans.supported = true;
  state.fans.rpm =
    state.fans.mode === "cool" ? 4200 : state.fans.mode === "quiet" ? 1180 : Math.round(1600 + state.cpu.usage * 22);
  state.topProcesses = [
    { pid: 501, name: "WindowServer", cpu: 6.2 + Math.random() * 2, memory: 2.1 },
    { pid: 882, name: "mole", cpu: 0.3, memory: 0.2 },
    { pid: 1204, name: "Safari", cpu: 3.8 + Math.random() * 3, memory: 4.6 },
    { pid: 1731, name: "Xcode", cpu: 11.0 + Math.random() * 8, memory: 9.8 },
    { pid: 2007, name: "Music", cpu: 1.1, memory: 1.4 },
    { pid: 2188, name: "Terminal", cpu: 0.6, memory: 0.5 },
  ];
}

function recordHistory(state) {
  push(state.cpu.history, state.cpu.usage);
  push(state.memory.history, state.memory.usedPercent);
  push(state.gpu.history, state.gpu.usage);
  push(state.disk.history, state.disk.readRate + state.disk.writeRate);
  push(state.network.history, state.network.rxRate + state.network.txRate);
  push(state.battery.history, state.battery.percent);
  push(state.thermal.history, state.thermal.cpuTemp);
  push(state.fans.history, state.fans.rpm);
}

// ---------------------------------------------------------------
// Public stores
// ---------------------------------------------------------------
export const fanMode = writable("auto");

export async function setFanMode(mode) {
  fanMode.set(mode);
  if (inTauri()) {
    const { invoke } = await import("@tauri-apps/api/core");
    await invoke("set_fan_mode", { mode });
  }
}

export const metrics = readable(emptyState(), (set) => {
  const state = emptyState();
  let t = 0;
  let timer = null;
  let unsubFan = fanMode.subscribe((m) => {
    state.fans.mode = m;
  });

  async function tick() {
    t += 1;
    if (inTauri()) {
      // Never simulate inside the app: while the stream warms up (or
      // recovers) keep the last real data and show "connecting"
      // rather than plausible-looking fake numbers.
      try {
        adaptMolePayload(state, await fetchMoleStatus());
        recordHistory(state);
      } catch {
        state.live = false;
      }
    } else {
      simulate(state, t);
      recordHistory(state);
    }
    set({ ...state });
  }

  // Polls never overlap: the next tick is scheduled only after the
  // previous one resolves. A fixed setInterval piled up invokes when
  // collection ran longer than the interval and froze the app.
  let running = false;
  async function loop() {
    await tick();
    if (running) timer = setTimeout(loop, TICK_MS);
  }

  function start() {
    if (running) return;
    running = true;
    loop();
  }

  function stop() {
    running = false;
    if (timer) clearTimeout(timer);
    timer = null;
  }

  // Pause sampling while hidden: zero idle work in the background.
  function onVisibility() {
    document.hidden ? stop() : start();
  }

  document.addEventListener("visibilitychange", onVisibility);
  start();

  return () => {
    stop();
    unsubFan();
    document.removeEventListener("visibilitychange", onVisibility);
  };
});
