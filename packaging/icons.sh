#!/usr/bin/env bash
# Regenerate every derived app icon from assets/app-icon/asenacad.svg.
# Needs resvg, ImageMagick (magick) and python3 with Pillow; runs on Linux or macOS.
# The outputs are committed, so builds and packaging never need these tools.
#
#   packaging/icons.sh
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIR="$ROOT/assets/app-icon"
SVG="$DIR/asenacad.svg"
ID="io.github.kaanalper.asenacad"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
for t in resvg magick python3; do command -v "$t" >/dev/null || { echo "error: $t not found" >&2; exit 1; }; done

png() { resvg -w "$1" -h "$1" "$SVG" "$2"; }

png 1024 "$DIR/asenacad-1024.png"
png 256 "$DIR/asenacad-256.png"
png 64 "$DIR/asenacad-64.png"
for s in 16 24 32 48 64 128 256 512; do
  mkdir -p "$DIR/hicolor/${s}x${s}/apps"
  png "$s" "$DIR/hicolor/${s}x${s}/apps/$ID.png"
done

# macOS grid: the artwork at 80 % on a transparent 1024 canvas.
png 824 "$TMP/inner.png"
magick "$TMP/inner.png" -background none -gravity center -extent 1024x1024 "$TMP/macos-1024.png"
magick "$TMP/macos-1024.png" -resize 512x512 "$DIR/asenacad-macos-512.png"

ICO_PNGS=()
for s in 16 20 24 32 40 48 64 128 256; do
  png "$s" "$TMP/ico-$s.png"
  ICO_PNGS+=("$TMP/ico-$s.png")
done
(cd "$ROOT" && cargo run -q -p xtask -- ico "$DIR/asenacad.ico" "${ICO_PNGS[@]}")

python3 - "$TMP/macos-1024.png" "$DIR/asenacad.icns" <<'PY'
import sys
from PIL import Image
Image.open(sys.argv[1]).save(sys.argv[2], format="ICNS")
PY
echo "icons written to $DIR"
