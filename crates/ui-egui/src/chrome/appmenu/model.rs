//! What the Application menu lists: AutoCAD 2026's first-level entries in its order, each
//! sub-pane's entries and the command each one runs. The labels are AutoCAD's; the one-line
//! descriptions under them are our own words.

use serde_json::Value;

use crate::icons::Icon;

/// What choosing a sub-pane entry does.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Run the command as a menu does: without parameters, or with these JSON parameters.
    Command(&'static str, Option<&'static str>),
    /// A list of its own (the entry has a ▸); its main part runs `default`, or opens the list.
    Nested { entries: &'static [Sub], default: Option<&'static str> },
    /// Needs Autodesk's online services, which AsenaCAD doesn't use: always disabled.
    Online,
}

/// One row of a sub-pane.
#[derive(Clone, Copy, Debug)]
pub enum Sub {
    Entry { icon: Icon, label: &'static str, description: &'static str, action: Action },
    Separator,
}

/// A first-level entry: its main part runs `default` (none: it opens the sub-pane), the 20 px ▸
/// part opens `sub` (empty: the entry has no ▸).
#[derive(Clone, Copy, Debug)]
pub struct Top {
    pub icon: Icon,
    pub label: &'static str,
    /// The sub-pane's heading line (our own words).
    pub heading: &'static str,
    pub default: Option<&'static str>,
    pub sub: &'static [Sub],
}

#[derive(Clone, Copy, Debug)]
pub enum Row {
    Top(Top),
    Separator,
}

const fn run(icon: Icon, label: &'static str, description: &'static str, id: &'static str) -> Sub {
    Sub::Entry { icon, label, description, action: Action::Command(id, None) }
}

const fn with(icon: Icon, label: &'static str, description: &'static str, id: &'static str, params: &'static str) -> Sub {
    Sub::Entry { icon, label, description, action: Action::Command(id, Some(params)) }
}

const fn online(icon: Icon, label: &'static str, description: &'static str) -> Sub {
    Sub::Entry { icon, label, description, action: Action::Online }
}

const fn top(icon: Icon, label: &'static str, heading: &'static str, default: Option<&'static str>, sub: &'static [Sub]) -> Row {
    Row::Top(Top { icon, label, heading, default, sub })
}

/// The command `SAVE AS ▸ Save Layout as a Drawing` runs; it is enabled only on a layout.
pub const SAVE_LAYOUT: &str = "exportlayout";

const NEW: &[Sub] = &[
    run(Icon::New, "Drawing", "Start a new drawing from the default template.", "new"),
    run(Icon::SheetSet, "Sheet Set", "Set up a new sheet set.", "newsheetset"),
];

const SAMPLES: &[Sub] = &[run(Icon::SampleFiles, "Bracket", "Open AsenaCAD's sample drawing of a bracket.", "ui.sample")];

const OPEN: &[Sub] = &[
    run(Icon::Open, "Drawing", "Open a drawing file.", "open"),
    online(Icon::WebMobile, "Drawing from AutoCAD Web & Mobile", "Open a drawing kept in Autodesk's online storage."),
    run(Icon::SheetSet, "Sheet Set", "Open a sheet set file.", "opensheetset"),
    run(Icon::DgnImport, "DGN", "Bring a MicroStation DGN file in as a new drawing.", "dgnimport"),
    Sub::Separator,
    Sub::Entry {
        icon: Icon::SampleFiles,
        label: "Sample Files",
        description: "Drawings to look at and try things on.",
        action: Action::Nested { entries: SAMPLES, default: None },
    },
];

const SAVE_AS: &[Sub] = &[
    with(Icon::SaveAs, "Drawing", "Save a copy as a DWG drawing.", "ui.saveas", r#"{"ext":"dwg"}"#),
    online(Icon::WebMobile, "Drawing to AutoCAD Web & Mobile", "Save a copy in Autodesk's online storage."),
    with(Icon::SaveTemplate, "Drawing Template", "Keep this drawing's settings as a DWT to start others from.", "ui.saveas", r#"{"ext":"dwt"}"#),
    run(Icon::SaveStandards, "Drawing Standards", "Save the drawing's styles and layers as a standards file (DWS).", "saveas.dws"),
    run(Icon::SaveOther, "Other Formats", "Save a copy as DWG or DXF, in the version you pick.", "saveas"),
    run(Icon::SaveLayout, "Save Layout as a Drawing", "Make a new drawing from what the current layout shows.", SAVE_LAYOUT),
    run(Icon::DwgConvert, "DWG Convert", "Convert drawing files to another DWG version.", "dwgconvert"),
];

const IMPORT: &[Sub] = &[
    run(Icon::PdfImport, "PDF", "Bring the geometry and text of a PDF page into the drawing.", "pdfimport"),
    run(Icon::DgnImport, "DGN", "Bring a MicroStation DGN file into the drawing.", "dgnimport"),
    run(Icon::Import, "Other Formats", "Bring in a file of another format.", "import"),
];

const EXPORT: &[Sub] = &[
    run(Icon::ExportDwf, "DWF", "Write the drawing as a DWF file.", "exportdwf"),
    run(Icon::ExportDwfx, "DWFx", "Write the drawing as a DWFx file.", "exportdwfx"),
    run(Icon::Export3dDwf, "3D DWF", "Write the 3D model as a 3D DWF file.", "3ddwf"),
    run(Icon::ExportPdf, "PDF", "Write the drawing as a PDF file.", "exportpdf"),
    run(Icon::ExportDgn, "DGN", "Write the drawing as a MicroStation DGN file.", "dgnexport"),
    run(Icon::Export, "Other Formats", "Write the drawing in another file format.", "export"),
];

const PUBLISH: &[Sub] = &[
    online(Icon::PrintService3d, "Send to 3D Print Service", "Send the model to an online 3D printing service."),
    run(Icon::Archive, "Archive", "Pack a sheet set and its files into one archive.", "archive"),
    run(Icon::ETransmit, "eTransmit", "Gather the drawing and the files it uses into a package.", "etransmit"),
    run(Icon::Email, "Email", "Attach the drawing to a new e-mail message.", "mail"),
    online(Icon::SharedViews, "Share View", "Publish views of the drawing for others to see online."),
    online(Icon::Share, "Share Drawing", "Hand out a link to the drawing through Autodesk's cloud."),
];

const PRINT: &[Sub] = &[
    run(Icon::Plot, "Plot", "Put the current layout on paper or into a plot file.", "plot"),
    run(Icon::Publish, "Batch Plot", "Plot several sheets in one go.", "publish"),
    run(Icon::Preview, "Plot Preview", "Check on screen how the sheet will come out.", "preview"),
    run(Icon::ViewDetails, "View Plot and Publish Details", "List what the last plots and publishes did.", "viewplotdetails"),
    run(Icon::PageSetup, "Page Setup", "Choose the paper, plotter and scale each layout plots with.", "pagesetup"),
    run(Icon::Print3d, "3D Print", "Prepare the model for a 3D printer.", "3dprint"),
    run(Icon::PlotterManager, "Manage Plotters", "Add and set up plotters.", "plottermanager"),
    run(Icon::PlotStyles, "Manage Plot Styles", "Add and change plot style tables.", "stylesmanager"),
    run(Icon::EditPlotStyles, "Edit Plot Style Tables", "Open a plot style table in its editor.", "stylesmanager.edit"),
];

const RECOVER: &[Sub] = &[
    run(Icon::Recover, "Recover", "Fix what it can in a broken drawing, then load it.", "recover"),
    run(Icon::RecoverXrefs, "Recover with Xrefs", "Repair a drawing and the drawings it references.", "recoverall"),
];

const UTILITIES: &[Sub] = &[
    run(Icon::DrawingProperties, "Drawing Properties", "Title, author, keywords and other facts about the drawing.", "dwgprops"),
    run(Icon::DwgCompare, "DWG Compare", "Show what differs between this drawing and another one.", "compare"),
    run(Icon::Units, "Units", "Choose how lengths and angles are shown.", "units"),
    run(Icon::Audit, "Audit", "Check the drawing for errors and fix them.", "audit"),
    run(Icon::Status, "Status", "List the drawing's limits, extents and settings.", "status"),
    run(Icon::Purge, "Purge", "Remove unused layers, blocks and styles.", "purge"),
    Sub::Entry {
        icon: Icon::Recover,
        label: "Recover",
        description: "Repair damaged drawing files.",
        action: Action::Nested { entries: RECOVER, default: Some("recover") },
    },
    run(Icon::RecoveryManager, "Open the Drawing Recovery Manager", "List the backup and autosave copies to recover from.", "drawingrecovery"),
];

const CLOSE: &[Sub] = &[
    run(Icon::CloseDrawing, "Current Drawing", "Close this drawing.", "close"),
    run(Icon::CloseAll, "All Drawings", "Close every open drawing.", "closeall"),
];

/// The first-level list, in AutoCAD's order.
pub const ROWS: &[Row] = &[
    top(Icon::New, "New", "Begin a new drawing or sheet set", Some("new"), NEW),
    top(Icon::Open, "Open", "Open drawings, sheet sets and other files", Some("open"), OPEN),
    top(Icon::Save, "Save", "", Some("qsave"), &[]),
    top(Icon::SaveAs, "Save As", "Keep a copy under another name or format", Some("saveas"), SAVE_AS),
    top(Icon::Import, "Import", "Bring other file formats into this drawing", Some("import"), IMPORT),
    top(Icon::Export, "Export", "Write this drawing in another format", Some("export"), EXPORT),
    Row::Separator,
    top(Icon::Publish, "Publish", "Hand the drawing on to other people", None, PUBLISH),
    top(Icon::Plot, "Print", "Send sheets to paper, a plotter or a file", Some("plot"), PRINT),
    top(Icon::DrawingUtilities, "Drawing Utilities", "Look after the drawing file and its settings", None, UTILITIES),
    Row::Separator,
    top(Icon::CloseDrawing, "Close", "Close the drawings you are done with", Some("close"), CLOSE),
];

/// The first-level entries without the separators, in order.
pub fn tops() -> impl Iterator<Item = &'static Top> {
    ROWS.iter().filter_map(|r| match r {
        Row::Top(t) => Some(t),
        Row::Separator => None,
    })
}

/// The JSON parameters of a command entry (`Null` for a menu-style start).
pub fn params(raw: Option<&str>) -> Value {
    raw.and_then(|s| serde_json::from_str(s).ok()).unwrap_or(Value::Null)
}

/// Every command id the menu names, for tests and reports.
pub fn command_ids() -> Vec<&'static str> {
    fn walk(entries: &[Sub], out: &mut Vec<&'static str>) {
        for e in entries {
            match e {
                Sub::Entry { action: Action::Command(id, _), .. } => out.push(id),
                Sub::Entry { action: Action::Nested { entries, default }, .. } => {
                    out.extend(*default);
                    walk(entries, out);
                }
                Sub::Entry { action: Action::Online, .. } | Sub::Separator => {}
            }
        }
    }
    let mut out = Vec::new();
    for t in tops() {
        out.extend(t.default);
        walk(t.sub, &mut out);
    }
    out
}

/// Every label and description the menu shows, for the translation check.
pub fn labels() -> Vec<&'static str> {
    fn walk(entries: &[Sub], out: &mut Vec<&'static str>) {
        for e in entries {
            if let Sub::Entry { label, description, action, .. } = e {
                out.push(label);
                out.push(description);
                if let Action::Nested { entries, .. } = action {
                    walk(entries, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    for t in tops() {
        out.push(t.label);
        out.push(t.heading);
        walk(t.sub, &mut out);
    }
    out
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Entries naming an AutoCAD command AsenaCAD doesn't have yet: shown disabled with "Not
    /// available yet". docs/ui-parity.md's "Application menu items without a command" lists
    /// exactly these.
    pub(crate) const UNBUILT: &[&str] = &[
        "newsheetset",        // New ▸ Sheet Set
        "opensheetset",       // Open ▸ Sheet Set
        "dgnimport",          // Open ▸ DGN, Import ▸ DGN
        "saveas.dws",         // Save As ▸ Drawing Standards (SAVEAS to a .dws file)
        "exportlayout",       // Save As ▸ Save Layout as a Drawing
        "dwgconvert",         // Save As ▸ DWG Convert
        "pdfimport",          // Import ▸ PDF
        "exportdwf",          // Export ▸ DWF
        "exportdwfx",         // Export ▸ DWFx
        "3ddwf",              // Export ▸ 3D DWF
        "dgnexport",          // Export ▸ DGN
        "archive",            // Publish ▸ Archive
        "etransmit",          // Publish ▸ eTransmit
        "mail",               // Publish ▸ Email (id unverified)
        "viewplotdetails",    // Print ▸ View Plot and Publish Details
        "3dprint",            // Print ▸ 3D Print
        "stylesmanager.edit", // Print ▸ Edit Plot Style Tables (id unverified)
        "recoverall",         // Drawing Utilities ▸ Recover ▸ Recover with Xrefs
        "dwgprops",           // Drawing Utilities ▸ Drawing Properties
    ];

    #[test]
    fn every_command_exists_or_is_listed_as_unbuilt() {
        let ids = command_ids();
        let missing: Vec<_> = ids.iter().filter(|id| !crate::palettes::command_known(id) && !UNBUILT.contains(id)).collect();
        assert!(missing.is_empty(), "add these to UNBUILT or build them: {missing:?}");
        let stale: Vec<_> = UNBUILT.iter().filter(|id| crate::palettes::command_known(id)).collect();
        assert!(stale.is_empty(), "these exist now, remove them from UNBUILT: {stale:?}");
        let unused: Vec<_> = UNBUILT.iter().filter(|id| !ids.contains(id)).collect();
        assert!(unused.is_empty(), "no menu entry uses these, remove them from UNBUILT: {unused:?}");
    }

    /// docs/ui-parity.md's "Application menu items without a command" names exactly `UNBUILT`.
    #[test]
    fn the_parity_doc_lists_the_unbuilt_entries() {
        let doc = include_str!("../../../../../docs/ui-parity.md");
        let section = doc.split("### Application menu items without a command").nth(1).and_then(|s| s.split("\n## ").next()).unwrap_or("");
        let section = section.split("\n### ").next().unwrap_or("");
        let mut listed: Vec<&str> = section.lines().filter_map(|l| l.strip_prefix("| `")).filter_map(|l| l.split('`').next()).collect();
        let mut unbuilt = UNBUILT.to_vec();
        listed.sort_unstable();
        unbuilt.sort_unstable();
        assert_eq!(listed, unbuilt, "update docs/ui-parity.md's \"Application menu items without a command\" table to match UNBUILT");
    }

    #[test]
    fn json_parameters_parse() {
        assert_eq!(params(Some(r#"{"ext":"dwt"}"#))["ext"], "dwt");
        assert!(params(None).is_null());
        assert!(params(Some("not json")).is_null());
    }
}
