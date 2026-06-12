# Mole GUI

A native-feeling, Liquid Glass desktop front-end for Mole, built with
Tauri 2 + Svelte 5 + Vite.

## Why this stack

- Tauri renders in the macOS system WebView (WKWebView): no bundled
  Chromium, ~5 MB binaries, 30-60 MB idle RAM instead of Electron's
  150-400 MB, and near-instant cold start.
- Svelte 5 compiles components to plain JS with no virtual DOM
  runtime, so the front-end bundle stays tiny and idle CPU stays at
  zero between metric ticks.
- Styling is plain CSS custom properties (`src/styles/tokens.css`),
  no utility framework in the shipping bundle.

## Architecture

- `src/styles/tokens.css` - all design tokens (glass recipe, type
  scale, accent, motion). `base.css` defines the shared `.glass`,
  `.glass-raised`, `.glass-inset` surfaces and the `gentle-pulse`
  animation (auto-disabled under Reduce Motion).
- `src/App.svelte` - shell: floating glass sidebar + content stage,
  Cmd+1-5 module switching.
- `src/lib/metrics.js` - 1 Hz metrics store with 60-sample ring
  buffers per metric. Inside Tauri it adapts `mo status --json`;
  in a plain browser it falls back to a simulator with the same
  shape. Sampling pauses while the window is hidden.
- `src/modules/` - Status (fully functional), Clean, Software,
  Optimize, Analyze shells.
- `src-tauri/` - thin Rust layer: `mole_status` shells out to the
  existing `mo` binary; destructive logic intentionally stays in the
  audited shell/Go core. macOS vibrancy is applied natively via
  `window-vibrancy` so the glass panels sample the real desktop.

## Behavioral guardrails encoded in the UI

- Conservative defaults: only "Safe"-tier cache items pre-selected.
- Double-check confirmation: destructive actions always show the
  exact file list and byte counts in an `alertdialog` first; cache
  purges are labeled permanent, uninstalls route to Trash.
- Path refusal: protected paths (`/System`, `/Library/Apple`, ...)
  render as blocked rows and refuse selection.
- 100% local: there is no telemetry, analytics, or cloud-sync UI.
- Accessibility: semantic landmarks, ARIA radiogroup for fan modes,
  keyboard navigation throughout, visible focus rings, and
  `prefers-reduced-motion` support.

## Develop

```bash
cd gui
npm install
npm run dev          # browser preview with simulated metrics
npm run tauri:dev    # full app (requires Rust + macOS)
npm run tauri:build  # release .app / .dmg
```

Icons are not checked in; generate them once with
`npm run tauri icon path/to/mole-icon.png` before a release build.
