//! The Manage tab: AutoCAD 2026's six panels with their items in AutoCAD's order (column by
//! column, top to bottom), sizes, rows, split menus and slide-outs.

use super::build::*;
use super::model::{Combo, Entry, Panel, Tab};
use crate::icons::Icon;

// ---- Action Recorder ----

/// AutoCAD's slide-out holds the Action Tree (the recorded macro's steps): an empty, disabled box
/// of its width until the Action Recorder exists.
const ACTION_RECORDER: Panel = Panel {
    label: "Action Recorder",
    items: &[
        large(Icon::Record, "Record", "actrecord"),
        icon(Icon::InsertMessage, "Insert Message", "actusermessage", 1),
        icon(Icon::InsertBasePoint, "Insert Base Point", "actbasepoint", 2),
        icon(Icon::PauseInput, "Pause for User Input", "actuserinput", 3),
        GROUP_SEP,
        labeled(Icon::PlayMacro, "Play", "actplay", 1),
        icon(Icon::ActPreferences, "Preference", "actpreferences", 1),
        icon(Icon::ActManager, "Manage Action Macros", "actmanager", 1),
        combo(Combo::ActionMacro, 125.0, 2),
    ],
    slideout: &[combo(Combo::ActionTree, 250.0, 1)],
    launcher: None,
    launcher_name: None,
};

// ---- Customization ----

/// AutoCAD's Edit Aliases has a ▾ whose entries weren't captured: only its face is listed.
const EDIT_ALIASES: &[Entry] = &[e(Icon::EditAliases, "Edit Aliases", "editaliases")];

const CUSTOMIZATION: Panel = Panel {
    label: "Customization",
    items: &[
        large(Icon::CuiEditor, "User Interface", "cui"),
        // CUSTOMIZE: the tool palettes' Customize dialog.
        large(Icon::CustomizePalettes, "Tool Palettes", "customize"),
        labeled(Icon::CuiImport, "Import", "cuiimport", 1),
        labeled(Icon::CuiExport, "Export", "cuiexport", 2),
        small_split(None, true, 3, EDIT_ALIASES),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Applications ----

const APPLICATIONS: Panel = Panel {
    label: "Applications",
    items: &[
        large(Icon::LoadApp, "Load Application", "appload"),
        large(Icon::RunScript, "Run Script", "script"),
        labeled(Icon::VbaEditor, "Visual Basic Editor", "vbaide", 1),
        labeled(Icon::VlispEditor, "Visual LISP Editor", "vlisp", 2),
        labeled(Icon::VbaRun, "Run VBA Macro", "vbarun", 3),
    ],
    slideout: &[labeled(Icon::VbaLoad, "Load Project", "vbaload", 1), labeled(Icon::VbaManager, "VBA Manager", "vbaman", 2)],
    launcher: None,
    launcher_name: None,
};

// ---- CAD Standards ----

const CAD_STANDARDS: Panel = Panel {
    label: "CAD Standards",
    items: &[
        labeled(Icon::LayerTranslator, "Layer Translator", "laytrans", 1),
        labeled(Icon::CheckStandards, "Check", "checkstandards", 2),
        labeled(Icon::ConfigureStandards, "Configure", "standards", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Cleanup ----

const CLEANUP: Panel = Panel {
    label: "Cleanup",
    items: &[
        // PURGE opening on its Find Non-Purgeable Items view.
        large(Icon::FindNonPurgeable, "Find Non-Purgeable Items", "purge.nonpurgeable"),
        labeled(Icon::Purge, "Purge", "purge", 1),
        labeled(Icon::Overkill, "Overkill", "overkill", 2),
        labeled(Icon::Audit, "Audit", "audit", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Performance ----

const PERFORMANCE: Panel = Panel {
    label: "Performance",
    items: &[large(Icon::PerformanceAnalyzer, "Performance Analyzer", "performanceanalyzer")],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

pub const MANAGE: Tab = Tab { label: "Manage", panels: &[ACTION_RECORDER, CUSTOMIZATION, APPLICATIONS, CAD_STANDARDS, CLEANUP, PERFORMANCE] };
