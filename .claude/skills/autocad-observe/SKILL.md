---
name: autocad-observe
description: Observe the reference Windows AutoCAD for one behaviour — run an AutoCAD script on the Windows PC through AsenaScale, save screenshots and the command log under plan/autocad/. Use before building or comparing any AsenaCAD command or UI element.
---

# Observe AutoCAD

Reference: Autodesk AutoCAD 2026 (Windows) on the PC named by `$ASENACAD_REF` (its AsenaScale
device name; the local value lives in `plan/STATUS.md`, together with the AutoCAD exe path).
Clean-room rules apply: everything you capture stays in `plan/autocad/` and is never committed.

`asenascale exec|windows|shot` need an AsenaScale release with the PC-to-PC commands
(KaanAlper/AsenaScale#2). If the `asenascale` on `PATH` is older, set `AS=/path/to/newer/asenascale`.

## 1. Write the script locally
`plan/autocad/scripts/<area>-<what>.scr`, one behaviour only, starting with `LOGFILEON` and
ending with `LOGFILEOFF` (see `.claude/rules/parity-method.md`). Example `draw-line-basic.scr`:

```
LOGFILEON
LINE 0,0 100,0 100,50

ZOOM E
LOGFILEOFF
```

## 2. Copy it to Windows

```bash
AS=${AS:-asenascale}
REF=${ASENACAD_REF:?set ASENACAD_REF to the reference PC\'s AsenaScale device name (see plan/STATUS.md)}
NAME=draw-line-basic
"$AS" exec "$REF" -- "New-Item -ItemType Directory -Force C:\\AsenaCAD-obs | Out-Null; Set-Content -Encoding ASCII -Path C:\\AsenaCAD-obs\\$NAME.scr -Value @'
$(cat plan/autocad/scripts/$NAME.scr)
'@"
```

## 3. Run AutoCAD on it
Close any running AutoCAD first so the script starts clean, then start it with the script
(`/b`) and set the log folder:

```bash
ACAD='C:\Program Files\Autodesk\AutoCAD 2026\acad.exe'   # as in plan/STATUS.md
"$AS" exec "$REF" -- "Get-Process acad -ErrorAction SilentlyContinue | Stop-Process -Force; Start-Process -FilePath '$ACAD' -ArgumentList '/nologo','/b','C:\\AsenaCAD-obs\\$NAME.scr'"
```
`Start-Process` through `asenascale exec` opens AutoCAD in the user's desktop session. If a GUI
program doesn't appear, check the session id of the process
(`Get-Process acad | Select-Object Id,SessionId`): it must be the desktop session, not 0.

Wait until the window exists (AutoCAD starts in 20–60 s):

```bash
for i in $(seq 1 30); do "$AS" windows "$REF" | grep -qi 'autocad' && break; sleep 3; done
"$AS" windows "$REF" | grep -i autocad
```

## 4. Capture
Screenshots of AutoCAD are taken **only** with `tools/autocad/shot-acad.sh`: it finds AutoCAD's
window on whichever monitor it is, shoots that monitor, crops to AutoCAD's window (keeping its
floating command line) and deletes the uncropped image. Never call `asenascale shot` directly:
the owner's private windows are on the other monitors.

```bash
tools/autocad/shot-acad.sh plan/autocad/screenshots/$NAME-1.png
```
Read the PNG to confirm it shows the end state (AutoCAD may still be running the script; if so,
wait and shoot again as `-2`, `-3`).

Command log: AutoCAD writes it to the folder in its `LOGFILEPATH` system variable.

```bash
"$AS" exec "$REF" -- "\$p = (Get-ItemProperty 'HKCU:\\Software\\Autodesk\\AutoCAD\\*\\*\\Profiles\\*\\General' -ErrorAction SilentlyContinue | Select-Object -First 1 -Expand LogFilePath); if (-not \$p) { \$p = \"\$env:LOCALAPPDATA\\Autodesk\" }; Get-ChildItem -Recurse -Filter *.log \$p | Sort-Object LastWriteTime | Select-Object -Last 1 | Get-Content" > plan/autocad/logs/$NAME.log
```
If the log is empty, add `LOGFILEPATH` + `C:\AsenaCAD-obs\` as the script's first two lines and
rerun step 3, then read `C:\AsenaCAD-obs\*.log` instead.

## 5. UI structure (optional)
For UI work, dump the UI Automation tree of AutoCAD's main window (control type, name,
AutomationId, class, rectangle; default depth 12, add `-MaxDepth N` after the `}` to change it):

```bash
"$AS" exec "$REF" -- "& { $(cat tools/autocad/uia-dump.ps1) }" > plan/autocad/uia/$NAME.tsv
```

## 6. Write the note
Append to `plan/autocad/notes/<area>.md`: the command, every prompt line in order, option
keywords (with their capitalised abbreviations), defaults shown in `<…>`, what the screenshot
shows. This note — not the screenshot — is what you build from.
