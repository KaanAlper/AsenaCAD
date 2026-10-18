---
name: parity-compare
description: Run the same observation script in AsenaCAD, compare prompts, keywords, defaults, geometry and UI with the AutoCAD observation in plan/autocad/, and update the docs/*-parity.md row. Use after building or changing any command or UI element.
---

# Compare AsenaCAD with AutoCAD

Target: Autodesk AutoCAD 2026 (Windows). Needs an observation from the `autocad-observe` skill:
`plan/autocad/scripts/<name>.scr`, `logs/<name>.log`, `screenshots/<name>-*.png`,
`notes/<area>.md`. Each block runs from the repo root in a fresh shell, sources
`plan/autocad/env.sh` like the observation skill and sets `NAME` itself.

## 1. Run the same script in AsenaCAD
One block: build, start the app on a free port, wait until the control channel answers, then
feed the script **one input at a time** with `cmdline.input`, exactly as AutoCAD reads a script:
every space and every line end is Enter, a blank line is an empty input. Each reply's
`prompt`, `keywords`, `output` (and `error`) is recorded as one JSON line. `LOGFILE*` lines are
for AutoCAD only and are never sent. Then `drawing.inspect`, `ui.screenshot`, and the app is
killed by PID. (Inputs that contain spaces, such as text strings, need a script line the
tokenizer below does not split: extend it when a script needs that.)

```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
cargo build --release -p asenacad || exit 1
PORT=$(python3 -c 'import socket; s=socket.socket(); s.bind(("127.0.0.1",0)); print(s.getsockname()[1])')
target/release/asenacad --control "$PORT" > "plan/autocad/logs/$NAME.asenacad-app.log" 2>&1 &
PID=$!
trap 'kill "$PID" 2>/dev/null' EXIT
python3 - "$PORT" "$NAME" "$PWD" <<'PY'
import json, socket, sys, time
port, name, root = int(sys.argv[1]), sys.argv[2], sys.argv[3]
def call(method, params=None, timeout=60):
    with socket.create_connection(("127.0.0.1", port), timeout=timeout) as s:
        s.sendall((json.dumps({"id": 1, "method": method, "params": params or {}}) + "\n").encode())
        return json.loads(s.makefile().readline())
deadline = time.time() + 120
while True:
    try:
        call("ping", timeout=2); break
    except OSError:
        if time.time() > deadline: sys.exit("AsenaCAD control channel did not answer in 120 s")
        time.sleep(1)
inputs = []
for line in open(f"{root}/plan/autocad/scripts/{name}.scr", encoding="utf-8").read().splitlines():
    if line.strip().upper().startswith("LOGFILE"):
        continue
    inputs += line.split(" ") if line else [""]
with open(f"{root}/plan/autocad/logs/{name}.asenacad.jsonl", "w") as out:
    for text in inputs:
        r = call("cmdline.input", {"text": text}).get("result") or {}
        out.write(json.dumps({"input": text, "prompt": r.get("prompt"), "keywords": r.get("keywords"),
                              "output": r.get("output"), "error": r.get("error")}) + "\n")
with open(f"{root}/plan/autocad/logs/{name}.asenacad-drawing.json", "w") as out:
    json.dump(call("drawing.inspect").get("result"), out, indent=1)
print(call("ui.screenshot", {"path": f"{root}/plan/autocad/screenshots/{name}.asenacad.png"}))
PY
```

## 2. Compare
```bash
. plan/autocad/env.sh; AS=${AS:-asenascale}
NAME=draw-line-basic
python3 - "$NAME" <<'PY'
import json, re, sys
name = sys.argv[1]; base = "plan/autocad"
norm = lambda s: " ".join(s.split())
# AutoCAD: "<prompt> <typed value>" per input; a prompt may wrap onto several lines (ZOOM).
acad, pending = [], ""
for raw in open(f"{base}/logs/{name}.log", encoding="utf-8", errors="replace").read().splitlines()[1:]:
    line = raw.rstrip()
    if not line: continue
    pending = f"{pending} {line}" if pending else line
    if line.endswith(":"):
        acad.append((norm(pending), "")); pending = ""
    elif ": " in line:
        p, v = pending.rsplit(": ", 1); acad.append((norm(p + ":"), v.strip())); pending = ""
acad = [(p, v) for p, v in acad if not v.upper().startswith("LOGFILE")]
# AsenaCAD: the prompt shown before input i is the reply to input i-1, minus its "<CMD> " prefix.
rows = [json.loads(l) for l in open(f"{base}/logs/{name}.asenacad.jsonl")]
shown, ours = "Command:", []
for r in rows:
    ours.append((norm(shown), r["input"], r["keywords"] or [], r.get("error")))
    p = r["prompt"] or "Command:"
    shown = p if p == "Command:" else re.sub(r"^(>>)?\S+ ", "", p)
kw = lambda p: (re.findall(r"\[([^\]]*)\]", p) or [""])[-1].split("/") if "[" in p else []
diffs = 0
for i in range(max(len(acad), len(ours))):
    a = acad[i] if i < len(acad) else ("<missing>", "")
    o = ours[i] if i < len(ours) else ("<missing>", "", [], None)
    if a[0] != o[0] or o[3]:
        diffs += 1
        print(f"#{i} input {a[1]!r}/{o[1]!r}\n  AutoCAD : {a[0]}\n  AsenaCAD: {o[0]}" + (f"\n  error: {o[3]}" if o[3] else ""))
prev_kw = [r["keywords"] or [] for r in rows]
for i, (p, _) in enumerate(acad[1:], 1):
    if i <= len(prev_kw) and kw(p) and kw(p) != prev_kw[i - 1]:
        diffs += 1; print(f"#{i} keywords AutoCAD {kw(p)} vs AsenaCAD {prev_kw[i - 1]}")
hist = {norm(l) for r in rows for l in (r["output"] or [])}
echo = [norm(f"{p} {v}") for p, v in acad]
missing = [e for e in echo if e not in hist]
if missing:
    diffs += 1; print(f"history: {len(missing)} of {len(echo)} AutoCAD history lines are not in AsenaCAD's output, first: {missing[0]!r}")
print(f"{len(acad)} AutoCAD inputs, {len(ours)} AsenaCAD inputs, {diffs} difference(s)")
ents = json.load(open(f"{base}/logs/{name}.asenacad-drawing.json")).get("entities", [])
for e in ents: print("entity", json.dumps({k: v for k, v in e.items() if k not in ("handle",)}))
PY
```
- **Prompts:** each AutoCAD log line is the prompt text before the typed value; the AsenaCAD
  prompt is the `prompt` of the previous reply with its `"<CMD> "` prefix stripped. Whitespace
  (AutoCAD wraps long prompts) is normalised. Every reported line is a wording, keyword or default
  difference.
- **Keywords:** the `[A/B/C]` list in AutoCAD's prompt vs AsenaCAD's `keywords` for that prompt.
- **Geometry:** the printed entities vs what the AutoCAD script drew (its coordinates and the
  note; if the script ends with `LIST`, the AutoCAD log has the entity data). Coordinates must
  match within 1e-9.
- **History:** AutoCAD's command history echoes "prompt value" for every input; compare with the
  `output` arrays in the jsonl.
- **UI:** read `screenshots/<name>-*.png` (AutoCAD) and `screenshots/<name>.asenacad.png`;
  compare position, order, labels and control types of the elements the script touched (table
  in `.claude/rules/parity-method.md`). Ignore icon artwork and colours.

Record every difference in `plan/autocad/notes/<area>.md` in your own words, one bullet each:
area, what AutoCAD does, what AsenaCAD does. Example (draw-line-basic, 2026-10-18):
- *ZOOM prompt:* AutoCAD has a comma after "(nX or nXP)" before "or [All/…]"; AsenaCAD has none.
- *History:* AutoCAD echoes each prompt with the typed value ("Specify first point: 0,0");
  AsenaCAD's history only shows "Command: LINE".

## 3. Record
Update the row in the matching `docs/*-parity.md` (✅ / ◐ with the differences / ✗), its status
line and revision history (`docs/standards/progress-docs.md`); rows measured against AutoCAD 2027
for Mac change only after this Windows comparison. Differences you are not fixing now go into
`docs/gaps.md`. Never put AutoCAD screenshots or log text into these docs — describe the
difference in your own words.
