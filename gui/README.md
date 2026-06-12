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
- `src-tauri/` - thin Rust layer. The Go status collector
  (`cmd/status`) is compiled by `scripts/build-sidecar.sh` and
  bundled inside the app (Contents/MacOS/mole-status), so the DMG is
  self-contained: Status shows real metrics with no separate CLI
  install. A system-wide `mo` is used as fallback. Destructive logic
  intentionally stays in the audited shell/Go core. macOS vibrancy is
  applied natively via `window-vibrancy` so the glass panels sample
  the real desktop.

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
npm run tauri:dev    # full app (requires Rust + Go + macOS)
npm run tauri:build  # release .app / .dmg
```

`tauri:dev` and `tauri:build` first run `npm run sidecar`, which
compiles `cmd/status` into `src-tauri/binaries/` for bundling, so Go
must be installed alongside Rust.

## Build the .dmg installer

DMG creation needs macOS (Tauri's bundler uses `hdiutil`). Two paths:

- CI: run the "GUI DMG" workflow from the Actions tab (or
  `gh workflow run gui-dmg.yml`). It builds an Apple Silicon
  `Mole_x.y.z_aarch64.dmg` with a `SHA256SUMS` file and uploads both
  as the `mole-gui-dmg` artifact. Pushing a `GUI-V*` tag triggers
  the same build. To also cover Intel Macs, switch the build step to
  `--target universal-apple-darwin` (roughly doubles compile time
  and binary size; zero runtime cost on either architecture).
- Locally on a Mac: `cd gui && npm ci && npm run tauri build`.
  The image lands in `src-tauri/target/release/bundle/dmg/`.

The bundle is ad-hoc signed (`signingIdentity: "-"`) but not
notarized, so a browser-downloaded copy is quarantined and Gatekeeper
blocks the first launch ("damaged" or "Apple could not verify").
Right-click → Open does not bypass this on recent macOS. Either:

```bash
xattr -cr /Applications/Mole.app
```

or attempt the launch once, then approve it under System Settings →
Privacy & Security → "Open Anyway". Proper signing requires a
Developer ID certificate wired into the workflow via Tauri's
`APPLE_CERTIFICATE` / notarization secrets.

App icons live in `src-tauri/icons/` (generated from `app-icon.png`
with `npm run tauri icon app-icon.png`).
