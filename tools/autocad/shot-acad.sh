#!/usr/bin/env bash
# Screenshot of AutoCAD's main window only. The image is cropped ON THE REFERENCE PC: one
# `asenascale exec` runs PowerShell that finds AutoCAD's main window, copies exactly its visible
# frame from the screen into a bitmap and prints it as base64 PNG. No other pixel of the owner's
# screens ever leaves the PC, and nothing is written to the PC's disk.
#
#   tools/autocad/shot-acad.sh plan/autocad/screenshots/NAME-1.png
#
# Env: ASENACAD_REF  the reference PC's AsenaScale device name (required; see plan/autocad/env.sh)
#      AS            the asenascale binary (default: asenascale on PATH; needs `exec`)
# The output is an AutoCAD screenshot: it must be under plan/autocad/ (gitignored, enforced
# below); never commit it. Read the PNG afterwards: if anything that is not AutoCAD's is visible
# (a notification or another window on top), delete it and shoot again.
set -euo pipefail
OUT=${1:?usage: shot-acad.sh plan/autocad/screenshots/NAME.png}
AS=${AS:-asenascale}
DEV=${ASENACAD_REF:?set ASENACAD_REF to the reference PC\'s AsenaScale device name (plan/autocad/env.sh)}

# OUT must resolve to a .png under <repo>/plan/autocad/.
ROOT=$(git -C "$(dirname "$0")" rev-parse --show-toplevel)
ABS=$(realpath -m -- "$OUT")
case "$ABS" in
  "$ROOT"/plan/autocad/*.png) ;;
  *) echo "shot-acad.sh: OUT must be a .png under plan/autocad/ (got $OUT)" >&2; exit 2 ;;
esac
mkdir -p -- "$(dirname -- "$ABS")"

# Prints "ok WxH+X+Y" on the first line, then the PNG as base64. Errors go to stderr, exit 3.
read -r -d '' PS <<'POWERSHELL' || true
$ErrorActionPreference = 'Stop'
function Fail($m) { [Console]::Error.WriteLine("shot-acad: $m"); exit 3 }
Add-Type -AssemblyName System.Drawing
Add-Type @"
using System; using System.Runtime.InteropServices;
public static class AcadShot {
  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int a, out RECT r, int size);
}
"@
[AcadShot]::SetProcessDPIAware() | Out-Null
$p = Get-Process acad -ErrorAction SilentlyContinue | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
if (-not $p) { Fail 'no AutoCAD main window (acad.exe not running in the desktop session?)' }
$h = $p.MainWindowHandle
if ([AcadShot]::IsIconic($h)) { Fail 'AutoCAD is minimized; restore it first' }
if (-not [AcadShot]::IsWindowVisible($h)) { Fail 'AutoCAD main window is not visible' }
$r = New-Object AcadShot+RECT
# DWMWA_EXTENDED_FRAME_BOUNDS = 9: the visible frame, without the invisible resize borders.
if ([AcadShot]::DwmGetWindowAttribute($h, 9, [ref]$r, 16) -ne 0) {
  if (-not [AcadShot]::GetWindowRect($h, [ref]$r)) { Fail 'cannot get the window rectangle' }
}
$w = $r.R - $r.L; $hh = $r.B - $r.T
if ($w -le 0 -or $hh -le 0) { Fail "empty window rectangle ${w}x${hh}" }
$fg = [AcadShot]::GetForegroundWindow(); $fgPid = [uint32]0
[AcadShot]::GetWindowThreadProcessId($fg, [ref]$fgPid) | Out-Null
if ($fgPid -ne $p.Id) { [Console]::Error.WriteLine('shot-acad: warning: AutoCAD is not the foreground window; check the image for other windows on top') }
$bmp = New-Object System.Drawing.Bitmap $w, $hh
$g = [System.Drawing.Graphics]::FromImage($bmp)
try {
  $g.CopyFromScreen($r.L, $r.T, 0, 0, (New-Object System.Drawing.Size $w, $hh))
  $ms = New-Object System.IO.MemoryStream
  $bmp.Save($ms, [System.Drawing.Imaging.ImageFormat]::Png)
  [Console]::Out.WriteLine("ok ${w}x${hh}+$($r.L)+$($r.T)")
  [Console]::Out.Write([Convert]::ToBase64String($ms.ToArray()))
  [Console]::Out.WriteLine()
} finally { $g.Dispose(); $bmp.Dispose() }
POWERSHELL

TMP=$(mktemp)
trap 'rm -f -- "$TMP"' EXIT
if ! "$AS" exec "$DEV" -- "$PS" > "$TMP"; then
  echo "shot-acad.sh: capture on $DEV failed (see the message above)" >&2
  exit 1
fi
HEADER=$(head -n 1 "$TMP" | tr -d '\r')
case "$HEADER" in
  "ok "*) ;;
  *) echo "shot-acad.sh: unexpected reply from $DEV: ${HEADER:0:200}" >&2; exit 1 ;;
esac
tail -n +2 "$TMP" | tr -d '\r\n ' | base64 -d > "$ABS" 2>/dev/null || true
if [ "$(head -c 8 "$ABS" | od -An -tx1 | tr -d ' \n')" != "89504e470d0a1a0a" ]; then
  rm -f -- "$ABS"
  echo "shot-acad.sh: the reply did not decode to a PNG; nothing saved" >&2
  exit 1
fi
echo "$OUT (AutoCAD window ${HEADER#ok })"
