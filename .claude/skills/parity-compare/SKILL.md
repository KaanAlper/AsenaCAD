---
name: parity-compare
description: Run the same observation script in AsenaCAD, compare prompts, keywords, defaults, geometry and UI with the AutoCAD observation in plan/autocad/, and update the docs/*-parity.md row. Use after building or changing any command or UI element.
---

# Compare AsenaCAD with AutoCAD

Target: Autodesk AutoCAD 2026 (Windows). Needs an observation from the `autocad-observe` skill:
`plan/autocad/scripts/<name>.scr`, `logs/<name>.log`, `screenshots/<name>-*.png`,
`notes/<area>.md`.

## 1. Run the same script in AsenaCAD
```bash
PORT=47811
cargo run --release -p asenacad -- --control $PORT &
sleep 15
ctl() { python3 -c 'import socket,sys; s=socket.create_connection(("127.0.0.1",int(sys.argv[1]))); s.sendall((sys.argv[2]+"\n").encode()); print(s.makefile().readline().strip())' $PORT "$1"; }
grep -v -E '^(LOGFILEON|LOGFILEOFF|LOGFILEPATH)' plan/autocad/scripts/<name>.scr | while IFS= read -r line; do
  ctl "$(python3 -c 'import json,sys; print(json.dumps({"id":1,"method":"cmdline.input","params":{"text":sys.argv[1]}}))' "$line")"
done > plan/autocad/logs/<name>.asenacad.jsonl
ctl '{"id":2,"method":"ui.screenshot","params":{"path":"'"$PWD"'/plan/autocad/screenshots/<name>-asenacad.png"}}'
ctl '{"id":3,"method":"drawing.inspect","params":{}}' > plan/autocad/logs/<name>.asenacad-drawing.json
kill %1
```
Each `cmdline.input` reply has an `output` array: the command-line lines AsenaCAD printed.

## 2. Compare
- **Prompts:** AutoCAD's log lines vs. the concatenated `output` arrays, in order. Report every
  line that differs (wording, keyword list, default).
- **Geometry:** entities in `<name>.asenacad-drawing.json` vs. what the AutoCAD script drew
  (from the script's coordinates and the note). Coordinates must match within 1e-9.
- **UI:** read both screenshots; compare position, order, labels and control types of the
  elements the script touched (see the table in `.claude/rules/parity-method.md`). Ignore icon
  artwork and colours.

## 3. Record
Update the row in the matching `docs/*-parity.md` (✅ / ◐ with the differences / ✗), its status
line and revision history (`docs/standards/progress-docs.md`). Differences you are not fixing now
go into `docs/gaps.md`. Never put AutoCAD screenshots or log text into these docs — describe the
difference in your own words.
