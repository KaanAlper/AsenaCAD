---
name: autocad-observe
description: Observe the reference Windows AutoCAD for one behaviour — run an AutoCAD script on the Windows PC through AsenaScale, save screenshots and the command log under plan/autocad/. Use before building or comparing any AsenaCAD command or UI element.
---

# Observe AutoCAD

Reference: Autodesk AutoCAD 2026 (Windows) on the PC named by `ASENACAD_REF`. Clean-room rules
apply: everything you capture stays in `plan/autocad/` and is never committed. AutoCAD's title
bar and InfoCenter may show the owner's Autodesk ID or file paths: notes never copy them.

Every block below runs from the repo root in a **fresh shell**, so each one starts by sourcing
`plan/autocad/env.sh` (local, gitignored: `ASENACAD_REF`, `ACAD`, optionally `AS`; fields in
AGENTS.md "Start every session here") and sets `NAME` itself. `asenascale exec` needs an
AsenaScale release with the PC-to-PC commands (KaanAlper/AsenaScale#2); set `AS` in `env.sh` if
the `asenascale` on `PATH` is older. `exec` runs Windows PowerShell 5.1 in the owner's desktop
session; never sign in to AutoCAD and never change its settings persistently (no `LOGFILEPATH`,
no options dialogs).

## 1. Write the script locally
`plan/autocad/scripts/<area>-<what>.scr`, one behaviour only, starting with `LOGFILEON` and
ending with `LOGFILEOFF` (see `.claude/rules/parity-method.md`). A blank line is Enter; keep a
newline at the end. Example `draw-line-basic.scr`:

```
LOGFILEON
LINE 0,0 100,0 100,50

ZOOM E
LOGFILEOFF
```

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
mkdir -p plan/autocad/{scripts,screenshots,logs,notes,uia}
```

## 2. Copy it to Windows
The file is sent as base64 with CRLF line ends, so trailing blank lines survive exactly.

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
B64=$(sed 's/\r$//; s/$/\r/' "plan/autocad/scripts/$NAME.scr" | base64 -w0)
"$AS" exec "$ASENACAD_REF" -- "New-Item -ItemType Directory -Force C:\\AsenaCAD-obs | Out-Null; [IO.File]::WriteAllBytes('C:\\AsenaCAD-obs\\$NAME.scr', [Convert]::FromBase64String('$B64')); 'bytes on PC: ' + (Get-Item 'C:\\AsenaCAD-obs\\$NAME.scr').Length"
echo "bytes local: $(sed 's/\r$//; s/$/\r/' "plan/autocad/scripts/$NAME.scr" | wc -c)"
```
The two byte counts must be equal.

## 3. Close AutoCAD
The script must start in a fresh AutoCAD. Force-closing is allowed **only** when `plan/STATUS.md`
has the exact line `Reference PC: disposable`. Otherwise AutoCAD is closed gracefully; if it is
still running after 20 s (usually its unsaved-changes dialog), the block fails: stop and ask the
owner, never click the dialog away yourself.

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
FORCE=0; grep -qx 'Reference PC: disposable' plan/STATUS.md && FORCE=1
"$AS" exec "$ASENACAD_REF" -- "\$p = Get-Process acad -ErrorAction SilentlyContinue
if (-not \$p) { 'AutoCAD not running'; exit 0 }
if ($FORCE) { \$p | Stop-Process -Force; Start-Sleep 3; 'AutoCAD force-closed'; exit 0 }
\$p | ForEach-Object { [void]\$_.CloseMainWindow() }
for (\$i = 0; \$i -lt 20; \$i++) { Start-Sleep 1; if (-not (Get-Process acad -ErrorAction SilentlyContinue)) { 'AutoCAD closed'; exit 0 } }
[Console]::Error.WriteLine('AutoCAD is still running after 20 s (unsaved-changes dialog?): ask the owner'); exit 4"
```

## 4. Start AutoCAD on the script and wait for it
`Start-Process` through `exec` opens AutoCAD in the owner's desktop session (`/b` runs the
script after start-up). The block waits up to 120 s for AutoCAD's main window, then lists
AutoCAD's own visible top-level windows (never the owner's other windows).

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
"$AS" exec "$ASENACAD_REF" -- "Start-Process -FilePath '$ACAD' -ArgumentList '/nologo','/b','C:\\AsenaCAD-obs\\$NAME.scr'
for (\$i = 0; \$i -lt 60; \$i++) { \$p = Get-Process acad -ErrorAction SilentlyContinue | Where-Object { \$_.MainWindowHandle -ne 0 } | Select-Object -First 1; if (\$p) { break }; Start-Sleep 2 }
if (-not \$p) { [Console]::Error.WriteLine('AutoCAD has no main window after 120 s'); exit 5 }
'AutoCAD up: session ' + \$p.SessionId
Add-Type @'
using System; using System.Text; using System.Collections.Generic; using System.Runtime.InteropServices;
public static class AcadWins {
  delegate bool Cb(IntPtr h, IntPtr l);
  [DllImport(\"user32.dll\")] static extern bool EnumWindows(Cb cb, IntPtr l);
  [DllImport(\"user32.dll\")] static extern bool IsWindowVisible(IntPtr h);
  [DllImport(\"user32.dll\")] static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport(\"user32.dll\", CharSet = CharSet.Unicode)] static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  public static List<string> Of(uint pid) {
    var r = new List<string>();
    EnumWindows((h, l) => { uint p; GetWindowThreadProcessId(h, out p); if (p == pid && IsWindowVisible(h)) { var s = new StringBuilder(256); GetWindowText(h, s, 256); r.Add(s.ToString()); } return true; }, IntPtr.Zero);
    return r;
  }
}
'@
Start-Sleep 5
'AutoCAD windows:'; [AcadWins]::Of([uint32]\$p.Id) | Where-Object { \$_ } | ForEach-Object { '  ' + \$_ }"
```
If the session is 0 instead of the desktop session, the GUI is not visible: stop and tell the
owner. Expect the main window ("Autodesk AutoCAD 2026 - [Drawing1.dwg]") and possibly the
floating command line. A force-close can bring up **Drawing Recovery** (a recovery palette or
dialog) at the next start: if any other AutoCAD window is listed, take a screenshot (step 6),
look at it, and if a dialog blocks the script, close AutoCAD again (step 3) and restart; tell the
owner if it keeps coming back.

## 5. Command log
With `LOGFILEPATH` unset (as on the reference PC), `LOGFILEON` writes to
`%LOCALAPPDATA%\Autodesk\AutoCAD 2026\R25.1\enu\<Drawing>_<n><hash>.log` (e.g.
`Drawing1_1_xxxxxxxx.log`). The block takes the newest `Drawing*.log` in **that folder only**
written after AutoCAD started, and waits (up to 120 s) until it contains `LOGFILEOFF`, i.e. the
script has finished. Never set `LOGFILEPATH` (it is a persistent profile setting); for another
AutoCAD release adjust the `R25.1\enu` folder.

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
"$AS" exec "$ASENACAD_REF" -- "\$dir = Join-Path \$env:LOCALAPPDATA 'Autodesk\\AutoCAD 2026\\R25.1\\enu'
\$p = Get-Process acad -ErrorAction SilentlyContinue | Where-Object { \$_.MainWindowHandle -ne 0 } | Select-Object -First 1
if (-not \$p) { [Console]::Error.WriteLine('AutoCAD is not running'); exit 5 }
for (\$i = 0; \$i -lt 60; \$i++) {
  \$log = Get-ChildItem -Path \$dir -Filter 'Drawing*.log' -File -ErrorAction SilentlyContinue | Where-Object { \$_.LastWriteTime -gt \$p.StartTime } | Sort-Object LastWriteTime | Select-Object -Last 1
  if (\$log -and (Select-String -Path \$log.FullName -Pattern 'LOGFILEOFF' -SimpleMatch -Quiet)) { Get-Content -Path \$log.FullName; exit 0 }
  Start-Sleep 2 }
[Console]::Error.WriteLine('no finished command log after 120 s (script still running, or no LOGFILEON/LOGFILEOFF?)'); exit 6" | tr -d '\r' > "plan/autocad/logs/$NAME.log"
cat "plan/autocad/logs/$NAME.log"
```

## 6. Screenshot
Screenshots of AutoCAD are taken **only** with `tools/autocad/shot-acad.sh`: it crops to
AutoCAD's main window on the PC, so no other pixel of the owner's screens leaves it. Never call
`asenascale shot` yourself.

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
AS=$AS tools/autocad/shot-acad.sh "plan/autocad/screenshots/$NAME-1.png"
```
Read the PNG. If anything that isn't AutoCAD's is visible (a notification or another window on
top), delete it and reshoot. If AutoCAD hasn't reached the end state yet, shoot again as `-2`,
`-3`.

## 7. UI structure (optional)
For UI work, dump the UI Automation tree of AutoCAD's main window (control type, name,
AutomationId, class, rectangle). The default depth is 12, which walks the whole ribbon and takes
a minute or more; add `-MaxDepth N` after the `}` for a quicker, shallower dump.

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
"$AS" exec "$ASENACAD_REF" -- "& { $(cat tools/autocad/uia-dump.ps1) } -MaxDepth 6" | tr -d '\r' > "plan/autocad/uia/$NAME.tsv"
wc -l "plan/autocad/uia/$NAME.tsv"
```

## 8. Write the note
Append to `plan/autocad/notes/<area>.md`: the command, every prompt line in order, option
keywords (with their capitalised abbreviations), defaults shown in `<…>`, what the screenshot
shows. This note — not the screenshot — is what you build from. Never copy the owner's Autodesk
ID, user name or file paths into it.
