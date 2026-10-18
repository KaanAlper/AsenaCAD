# AutoCAD observation helpers

Small scripts the `autocad-observe` skill uses to look at the reference Autodesk AutoCAD 2026
(Windows) from the outside, through AsenaScale. They contain no Autodesk material: only our own
shell and PowerShell code that talks to the Windows desktop.

Both need `ASENACAD_REF` (the reference PC's AsenaScale device name) and, optionally, `AS` (an
`asenascale` with the PC-to-PC command `exec`, KaanAlper/AsenaScale#2). Both are set in the local,
gitignored `plan/autocad/env.sh`: `. plan/autocad/env.sh` before calling a helper.

| Helper          | What it does |
|-----------------|--------------|
| `shot-acad.sh plan/autocad/screenshots/NAME.png` | One `asenascale exec` runs PowerShell on the PC that finds AutoCAD's main window, takes its visible frame (DWM extended frame bounds, DPI-aware), copies exactly that rectangle from the screen and prints it as base64 PNG; the script decodes it locally and checks it is a PNG. **No other pixel of the owner's screens ever leaves the PC** and nothing is written to the PC's disk. Fails if AutoCAD is not running or minimized; warns if AutoCAD is not the foreground window. OUT must be under `plan/autocad/`. **The only way to screenshot AutoCAD**: never call `asenascale shot` directly. Read the PNG afterwards; if anything that isn't AutoCAD's is visible (a notification on top), delete it and reshoot. |
| `uia-dump.ps1`  | Runs on the PC (`"$AS" exec "$ASENACAD_REF" -- "& { $(cat tools/autocad/uia-dump.ps1) } -MaxDepth 6"`) and prints the UI Automation tree of AutoCAD's main window: control type, name, AutomationId, class and rectangle per element, one tab-separated line each, indented by depth. It never reads HelpText. Depth 12 (default) takes a minute or more; 4–6 is quick. |

## Where the output goes
Everything these helpers produce is an observation of AutoCAD: screenshots go to
`plan/autocad/screenshots/`, dumps to `plan/autocad/uia/`. `plan/` is gitignored; **never commit
their output** and never paste it into issues, PRs, commit messages or docs. AutoCAD's title bar
and InfoCenter can show the owner's Autodesk ID and file paths: notes never copy them.

## Clean-room
Observe AutoCAD's behaviour, never copy Autodesk's material (icons, artwork, help text, files
from the install folder). Build from your own notes, not from screenshot pixels. Full rules:
[`.claude/rules/clean-room.md`](../../.claude/rules/clean-room.md).
