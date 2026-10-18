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
  a blank line is Enter (trailing blank lines count; the skill keeps them).
- Every script starts with `LOGFILEON` and ends with `LOGFILEOFF` so AutoCAD writes the command
  log. These two lines are for AutoCAD only: `parity-compare` never sends `LOGFILE*` lines to
  AsenaCAD.
- Use only synthetic geometry the script creates itself; never open real drawings.

## plan/ layout (local, gitignored)
```
plan/
  STATUS.md        Milestone:, Next task:, Reference PC: (note; `disposable` allows force-closing
                   AutoCAD), AutoCAD exe:, Upstream reviewed:  (see AGENTS.md "Start every session here")
  autocad/
    env.sh         export ASENACAD_REF=<device>  ACAD='<acad.exe path on the PC>'  [AS=<asenascale>]
                   sourced by every skill block: `. plan/autocad/env.sh; AS=${AS:-asenascale}`
    scripts/       <name>.scr                    (<name> = <area>-<what>)
    screenshots/   <name>-<n>.png                AutoCAD, only via tools/autocad/shot-acad.sh
                   <name>.asenacad.png           AsenaCAD (parity-compare)
    logs/          <name>.log                    AutoCAD command log
                   <name>.asenacad.jsonl         AsenaCAD: one reply per script line (parity-compare)
                   <name>.asenacad-drawing.json  AsenaCAD drawing.inspect after the script
                   <name>.asenacad-app.log       AsenaCAD's own stdout/stderr during the run
    notes/         <area>.md                     what was observed: prompts, keywords, defaults, diffs
    uia/           <name>.tsv                    UI Automation dumps (tools/autocad/uia-dump.ps1)
```
