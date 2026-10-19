---
name: add-command
description: Add or complete an AutoCAD command in AsenaCAD end to end — CommandSpec, interactive prompt machine matching AutoCAD's prompts, JSON form, tests, parity row. Use for any new command or missing command option.
---

# Add or complete a command

1. **Observe first** (`autocad-observe`): one script per option path you will build. The note in
   `plan/autocad/notes/<area>.md` is your spec: prompts, keywords, defaults, results.
2. **Find the neighbours:** `crates/engine/src/cmd/` holds one module per area; read a similar
   command there and its tests before writing (pattern: `CommandSpec` with id = AutoCAD's name in
   lower case, label, menu path, shortcut, aliases, params doc, `enabled`, JSON `run`,
   optional `interactive`). Register it where that module registers its siblings.
3. **Write the tests first** (in the module's `#[cfg(test)]` or `crates/engine/tests/`):
   - JSON form: `session.execute("<id>", &json!({...}))` produces the expected entities.
   - Interactive form: feeding the observed inputs through `Session::input` yields AutoCAD's
     prompt sequence (exact strings from the note) and the same entities.
   - Hostile params never panic (copy the `hostile_params_never_panic` pattern).
   Run `cargo test -p asenacad-engine <test>` and see them fail.
4. **Implement** the minimum to pass. Never-crash rules apply (`docs/standards/never-crash.md`).
   Programmatic (JSON) calls never open dialogs.
5. **Wire the UI** only through the command: ribbon/menu entries call `app.start(id)`; no logic in
   `ui-egui`.
6. **Compare** (`parity-compare`) and **record** the parity row.
7. `cargo xtask ci`, commit: `feat(engine): <COMMAND> <what>`.
