---
name: ui-match
description: Rebuild one Windows AutoCAD UI element (ribbon tab/panel, Application menu, Quick Access toolbar, command line, status bar, palette, dialog) in AsenaCAD's egui UI from observation, without copying Autodesk artwork. Use for any M1 UI-parity task.
---

# Match one UI element

Target: Autodesk AutoCAD 2026 (Windows).

1. **Observe** (`autocad-observe`): screenshots of the element in its default state and in each
   state you will build (hover/open/disabled), plus a UI Automation dump (its step 7) for the
   element's structure. Write in `plan/autocad/notes/ui-<element>.md`:
   the element's children in order, each child's label, control type (button, split button,
   drop-down, toggle, text field), which command it runs, size class (large/small button),
   grouping and separators, keyboard access (KeyTips, shortcuts).
2. **Find the code:** `crates/ui-egui/src/` (`chrome.rs`, `menus.rs`, `cmdline.rs`,
   `docking.rs`, `palettes/`, `dialogs.rs`). Read how an existing element is built and tested
   (`crates/ui-egui/tests/`), and the colour tokens in `theme.rs`.
3. **Build** from the note, never from the screenshot pixels: our own icons drawn in
   `crates/ui-egui/src/icons.rs`, our own colours from `theme::Tokens`. Every item calls an
   existing command (`app.start(id)`/`app.run`); if the command does not exist, add it with the
   `add-command` skill first or leave the item disabled and list it in `docs/gaps.md`.
4. **New UI strings** must be added to every catalog in `crates/ui-egui/src/i18n/*.tsv`
   (`every_marked_ui_literal_has_a_translation`); use the English text in other languages until
   translated, and say so in the commit.
5. **Test:** an `egui_kittest` test that the element renders its children in the observed order
   and that clicking a child starts the right command.
6. **Compare** (`parity-compare`, UI part) and record the row in `docs/ui-parity.md`.
7. `cargo xtask ci`, run `clean-room-auditor`, commit: `feat(ui): <element> matches AutoCAD`.
