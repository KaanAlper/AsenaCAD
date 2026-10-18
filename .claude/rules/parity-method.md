# Parity method

Goal: AsenaCAD behaves and looks like **Autodesk AutoCAD 2026 (Windows)**. Every claim of parity
is backed by a side-by-side observation.

## The loop
1. **Observe** (`autocad-observe` skill): write a small AutoCAD script for exactly one behaviour,
   run it on the reference PC, save the screenshot(s) and the command log under `plan/autocad/`.
2. **Build** (`add-command` / `ui-match` skills).
3. **Compare** (`parity-compare` skill or `parity-checker` agent) using the *same* script.
4. **Record** in `docs/*-parity.md`: ✅ matches, ◐ partial (list what differs), ✗ missing.

## What "matches" means
| Area          | Must be equal                                                                                 |
|---------------|-----------------------------------------------------------------------------------------------|
| Command       | Name and aliases; every option keyword and its abbreviation; prompt wording and order; defaults (`<…>`); the resulting geometry (coordinates within 1e-9); undo granularity |
| UI element    | Position and order relative to its neighbours, label text, which control type (button, split button, drop-down, toggle), enabled/disabled state rules, keyboard access |
| Dialog        | Field set and order, defaults, OK/Cancel behaviour, what is remembered next time              |
| Not required  | Pixel-identical rendering, Autodesk's icons/colours/fonts (forbidden anyway, see clean-room)  |

## Scripts
- One behaviour per script, named `plan/autocad/scripts/<area>-<what>.scr`
  (e.g. `modify-trim-fence.scr`). Plain AutoCAD script syntax: one token per line or space,
  a blank line is Enter.
- Every script starts with `LOGFILEON` and ends with `LOGFILEOFF` so the command log is captured.
- Use only synthetic geometry the script creates itself; never open real drawings.

## plan/ layout (local, gitignored)
```
plan/
  STATUS.md                      milestone, next task, reference PC (ASENACAD_REF), AutoCAD path/start method
  autocad/
    scripts/   <area>-<what>.scr
    screenshots/ <area>-<what>-<n>.png   (only via tools/autocad/shot-acad.sh)
    logs/      <area>-<what>.log  (AutoCAD command log)
    notes/     <area>.md          (what was observed: prompts, keywords, defaults)
    uia/       <name>.tsv         (UI Automation dumps from tools/autocad/uia-dump.ps1)
```
