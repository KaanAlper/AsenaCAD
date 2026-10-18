# AsenaCAD — instructions for agents

AsenaCAD is an open-source, clean-room, pure-Rust CAD application whose **interface, commands and
workflow match Autodesk AutoCAD 2026 (Windows) one to one**. Matching AutoCAD is the goal; we do
not add our own twists to its UI or commands. It runs on Linux, Windows, macOS, FreeBSD and the web
(WASM). It is a fork of [CADCraft](https://github.com/storytold/cadcraft) (see `NOTICE`).

## Start every session here
1. Read `plan/STATUS.md` (milestone, next task, reference PC, AutoCAD path). `plan/` is local and
   gitignored; if it is missing, create it from the template in `.claude/rules/parity-method.md`.
2. Milestones and goals: `docs/specs/2026-10-18-asenacad-design.md`. Work list: `docs/gaps.md`.
3. Library decisions and licence limits: `docs/architecture.md`, `NOTICE`.

## Never crash
Outranks feature work. Standard: [`docs/standards/never-crash.md`](docs/standards/never-crash.md).
No panics in non-test code, no `unsafe`, errors through `Result`, input is hostile, every crash fix
lands with a regression test.

## Clean-room (the most serious rule)
AutoCAD runs on the reference Windows PC and may only be **observed** from the outside: run it,
use it with synthetic drawings, take screenshots and command-line logs. Details:
[`.claude/rules/clean-room.md`](.claude/rules/clean-room.md).
- **Never** read, disassemble or copy anything inside the AutoCAD installation (names and listings
  only). Never copy Autodesk icons, fonts (SHX), hatch patterns (`.pat`), linetypes (`.lin`),
  templates, CUI/PGP files, artwork or help text.
- **Never commit** AutoCAD screenshots, logs or files AutoCAD produced. They live only under
  `plan/autocad/`. This repo is public.
- File formats come from public specifications (Autodesk's DXF Reference, the ODA's public DWG
  specification) and our own synthetic tests. Never copy or link GPL/LGPL/AGPL CAD code
  (LibreDWG, LibreCAD, QCAD, FreeCAD, SolveSpace, OpenCASCADE); don't read their sources.
- Every bundled asset is original or permissively licensed and has a row in `ATTRIBUTION.md`
  (`cargo xtask assets` enforces it). Icons are drawn in code (`crates/ui-egui/src/icons.rs`),
  hatch patterns and linetypes are our own definitions (`crates/doc/src/library.rs`), the
  drafting font is our own (`crates/fonts/src/stroke.rs`).
- Fonts live in [`storytold/craft-fonts`](https://github.com/storytold/craft-fonts), never in this
  repo: an optional build input (`CRAFT_FONTS_DIR`, embedded by `crates/fonts/build.rs`), never a
  `Cargo.toml` dependency. Code and tests must work without it.
- Shared test corpora live in separate repos; never commit large binary fixtures here.

## Architecture rules
- **Everything is a command.** User-visible behaviour is a `CommandSpec` in
  `crates/engine/src/cmd/*` (id = AutoCAD's command name in lower case, label, menu path,
  shortcut, aliases, params doc, `enabled`, JSON `run`, optional interactive prompt machine) plus
  tests. UI-only commands live in `crates/ui-egui/src/menus.rs` (`UI_COMMANDS`). Command line,
  ribbon, menus, scripts, CLI, control channel and MCP all reach the same commands.
- **Programmatic calls never open dialogs.** `engine.execute` runs the JSON form with defaults.
- **Layering** (`cargo xtask layers`): L0 `geom`, `dxf` → L1 `color`, `doc` → L2 `fonts`,
  `render`, `constraints` → L3 `io` → L4 `engine` → L5 `ui-egui`, `mcp` → apps. Nothing below L5
  depends on egui/eframe/winit/wgpu/rfd.
- **The UI is thin**: panels read engine state and act through `app.run(id, params)` /
  `app.start(id)` / `app.cmdline(text)`. Colours come from `theme::Tokens`.
- **Rust only**, never break wasm (`cargo xtask wasm`).
- Library crates keep their upstream names (`cadcraft-geom` … `cadcraft-mcp`) and upper-case
  `CADCRAFT*` identifiers (env vars, DXF xdata names) stay as they are: this keeps upstream
  cherry-picks clean and drawings compatible. Product, apps and app id are AsenaCAD
  (`asenacad`, `asenacad-cli`, `asenacad-web`, `io.github.kaanalper.asenacad`).

## The parity loop
Every feature is done the same way (details in
[`.claude/rules/parity-method.md`](.claude/rules/parity-method.md)):
1. **Observe** AutoCAD with the `autocad-observe` skill (script → screenshot + command log).
2. **Build** it in AsenaCAD (`add-command` skill for commands, `ui-match` for UI).
3. **Compare** with the `parity-compare` skill, or hand both to the `parity-checker` agent.
4. **Record** the result in the matching `docs/*-parity.md` row (progress-doc standard).
A feature is not done until step 3 shows no differences, or the remaining ones are listed in
`docs/gaps.md`.

## Running and looking at the app
- `cargo run --release -p asenacad -- --sample --control PORT` (sample drawing + control channel;
  pick a free port).
- JSON lines on `127.0.0.1:PORT`:
  - `{"id":1,"method":"cmdline.input","params":{"text":"circle 0,0 5"}}` types at the command line.
  - `{"id":2,"method":"engine.execute","params":{"command":"line","params":{"points":[[0,0],[10,0]]}}}`
  - `{"id":3,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}`, then read the PNG.
  - Methods: `crates/ui-egui/src/control.rs`; protocol: `docs/control-protocol.md`.
- For UI work, look at the result (screenshot, read the PNG). If no frame is presented (screen
  locked) use `ui.render` or `asenacad-cli`.
- Headless: `asenacad-cli run --sample --script 'LINE 0,0 10,10\n' --export out.png`;
  `asenacad-cli info file.dxf`; `asenacad-cli mcp`.
- **The reference AutoCAD:** `asenascale windows|exec|shot "$ASENACAD_REF" …` (see the
  `autocad-observe` skill; the device name is in `plan/STATUS.md`). These commands need an
  AsenaScale release with the PC-to-PC commands (KaanAlper/AsenaScale#2); if the `asenascale` on
  your `PATH` is older, point `AS=/path/to/newer/asenascale` at a newer build (every script and
  skill calls `${AS:-asenascale}`).
- **AutoCAD helpers** in `tools/autocad/` (see its `README.md`): `shot-acad.sh` is the only way to
  screenshot AutoCAD (it finds AutoCAD's window and crops to it, so the owner's other windows are
  never captured); `uia-dump.ps1` dumps the UI Automation tree of AutoCAD's main window. Their
  output goes only to `plan/autocad/`.
- Parallel agents: separate `CARGO_TARGET_DIR=target/agent-<name>`, edit only the crates you own,
  delete your target dir when done.
- **Disk:** dev builds set `CARGO_PROFILE_DEV_DEBUG=line-tables-only`. Test executables in
  `target/debug/deps` take ~25 GB; after test/CI runs delete them:
  `find target/debug/deps -maxdepth 1 -type f -perm -u+x ! -name '*.so' -delete`.
- Shell gotcha: `mv`/`cp` may be aliased interactive — use `/bin/mv -f` / `/bin/cp -f`.

## Quality gate
`cargo xtask ci` (fmt, clippy -D warnings, tests, assets, layers, wasm) before every commit.
Before committing anything that touches assets, UI text or `docs/`, run the
`clean-room-auditor` agent on the diff.

## Upstream
We forked CADCraft and diverge; there are no regular merges. Taking a single upstream fix is
described in [`.claude/rules/cherry-pick.md`](.claude/rules/cherry-pick.md). Never open PRs or
issues against `storytold/cadcraft` from this repo without the owner asking.

## Progress docs
`ROADMAP.md` and `docs/*-parity.md` follow
[`docs/standards/progress-docs.md`](docs/standards/progress-docs.md). Update the affected rows,
status line and revision history in the same commit as the work.

## Contributor credits (About window)
About ▸ Contributors/Models are compiled from `contributors/contributors.json` (generated, never
hand-edit) and `contributors/people.toml` (names people chose for themselves); see
`docs/contributors.md`. When you prepare a PR for a contributor whose GitHub username has no
`[people.<username>]` entry, ask them once whether they want to be credited by more than their
username; if yes, add **only their own** entry (template at the top of the file). Never add, edit,
guess or copy anyone else's entry or name.
