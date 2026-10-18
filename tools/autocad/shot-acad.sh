#!/usr/bin/env bash
# Screenshot of AutoCAD only, wherever it is: finds AutoCAD's main window rectangle and the
# monitor it is on, shoots that monitor, crops to AutoCAD's rectangle (which keeps AutoCAD's own
# floating command line, unlike a window shot) and deletes the uncropped image at once, so
# nothing else on the owner's screens is kept.
#
#   tools/autocad/shot-acad.sh plan/autocad/screenshots/NAME-1.png
#
# Env: ASENACAD_REF  the reference PC's AsenaScale device name (required; see plan/STATUS.md)
#      AS            the asenascale binary (default: asenascale on PATH; needs exec/windows/shot)
# Needs ImageMagick (`magick`) locally. The output is an AutoCAD screenshot: keep it under
# plan/autocad/ (gitignored), never commit it.
set -euo pipefail
OUT=${1:?usage: shot-acad.sh OUT.png}
AS=${AS:-asenascale}
DEV=${ASENACAD_REF:?set ASENACAD_REF to the reference PC\'s AsenaScale device name}

# "monitorDeviceName x y w h": AutoCAD's rect relative to the monitor that holds its centre.
read -r MON X Y W H < <("$AS" exec "$DEV" -- 'Add-Type -AssemblyName System.Windows.Forms
Add-Type @"
using System; using System.Runtime.InteropServices;
public class AcadRect { [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L,T,R,B; }
[DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r); }
"@
$p = Get-Process acad -ErrorAction Stop | Where-Object { $_.MainWindowHandle -ne 0 } | Select-Object -First 1
$r = New-Object AcadRect+RECT; [AcadRect]::GetWindowRect($p.MainWindowHandle, [ref]$r) | Out-Null
$cx = [int](($r.L + $r.R) / 2); $cy = [int](($r.T + $r.B) / 2)
$screens = [System.Windows.Forms.Screen]::AllScreens
for ($i = 0; $i -lt $screens.Length; $i++) {
  $b = $screens[$i].Bounds
  if ($cx -ge $b.X -and $cx -lt $b.X + $b.Width -and $cy -ge $b.Y -and $cy -lt $b.Y + $b.Height) {
    $x = [Math]::Max($r.L, $b.X) - $b.X; $y = [Math]::Max($r.T, $b.Y) - $b.Y
    $w = [Math]::Min($r.R, $b.X + $b.Width) - $b.X - $x; $h = [Math]::Min($r.B, $b.Y + $b.Height) - $b.Y - $y
    "$($screens[$i].DeviceName) $x $y $w $h"; break } }' | tr -d '\r') || true
[ -n "${H:-}" ] || { echo "AutoCAD's main window not found on $DEV (is acad running in the desktop session?)" >&2; exit 1; }

# Map the Windows display (\\.\DISPLAYn) to the xcap output index by resolution in `windows`.
SCREEN_W_H=$("$AS" exec "$DEV" -- "Add-Type -AssemblyName System.Windows.Forms; \$s=[System.Windows.Forms.Screen]::AllScreens | Where-Object { \$_.DeviceName -eq '$MON' }; \"\$(\$s.Bounds.Width)x\$(\$s.Bounds.Height)\"" | tr -d '\r')
IDX=$("$AS" windows "$DEV" | awk -F'\t' -v wh="$SCREEN_W_H" '$1=="output" && index($3, wh) {print $2; exit}')
[ -n "$IDX" ] || { echo "can't find the xcap output for $MON ($SCREEN_W_H)" >&2; exit 1; }

RAW=$(mktemp --suffix=.png)
trap 'rm -f "$RAW"' EXIT
"$AS" shot "$DEV" output "$IDX" -o "$RAW" >/dev/null
magick "$RAW" -crop "${W}x${H}+${X}+${Y}" +repage "$OUT"
echo "$OUT (AutoCAD on $MON = output $IDX, ${W}x${H}+${X}+${Y})"
