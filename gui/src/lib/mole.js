// Bridge to the Mole CLI through the Rust shell. Tasks are an
// allowlist on the Rust side; this module adds event plumbing and a
// small console-state factory shared by Clean and Optimize.

import { writable } from "svelte/store";

export function inTauri() {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

async function api() {
  const { invoke } = await import("@tauri-apps/api/core");
  return invoke;
}

/// Resolved path of the system-wide `mo`, or null when not installed
/// (or when running in a plain browser).
export async function moleCliPath() {
  if (!inTauri()) return null;
  const invoke = await api();
  return invoke("mole_cli_path");
}

export async function listInstalledApps() {
  const invoke = await api();
  const raw = await invoke("list_installed_apps");
  return JSON.parse(raw);
}

const MAX_LINES = 800;

// One console: lines + running flag + start/cancel. Each module
// creates its own; the Rust side enforces one task at a time.
export function createTaskRunner() {
  const lines = writable([]);
  const running = writable(false);
  let active = null;
  let unlisteners = [];

  async function ensureListeners() {
    if (unlisteners.length) return;
    const { listen } = await import("@tauri-apps/api/event");
    unlisteners = [
      await listen("mole-task-output", (event) => {
        if (event.payload.task !== active) return;
        lines.update((all) => {
          const next = all.length >= MAX_LINES ? all.slice(-MAX_LINES + 1) : all.slice();
          next.push(event.payload.line);
          return next;
        });
      }),
      await listen("mole-task-done", (event) => {
        if (event.payload.task !== active) return;
        running.set(false);
        const code = event.payload.code;
        lines.update((all) => [
          ...all,
          code === 0 ? "✓ Done" : `✕ Exited with code ${code ?? "unknown"}`,
        ]);
      }),
    ];
  }

  async function start(task) {
    const invoke = await api();
    await ensureListeners();
    active = task;
    lines.set([]);
    running.set(true);
    try {
      await invoke("run_mole_task", { task });
    } catch (error) {
      running.set(false);
      lines.set([String(error)]);
    }
  }

  async function cancel() {
    const invoke = await api();
    await invoke("cancel_mole_task");
    running.set(false);
    lines.update((all) => [...all, "✕ Stopped"]);
  }

  function destroy() {
    for (const unlisten of unlisteners) unlisten();
    unlisteners = [];
  }

  return { lines, running, start, cancel, destroy };
}
