#!/usr/bin/env bash
# Builds the mole-status sidecar that gets bundled inside the GUI app.
# Tauri's externalBin expects the file name to end with the host target
# triple; the bundler strips it and ships Contents/MacOS/mole-status.
set -euo pipefail

triple="$(rustc -vV | awk '/^host:/ {print $2}')"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
out="$repo_root/gui/src-tauri/binaries/mole-status-$triple"

mkdir -p "${out%/*}"
echo "Building mole-status sidecar for $triple"
(cd "$repo_root" && CGO_ENABLED=0 go build -ldflags="-s -w" -o "$out" ./cmd/status)
