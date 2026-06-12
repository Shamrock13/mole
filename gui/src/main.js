import { mount } from "svelte";
import App from "./App.svelte";
import TrayPanel from "./TrayPanel.svelte";

// Both Tauri windows load the same bundle; the window label picks the
// root component. `#tray` mimics the popover in plain `vite dev`.
function windowLabel() {
  const internals = typeof window !== "undefined" ? window.__TAURI_INTERNALS__ : null;
  const label = internals?.metadata?.currentWindow?.label;
  if (label) return label;
  return location.hash === "#tray" ? "tray" : "main";
}

const app = mount(windowLabel() === "tray" ? TrayPanel : App, {
  target: document.getElementById("app"),
});

export default app;
