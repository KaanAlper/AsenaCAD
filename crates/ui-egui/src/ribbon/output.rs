//! The Output and Collaborate tabs: AutoCAD 2026's panels with their items in AutoCAD's order
//! (column by column, top to bottom), sizes, rows, split menus and dialog launchers.
//!
//! Collaborate's tools use Autodesk's online services (sharing, Autodesk Docs, Traces, markup
//! import), which AsenaCAD doesn't use: they are shown disabled so the tab keeps AutoCAD's
//! structure. DWG Compare runs COMPARE.

use super::build::*;
use super::model::{Combo, Entry, Panel, Tab, Toggle};
use crate::icons::Icon;

// ---- Output ▸ Plot ----

const PLOT: Panel = Panel {
    label: "Plot",
    items: &[
        large(Icon::Plot, "Plot", "plot"),
        // AutoCAD's Batch Plot opens the Publish dialog.
        large(Icon::Publish, "Batch Plot", "publish"),
        large(Icon::Preview, "Preview", "preview"),
        labeled(Icon::PageSetup, "Page Setup Manager", "pagesetup", 1),
        labeled(Icon::ViewDetails, "View Details", "viewplotdetails", 2),
        labeled(Icon::PlotterManager, "Plotter Manager", "plottermanager", 3),
    ],
    slideout: &[],
    launcher: Some("plot"),
    launcher_name: Some("Plot, Plot Options Dialog"),
};

// ---- Output ▸ Export to DWF/PDF ----

const EXPORT: &[Entry] = &[e(Icon::ExportDwfx, "DWFx", "exportdwfx"), e(Icon::ExportDwf, "DWF", "exportdwf"), e(Icon::ExportPdf, "PDF", "exportpdf")];

/// The two lists choose what the export takes (the display, the extents or a window) and which
/// page setup it uses; the small tools next to them pick the window, override the page setup,
/// preview the export and open the DWF and PDF options. None of these settings exists yet.
const EXPORT_PANEL: Panel = Panel {
    label: "Export to DWF/PDF",
    items: &[
        large_split(Some("Export"), EXPORT),
        combo(Combo::ExportArea, 200.0, 1),
        icon(Icon::ExportWindow, "Export Window", "eplotexport.window", 1),
        combo(Combo::ExportPageSetup, 200.0, 2),
        icon(Icon::ExportPageSetup, "Page Setup", "eplotexport.pagesetup", 2),
        icon(Icon::ExportPreview, "Preview", "eplotexport.preview", 3),
        icon(Icon::DwfOptions, "Export to DWF Options", "exportsettings.dwf", 3),
        icon(Icon::PdfOptions, "Export to PDF Options", "exportsettings.pdf", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

pub const OUTPUT: Tab = Tab { label: "Output", panels: &[PLOT, EXPORT_PANEL] };

// ---- Collaborate ----

const SHARE: Panel = Panel {
    label: "Share",
    items: &[large(Icon::Share, "Share Drawing", "share"), large_toggle(Icon::SharedViews, "Shared Views", "sharedviews", Toggle::NotBuilt)],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

const AUTODESK_DOCS: Panel = Panel {
    label: "Autodesk Docs",
    items: &[large_toggle(Icon::PushToDocs, "Push to Autodesk Docs", "pushtodocsopen", Toggle::NotBuilt)],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

const TRACES: Panel = Panel {
    label: "Traces",
    items: &[
        large_toggle(Icon::TracesPalette, "Traces Palette", "tracepaletteopen", Toggle::NotBuilt),
        large(Icon::MarkupImport, "Markup Import", "markupimport"),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

const COMPARE: Panel =
    Panel { label: "Compare", items: &[large(Icon::DwgCompare, "DWG Compare", "compare")], slideout: &[], launcher: None, launcher_name: None };

pub const COLLABORATE: Tab = Tab { label: "Collaborate", panels: &[SHARE, AUTODESK_DOCS, TRACES, COMPARE] };
