#!/usr/bin/env bash
# Copies the AMECAH icons and logos over the RustDesk ones before building.
# It runs on the build tree (CI or local) and is not committed applied, so
# upstream merges do not conflict on binary files.
set -euo pipefail

root="$(cd "$(dirname "$0")/.." && pwd)"
b="$root/amecah/branding"

cp "$b/icon.ico" "$root/res/icon.ico"
cp "$b/icon.ico" "$root/res/tray-icon.ico"
cp "$b/icon.png" "$root/res/icon.png"
cp "$b/icon.ico" "$root/flutter/windows/runner/resources/app_icon.ico"

# The Flutter UI shows assets/logo.png (and logo_dark.png in dark mode) when present,
# and prefers assets/icon.png over assets/icon.svg.
cp "$b/icon.png" "$root/flutter/assets/icon.png"
cp "$b/logo.png" "$root/flutter/assets/logo.png"
cp "$b/logo_dark.png" "$root/flutter/assets/logo_dark.png"

echo "AMECAH branding applied"
