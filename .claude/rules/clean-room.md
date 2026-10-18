# Clean-room

AsenaCAD copies AutoCAD's **behaviour**, never Autodesk's **material**.

## May be the same as AutoCAD
- Command names, aliases, options, keywords, defaults, the order and wording of command-line
  prompts (they are the interface people type against).
- Layout and structure of the UI: ribbon tabs and panels, which tools sit where, dialog layouts,
  status-bar toggles, keyboard shortcuts.
- File formats, from public specs only: Autodesk's published DXF Reference, the Open Design
  Alliance's public DWG specification, our own synthetic test files.

## Never copied, never committed
- Icons, images, splash screens, colours/artwork taken from AutoCAD screenshots (we pick our own
  colours and draw icons in code; matching *placement* and *meaning* is fine).
- SHX fonts, `.pat` hatch patterns, `.lin` linetypes, `.dwt` templates, CUI/CUIx, `acad.pgp`,
  plot styles shipped with AutoCAD, sample drawings.
- Help text, tooltips' long descriptions, documentation prose.
- Anything inside the AutoCAD install folder beyond file **names** (no reading, no disassembly).
- Screenshots of AutoCAD, its command logs, UI Automation dumps and files it produced: only under
  `plan/autocad/`, which is gitignored. Never paste them into issues, PRs, commit messages or docs.

## Code we may not use
GPL/LGPL/AGPL CAD code — LibreDWG, LibreCAD, QCAD, FreeCAD (incl. planegcs), SolveSpace,
OpenCASCADE — not linked, not copied, not read.

## Before a commit
If the diff touches assets, UI strings, `docs/` or test fixtures, run the `clean-room-auditor`
agent and fix what it reports.
