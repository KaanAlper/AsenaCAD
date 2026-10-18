# AutoCAD observation helpers

Small scripts the `autocad-observe` skill uses to look at the reference Autodesk AutoCAD 2026
(Windows) from the outside, through AsenaScale. They contain no Autodesk material: only our own
shell and PowerShell code that talks to the Windows desktop.

Both need `ASENACAD_REF` (the reference PC's AsenaScale device name; the local value is in
`plan/STATUS.md`) and an `asenascale` with the PC-to-PC commands `exec`/`windows`/`shot`
(KaanAlper/AsenaScale#2). Set `AS=/path/to/asenascale` if the one on `PATH` is older.

| Helper          | What it does |
|-----------------|--------------|
| `shot-acad.sh OUT.png` | Finds AutoCAD's main window and the monitor it is on, shoots that monitor, crops to AutoCAD's window (keeping its floating command line) and deletes the uncropped image at once. **The only way to screenshot AutoCAD**: the owner moves AutoCAD between monitors and has private windows on the others, so never call `asenascale shot` directly. Needs ImageMagick (`magick`). |
| `uia-dump.ps1`  | Runs on the PC (`asenascale exec "$ASENACAD_REF" -- "& { $(cat tools/autocad/uia-dump.ps1) } -MaxDepth 12"`) and prints the UI Automation tree of AutoCAD's main window: control type, name, AutomationId, class and rectangle per element, indented by depth. It never reads HelpText. |

## Where the output goes
Everything these helpers produce is an observation of AutoCAD: screenshots go to
`plan/autocad/screenshots/`, dumps to `plan/autocad/uia/`. `plan/` is gitignored; **never commit
their output** and never paste it into issues, PRs, commit messages or docs.

## Clean-room
Observe AutoCAD's behaviour, never copy Autodesk's material (icons, artwork, help text, files
from the install folder). Build from your own notes, not from screenshot pixels. Full rules:
[`.claude/rules/clean-room.md`](../../.claude/rules/clean-room.md).
