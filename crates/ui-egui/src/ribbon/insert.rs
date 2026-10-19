//! The Insert tab: AutoCAD 2026's seven panels with their items in AutoCAD's order (column by
//! column, top to bottom), sizes, rows, split menus, slide-outs and dialog launchers.

use super::build::*;
use super::model::{Combo, Entry, Panel, Size, Slider, Tab, Toggle};
use crate::icons::Icon;

// ---- Block ----

const DETECT: &[Entry] = &[e(Icon::DetectBlocks, "Detect and Convert", "bdetect"), e(Icon::ConvertBlocks, "Search and Convert", "bconvert")];

const EDIT_ATTRIBUTE: &[Entry] = &[e(Icon::AttEdit, "Single", "attedit"), e(Icon::AttEditMultiple, "Multiple", "-attedit")];

const BLOCK: Panel = Panel {
    label: "Block",
    items: &[
        large_menu(Icon::Insert, "Insert", INSERT_GALLERY),
        large_split(None, DETECT),
        small_split(Some("Edit Attribute"), true, 1, EDIT_ATTRIBUTE),
        combo(Combo::AttributeDisplay, 165.0, 2),
        labeled(Icon::BlockReplace, "Replace", "breplace", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Block Definition ----

const CREATE_BLOCK: &[Entry] = &[e(Icon::BlockCreate, "Create Block", "block"), e(Icon::WriteBlock, "Write Block", "wblock")];

const BLOCK_DEFINITION: Panel = Panel {
    label: "Block Definition",
    items: &[
        large_split(None, CREATE_BLOCK),
        large(Icon::AttDef, "Define Attributes", "attdef"),
        large(Icon::AttManager, "Manage Attributes", "battman"),
        large(Icon::BlockEdit, "Block Editor", "bedit"),
    ],
    slideout: &[labeled(Icon::BasePoint, "Set Base Point", "base", 1), labeled(Icon::AttSync, "Synchronize", "attsync", 2)],
    launcher: None,
    launcher_name: None,
};

// ---- Reference ----

const REFERENCE: Panel = Panel {
    label: "Reference",
    items: &[
        large(Icon::Attach, "Attach", "attach"),
        large(Icon::XClip, "Clip", "clip"),
        large(Icon::UnderlayAdjust, "Adjust", "adjust"),
        labeled(Icon::UnderlayLayers, "Underlay Layers", "ulayers", 1),
        combo(Combo::XrefFrames, 114.0, 2),
        combo(Combo::UnderlaySnap, 157.0, 3),
    ],
    slideout: &[
        labeled(Icon::RefEdit, "Edit Reference", "refedit", 1),
        toggle(Icon::XrefFade, "Xref Fading Toggle Button", "xdwgfadectl", Size::Small { label: false }, 2, Toggle::XrefFade),
        slider(Slider::XrefFade, 200.0, 2),
    ],
    // AutoCAD's launcher opens the External References palette (XREF).
    launcher: Some("xref"),
    launcher_name: Some("External References"),
};

// ---- Import ----

const PDF_IMPORT: &[Entry] =
    &[e(Icon::PdfImport, "PDF Import", "pdfimport"), e(Icon::DgnImport, "DGN Import", "dgnimport"), e(Icon::Import, "Import", "import")];

const IMPORT: Panel = Panel {
    label: "Import",
    items: &[
        large_split(None, PDF_IMPORT),
        // AutoCAD draws a thin line between PDF Import and the SHX text tools.
        GROUP_SEP,
        labeled(Icon::RecognizeShx, "Recognize SHX Text", "pdfshxtext", 1),
        labeled(Icon::ShxSettings, "Recognition Settings", "pdfshxtext.settings", 2),
        large(Icon::CombineText, "Combine Text", "txt2mtxt"),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Data ----

const DATA: Panel = Panel {
    label: "Data",
    items: &[
        large(Icon::Field, "Field", "field"),
        labeled(Icon::UpdateFields, "Update Fields", "updatefield", 1),
        labeled(Icon::OleObject, "OLE Object", "insertobj", 2),
        labeled(Icon::Hyperlink, "Hyperlink", "hyperlink", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Linking & Extraction ----

const LINKING: Panel = Panel {
    label: "Linking & Extraction",
    items: &[
        large(Icon::DataLink, "Data Link", "datalink"),
        labeled(Icon::DataDownload, "Download from Source", "datalinkupdate", 1),
        labeled(Icon::DataUpload, "Upload to Source", "datalinkupdate.write", 2),
        // AutoCAD's label has two spaces.
        labeled(Icon::DataExtract, "Extract  Data", "dataextraction", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Location ----

/// Esri and Bing are online map services.
const SET_LOCATION: &[Entry] = &[
    e(Icon::GeoMap, "From Esri Maps", "geographiclocation.esri"),
    e(Icon::GeoFile, "From File", "geographiclocation.file"),
    SEP,
    e(Icon::GeoMap, "From Bing Maps", "geographiclocation.bing"),
];

const LOCATION: Panel =
    Panel { label: "Location", items: &[large_menu(Icon::GeoMap, "Set Location", SET_LOCATION)], slideout: &[], launcher: None, launcher_name: None };

pub const INSERT: Tab = Tab { label: "Insert", panels: &[BLOCK, BLOCK_DEFINITION, REFERENCE, IMPORT, DATA, LINKING, LOCATION] };
