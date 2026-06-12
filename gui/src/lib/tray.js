// Menu bar (tray) configuration. Persisted in localStorage on the
// front-end; the Rust side holds only the live state, so the main
// window pushes the stored config on startup and whenever Settings
// changes it.

const STORAGE_KEY = "mole.menubar";

export const DEFAULT_TRAY_CONFIG = { enabled: true, display: "icon" };

export function loadTrayConfig() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY);
    if (!raw) return { ...DEFAULT_TRAY_CONFIG };
    const parsed = JSON.parse(raw);
    return {
      enabled: typeof parsed.enabled === "boolean" ? parsed.enabled : true,
      display: parsed.display === "metrics" ? "metrics" : "icon",
    };
  } catch {
    return { ...DEFAULT_TRAY_CONFIG };
  }
}

export function saveTrayConfig(config) {
  try {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(config));
  } catch {
    // Private mode or full storage: the live state still applies.
  }
}

export async function applyTrayConfig(config) {
  if (typeof window === "undefined" || !("__TAURI_INTERNALS__" in window)) return;
  const { invoke } = await import("@tauri-apps/api/core");
  await invoke("set_tray_config", {
    enabled: config.enabled,
    display: config.display,
  });
}
