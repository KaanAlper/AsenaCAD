//! Data model of the ribbon: static tables, so tests and the i18n checks can walk them.
//!
//! Layout rule for small items (what the widgets and `layout.rs` follow): consecutive small
//! items whose `row` never decreases form one block of up to three rows (slide-outs: four);
//! within a block the items of a row flow left to right. A row number lower than the previous
//! item's, or a large item, starts the next block to the right. Large items fill the content
//! height and use `row: 0`.

use crate::icons::Icon;

/// Large: 72 px tall, 32 px icon, label under it (may wrap to 2 lines).
/// Small: 22 px row item, 16 px icon, optionally with the label to the right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Large,
    Small { label: bool },
}

/// One entry of a split button's or drop-down's menu.
///
/// Every AutoCAD variant (Circle "2-Point", Arc "Start, End, Radius", "Polar Array", …) is its
/// own command id in the engine (`circle.2p`, `arc.ser`, `arraypolar`), so an entry only names a
/// command; widgets run it with `app.start(cmd)`.
#[derive(Clone, Copy, Debug)]
pub enum Entry {
    Command { icon: Icon, label: &'static str, cmd: &'static str },
    Separator,
}

/// What a combo box (or data-bound drop-down) shows and edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combo {
    Layer,
    Color,
    Linetype,
    Lineweight,
    TextStyle,
    DimStyle,
    MLeaderStyle,
    TableStyle,
    LayerState,
    PlotStyle,
    /// ByLayer / ByBlock / value: how new objects get their transparency (CETRANSPARENCY).
    Transparency,
    /// Retain / display all / hide all attributes (ATTMODE, the ATTDISP command).
    AttributeDisplay,
    SectionViewStyle,
    DetailViewStyle,
    /// Whether xref, image and underlay frames show and plot (FRAME).
    XrefFrames,
    /// Whether object snaps find points on underlays (UOSNAP).
    UnderlaySnap,
    /// A text field: Enter finds the typed text in the drawing (FIND).
    FindText,
    /// An editable box: the height new text starts with (TEXTSIZE).
    TextHeight,
    /// The layer new text, dimensions, centerlines and multileaders go on (AutoCAD's "Use
    /// Current" or a layer).
    TextLayer,
    DimLayer,
    CenterLayer,
    MLeaderLayer,
    /// The model or layout views saved in the drawing (VIEW); choosing one restores it.
    NamedView,
    /// The recorded action macros to play (the Action Recorder isn't built: empty, disabled).
    ActionMacro,
    /// What Output ▸ Export takes: the display, the extents or a window (not built: disabled).
    ExportArea,
    /// Whether Output ▸ Export uses the current page setup or an override (not built: disabled).
    ExportPageSetup,
}

impl Combo {
    /// The control's name (accessibility, control channel), AutoCAD's UI Automation name. The
    /// box itself shows the value; some of AutoCAD's controls are named after their default value
    /// ("Retain Attribute Display", "*Frames vary*").
    pub const fn label(self) -> &'static str {
        match self {
            Self::Layer => "Layer",
            Self::Color => "Object Color",
            Self::Linetype => "Linetype",
            Self::Lineweight => "Lineweight",
            Self::TextStyle => "Text Style",
            Self::DimStyle => "Dimension Style",
            Self::MLeaderStyle => "Multileader Style",
            Self::TableStyle => "Table Style",
            Self::LayerState => "Layer State",
            Self::PlotStyle => "Plot Style",
            Self::Transparency => "Transparency Mode",
            Self::AttributeDisplay => "Retain Attribute Display",
            Self::SectionViewStyle => "Section View Style",
            Self::DetailViewStyle => "Detail View Style",
            Self::XrefFrames => "*Frames vary*",
            Self::UnderlaySnap => "Snap to Underlays ON",
            Self::FindText => "Find Text",
            Self::TextHeight => "Annotation Text Height",
            Self::TextLayer => "Text Layer Gallery",
            Self::DimLayer => "Dim Layer Gallery",
            Self::CenterLayer => "Center Layer Gallery",
            Self::MLeaderLayer => "Mleader Layer Gallery",
            Self::NamedView => "Named Views",
            Self::ActionMacro => "Available Action Macro",
            // AutoCAD names both export lists "EplotExport".
            Self::ExportArea | Self::ExportPageSetup => "EplotExport",
        }
    }
}

/// A slider with a value box at its right end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Slider {
    /// How much locked layers fade (LAYLOCKFADECTL, 0–90 %).
    LockedLayerFade,
    /// Transparency of new objects (CETRANSPARENCY, 0–90).
    Transparency,
    /// How much xrefs fade (XDWGFADECTL, 0–90 %).
    XrefFade,
}

impl Slider {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LockedLayerFade => "Layer fading",
            Self::Transparency => "Transparency",
            Self::XrefFade => "Xref fading",
        }
    }
}

/// What a toggle button's pressed look follows (read by `widgets::is_on`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Toggle {
    /// The Layer Properties Manager is open.
    LayerPalette,
    /// Locked layers are faded (LAYLOCKFADECTL > 0).
    LockedLayerFade,
    /// Picking a group member selects the group (PICKSTYLE bit 1).
    GroupSelection,
    /// Selected groups show one bounding box (GROUPDISPLAYMODE 2).
    GroupBox,
    /// The QuickCalc palette is open.
    QuickCalcPalette,
    /// Xrefs are faded (XDWGFADECTL > 0).
    XrefFade,
    /// The Parameters Manager is open.
    ParametersPalette,
    /// New dimensional constraints are dynamic (CCONSTRAINTFORM 0, AutoCAD's default).
    DynamicConstraints,
    /// New dimensional constraints are annotational (CCONSTRAINTFORM 1).
    AnnotationalConstraints,
    /// The UCS icon is shown in the drawing area.
    UcsIcon,
    /// The ViewCube is shown.
    ViewCube,
    /// The navigation bar is shown (NAVBAR).
    NavBar,
    /// The command line is shown.
    CommandLine,
    /// The drawing (file) tabs are shown.
    FileTabs,
    /// The layout tabs are shown on the status bar.
    LayoutTabs,
    /// The Properties palette is open.
    PropertiesPalette,
    /// The Blocks palette is open.
    BlocksPalette,
    /// The External References palette is open.
    XrefPalette,
    /// The palette or display the button opens isn't built yet: never shown pressed.
    NotBuilt,
}

#[derive(Clone, Copy, Debug)]
pub enum Item {
    Button {
        icon: Icon,
        label: &'static str,
        cmd: &'static str,
        size: Size,
        row: u8,
    },
    /// A button that shows pressed while `state` is on; clicking runs `cmd` (which flips it).
    Toggle {
        icon: Icon,
        label: &'static str,
        cmd: &'static str,
        size: Size,
        row: u8,
        state: Toggle,
    },
    /// Face runs the last-used entry (initially `menu[default]`); ▾ opens the menu.
    /// `label: None` — the face shows the remembered entry's label (Trim → Extend);
    /// `Some(l)` — the face keeps `l` and only its icon follows the entry (Circle, Arc, Array).
    Split {
        label: Option<&'static str>,
        size: Size,
        row: u8,
        menu: &'static [Entry],
        default: usize,
    },
    /// A drop-down button: the whole button opens `menu`, it has no face that runs a command.
    /// Its ▾ sits under a one-line label and right of a two-line one (Set Location, Insert's
    /// block gallery).
    Menu {
        icon: Icon,
        label: &'static str,
        size: Size,
        row: u8,
        menu: &'static [Entry],
    },
    Combo {
        kind: Combo,
        width: f32,
        row: u8,
    },
    Slider {
        kind: Slider,
        width: f32,
        row: u8,
    },
    /// A thin vertical line between two groups of a panel's columns, a column of its own
    /// (AutoCAD draws one between Parametric ▸ Geometric's constraint icons and its Show/Hide
    /// column).
    Separator,
}

#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub label: &'static str,
    pub items: &'static [Item],
    pub slideout: &'static [Item],
    /// Dialog launcher arrow in the title bar: the command it runs.
    pub launcher: Option<&'static str>,
    /// The launcher's accessible name and tooltip, AutoCAD's (the dialog it opens); `None` uses
    /// the command's label.
    pub launcher_name: Option<&'static str>,
}

impl Panel {
    /// The launcher's accessible name: [`Panel::launcher_name`], else its command's label, else
    /// the panel's name.
    pub fn launcher_label(&self) -> Option<&'static str> {
        let cmd = self.launcher?;
        Some(self.launcher_name.or_else(|| cadcraft_engine::find_command(cmd).map(|c| c.label)).unwrap_or(self.label))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tab {
    pub label: &'static str,
    pub panels: &'static [Panel],
}

/// Ribbon tabs in AutoCAD's order.
pub fn tabs() -> &'static [Tab] {
    TABS
}

const TABS: &[Tab] = &[
    crate::ribbon::home::HOME,
    crate::ribbon::insert::INSERT,
    crate::ribbon::annotate::ANNOTATE,
    crate::ribbon::parametric::PARAMETRIC,
    crate::ribbon::view::VIEW,
    crate::ribbon::manage::MANAGE,
    crate::ribbon::output::OUTPUT,
    crate::ribbon::output::COLLABORATE,
    crate::ribbon::express::EXPRESS,
];

/// Whether `name` is one of the ribbon's tabs (a saved `ribbonTab` must be).
pub fn is_tab(name: &str) -> bool {
    tabs().iter().any(|t| t.label == name)
}

fn entry_commands(menu: &'static [Entry]) -> impl Iterator<Item = &'static str> {
    menu.iter().filter_map(|e| match e {
        Entry::Command { cmd, .. } => Some(*cmd),
        Entry::Separator => None,
    })
}

/// Every command id the ribbon references (faces, menus, launchers), for tests and reports.
pub fn command_ids() -> Vec<&'static str> {
    let mut ids = Vec::new();
    for tab in tabs() {
        for panel in tab.panels {
            for item in panel.items.iter().chain(panel.slideout) {
                match item {
                    Item::Button { cmd, .. } | Item::Toggle { cmd, .. } => ids.push(*cmd),
                    Item::Split { menu, .. } | Item::Menu { menu, .. } => ids.extend(entry_commands(menu)),
                    Item::Combo { .. } | Item::Slider { .. } | Item::Separator => {}
                }
            }
            ids.extend(panel.launcher);
        }
    }
    ids
}

/// Every user-visible label of the ribbon (tabs, panels, items, menu entries, control names,
/// minimize states),
/// for the translation coverage check.
pub fn labels() -> Vec<&'static str> {
    let mut out = Vec::new();
    for tab in tabs() {
        out.push(tab.label);
        for panel in tab.panels {
            out.push(panel.label);
            out.extend(panel.launcher_name);
            for item in panel.items.iter().chain(panel.slideout) {
                match item {
                    Item::Button { label, .. } | Item::Toggle { label, .. } => out.push(*label),
                    Item::Menu { label, menu, .. } => {
                        out.push(*label);
                        out.extend(menu.iter().filter_map(|e| match e {
                            Entry::Command { label, .. } => Some(*label),
                            Entry::Separator => None,
                        }));
                    }
                    Item::Split { label, menu, .. } => {
                        out.extend(*label);
                        out.extend(menu.iter().filter_map(|e| match e {
                            Entry::Command { label, .. } => Some(*label),
                            Entry::Separator => None,
                        }));
                    }
                    Item::Combo { kind, .. } => out.push(kind.label()),
                    Item::Slider { kind, .. } => out.push(kind.label()),
                    Item::Separator => {}
                }
            }
        }
    }
    out.extend(crate::ribbon::RibbonMin::ALL.iter().map(|s| s.label()));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ribbon items that reference an AutoCAD command AsenaCAD doesn't have yet.
    /// They render disabled; docs/ui-parity.md lists them. Shrink this list, never grow it silently.
    pub(crate) const UNBUILT: &[&str] = &[
        "setbylayer",         // Modify ▸ Set to ByLayer
        "copytolayer",        // Layers ▸ Copy Objects to New Layer
        "laywalk",            // Layers ▸ Layer Walk
        "layvpi",             // Layers ▸ VP Freeze in All Viewports except Current
        "laylockfadectl",     // Layers ▸ Locked Layer fading (toggle)
        "bdetect",            // Block ▸ Detect and Convert
        "bconvert",           // Block ▸ Search and Convert
        "breplace",           // Block ▸ Replace
        "classicgroup",       // Groups ▸ Group Manager
        "groupdisplaymode",   // Groups ▸ Group Bounding Box (toggle)
        "measuregeom.quick",  // Utilities ▸ Measure ▸ Quick
        "measuregeom.volume", // Utilities ▸ Measure ▸ Volume
        "pastehlnk",          // Clipboard ▸ Paste ▸ Paste as Hyperlink
        "pastespec",          // Clipboard ▸ Paste ▸ Paste Special
        "viewbase",           // View ▸ Base
        "viewsectionstyle",   // View ▸ Section View Style
        "viewdetailstyle",    // View ▸ Detail View Style
        "viewstd",            // View ▸ Drafting Standard (dialog launcher)
        // ---- Insert ----
        "clip",                    // Reference ▸ Clip
        "adjust",                  // Reference ▸ Adjust
        "ulayers",                 // Reference ▸ Underlay Layers
        "xdwgfadectl",             // Reference ▸ Xref Fading (toggle)
        "pdfimport",               // Import ▸ PDF Import
        "dgnimport",               // Import ▸ PDF Import ▸ DGN Import
        "pdfshxtext",              // Import ▸ Recognize SHX Text
        "pdfshxtext.settings",     // Import ▸ Recognition Settings
        "txt2mtxt",                // Import ▸ Combine Text
        "field",                   // Data ▸ Field
        "updatefield",             // Data ▸ Update Fields
        "insertobj",               // Data ▸ OLE Object
        "hyperlink",               // Data ▸ Hyperlink
        "datalinkupdate.write",    // Linking & Extraction ▸ Upload to Source
        "geographiclocation.esri", // Location ▸ Set Location ▸ From Esri Maps (online map service)
        "geographiclocation.file", // Location ▸ Set Location ▸ From File
        "geographiclocation.bing", // Location ▸ Set Location ▸ From Bing Maps (online map service)
        // ---- Annotate ----
        "spell",       // Text ▸ Check Spelling
        "textalign",   // Text ▸ Text Align
        "justifytext", // Text ▸ Justify
        "scaletext",   // Text ▸ Scale (slide-out)
        // ---- Parametric ----
        "dcconvert",                    // Dimensional ▸ Convert
        "cconstraintform.dynamic",      // Dimensional ▸ Dynamic Constraint Mode (toggle, slide-out)
        "cconstraintform.annotational", // Dimensional ▸ Annotational Constraint Mode (toggle, slide-out)
        // ---- View ----
        "navsmotion",          // Viewport Tools ▸ Show Motion (toggle, slide-out)
        "newview",             // Named Views ▸ New View
        "vports.2v",           // Model Viewports ▸ Viewport Configuration ▸ Two: Vertical
        "vports.2h",           // Model Viewports ▸ Viewport Configuration ▸ Two: Horizontal
        "vports.3r",           // Model Viewports ▸ Viewport Configuration ▸ Three: Right
        "vports.3l",           // Model Viewports ▸ Viewport Configuration ▸ Three: Left
        "vports.3a",           // Model Viewports ▸ Viewport Configuration ▸ Three: Above
        "vports.3b",           // Model Viewports ▸ Viewport Configuration ▸ Three: Below
        "vports.3v",           // Model Viewports ▸ Viewport Configuration ▸ Three: Vertical
        "vports.3h",           // Model Viewports ▸ Viewport Configuration ▸ Three: Horizontal
        "vports.4r",           // Model Viewports ▸ Viewport Configuration ▸ Four: Right
        "vports.4l",           // Model Viewports ▸ Viewport Configuration ▸ Four: Left
        "vports.join",         // Model Viewports ▸ Join
        "vports.toggle",       // Model Viewports ▸ Restore
        "activityinsights",    // History ▸ Activity Insights (toggle)
        "toolpalettes",        // Palettes ▸ Tool Palettes (toggle)
        "commandmacros",       // Palettes ▸ Command Macros (toggle)
        "sheetset",            // Palettes ▸ Sheet Set Manager (toggle)
        "markup",              // Palettes ▸ Markup Set Manager (toggle)
        "adcenter",            // Palettes ▸ Design Center (toggle)
        "matbrowseropen",      // Palettes ▸ Materials Browser (toggle, slide-out)
        "mateditoropen",       // Palettes ▸ Materials Editor (toggle, slide-out)
        "visualstyles",        // Palettes ▸ Visual Styles (toggle, slide-out)
        "rpref",               // Palettes ▸ Render Settings (toggle, slide-out)
        "lightlist",           // Palettes ▸ Lights in Model (toggle, slide-out)
        "sunproperties",       // Palettes ▸ Sun Properties (toggle, slide-out)
        "switchwindows",       // Interface ▸ Switch Windows (the list of open drawings)
        "syswindows.tilehorz", // Interface ▸ Tile Horizontally
        "syswindows.tilevert", // Interface ▸ Tile Vertically
        "syswindows.cascade",  // Interface ▸ Cascade
        // ---- Manage ----
        "actrecord",           // Action Recorder ▸ Record
        "actusermessage",      // Action Recorder ▸ Insert Message
        "actbasepoint",        // Action Recorder ▸ Insert Base Point
        "actuserinput",        // Action Recorder ▸ Pause for User Input
        "actplay",             // Action Recorder ▸ Play
        "actpreferences",      // Action Recorder ▸ Preference
        "actmanager",          // Action Recorder ▸ Manage Action Macros
        "cui",                 // Customization ▸ User Interface
        "customize",           // Customization ▸ Tool Palettes
        "cuiimport",           // Customization ▸ Import
        "cuiexport",           // Customization ▸ Export
        "editaliases",         // Customization ▸ Edit Aliases
        "vbaide",              // Applications ▸ Visual Basic Editor
        "vlisp",               // Applications ▸ Visual LISP Editor
        "vbarun",              // Applications ▸ Run VBA Macro
        "vbaload",             // Applications ▸ Load Project (slide-out)
        "vbaman",              // Applications ▸ VBA Manager (slide-out)
        "laytrans",            // CAD Standards ▸ Layer Translator
        "checkstandards",      // CAD Standards ▸ Check
        "standards",           // CAD Standards ▸ Configure
        "purge.nonpurgeable",  // Cleanup ▸ Find Non-Purgeable Items
        "performanceanalyzer", // Performance ▸ Performance Analyzer
        // ---- Output ----
        "viewplotdetails",       // Plot ▸ View Details
        "exportdwfx",            // Export to DWF/PDF ▸ Export ▸ DWFx
        "exportdwf",             // Export to DWF/PDF ▸ Export ▸ DWF
        "eplotexport.window",    // Export to DWF/PDF ▸ Export Window
        "eplotexport.pagesetup", // Export to DWF/PDF ▸ Page Setup (override)
        "eplotexport.preview",   // Export to DWF/PDF ▸ Preview
        "exportsettings.dwf",    // Export to DWF/PDF ▸ Export to DWF Options
        "exportsettings.pdf",    // Export to DWF/PDF ▸ Export to PDF Options
        // ---- Collaborate (Autodesk online services) ----
        "sharedviews",      // Share ▸ Shared Views (toggle)
        "pushtodocsopen",   // Autodesk Docs ▸ Push to Autodesk Docs (toggle)
        "tracepaletteopen", // Traces ▸ Traces Palette (toggle)
        "markupimport",     // Traces ▸ Markup Import
        // ---- Express Tools ----
        "burst",        // Blocks ▸ Explode Attributes
        "blockreplace", // Blocks ▸ Replace Block
        "xlist",        // Blocks ▸ List Properties
        "attin",        // Blocks ▸ Import Attributes
        "attout",       // Blocks ▸ Export Attributes
        "blocktoxref",  // Blocks ▸ Convert block to xref (slide-out)
        "bextend",      // Blocks ▸ Extend to Nested Objects (slide-out)
        "btrim",        // Blocks ▸ Trim to Nested Objects (slide-out)
        "arctext",      // Text ▸ Arc Aligned
        "txtexp",       // Text ▸ Modify Text ▸ Explode
        "tcase",        // Text ▸ Modify Text ▸ Change Case
        "torient",      // Text ▸ Modify Text ▸ Rotate
        "textfit",      // Text ▸ Modify Text ▸ Fit
        "tcount",       // Text ▸ Auto Number
        "tcircle",      // Text ▸ Enclose in Object
        "rtext",        // Text ▸ Remote Text (slide-out)
        "textmask",     // Text ▸ Text Mask (slide-out)
        "textunmask",   // Text ▸ Unmask Text (slide-out)
        "mocoro",       // Modify ▸ Move/Copy/Rotate
        "mstretch",     // Modify ▸ Stretch Multiple
        "clipit",       // Modify ▸ Extended Clip (slide-out)
        "exoffset",     // Modify ▸ Extended Offset (slide-out)
        "shp2blk",      // Modify ▸ Convert Shape to Block (slide-out)
        "alignspace",   // Layout ▸ Align Space
        "vpsync",       // Layout ▸ Synchronize Viewports
        "layoutmerge",  // Layout ▸ Merge Layout
        "vpscale",      // Layout ▸ List Viewport Scale (slide-out)
        "breakline",    // Draw ▸ Break-line Symbol
        "superhatch",   // Draw ▸ Super Hatch
        "qlattach",     // Dimension ▸ Annotation Attachment ▸ Attach Leader to Annotation
        "qlattachset",  // Dimension ▸ Annotation Attachment ▸ Global Attach Leader to Annotation
        "qldetachset",  // Dimension ▸ Annotation Attachment ▸ Detach Leaders from Annotation
        "dimreassoc",   // Dimension ▸ Reset Text
        "dimim",        // Dimension ▸ Import Style
        "dimex",        // Dimension ▸ Export Style
        "xdata",        // Tools ▸ Attach Xdata
        "xdlist",       // Tools ▸ List Xdata
        "sysvdlg",      // Tools ▸ System Variables
        "explan",       // Tools ▸ Extended Plan (slide-out)
        "mkltype",      // Tools ▸ Make Linetype (slide-out)
        "mkshape",      // Tools ▸ Make Shape (slide-out)
        "showurls",     // Web ▸ URL Options ▸ Show URLs
        "churls",       // Web ▸ URL Options ▸ Change URLs
        "repurls",      // Web ▸ URL Options ▸ Find and Replace URLs
    ];

    #[test]
    fn every_ribbon_command_exists_or_is_listed_unbuilt() {
        let missing: Vec<_> = command_ids().into_iter().filter(|id| !crate::palettes::command_known(id) && !UNBUILT.contains(id)).collect();
        assert!(missing.is_empty(), "add these to UNBUILT or build them: {missing:?}");
    }

    #[test]
    fn unbuilt_list_has_no_stale_entries() {
        let stale: Vec<_> = UNBUILT.iter().filter(|id| crate::palettes::command_known(id)).collect();
        assert!(stale.is_empty(), "these exist now, remove them from UNBUILT: {stale:?}");
        let referenced = command_ids();
        let unused: Vec<_> = UNBUILT.iter().filter(|id| !referenced.contains(id)).collect();
        assert!(unused.is_empty(), "no ribbon item uses these, remove them from UNBUILT: {unused:?}");
    }

    /// docs/ui-parity.md's "Ribbon items without a command" names exactly the `UNBUILT` ids, so
    /// the doc and the list can't drift apart.
    #[test]
    fn ui_parity_doc_lists_exactly_the_unbuilt_ribbon_commands() {
        const DOC: &str = include_str!("../../../../docs/ui-parity.md");
        let section = DOC.split("### Ribbon items without a command").nth(1).expect("docs/ui-parity.md has the section");
        let section = section.split("\n#").next().unwrap_or(section);
        let mut listed: Vec<&str> = section.lines().filter(|l| l.starts_with("| `")).filter_map(|l| l.split('`').nth(1)).collect();
        let mut unbuilt = UNBUILT.to_vec();
        listed.sort_unstable();
        unbuilt.sort_unstable();
        assert_eq!(listed, unbuilt, "update docs/ui-parity.md's \"Ribbon items without a command\" table to match UNBUILT");
    }

    #[test]
    fn the_tab_row_has_autocads_nine_tabs_in_order() {
        let labels: Vec<_> = tabs().iter().map(|t| t.label).collect();
        assert_eq!(labels, ["Home", "Insert", "Annotate", "Parametric", "View", "Manage", "Output", "Collaborate", "Express Tools"]);
        assert!(tabs().iter().all(|t| !t.panels.is_empty()), "every tab is built");
        assert!(is_tab("Express Tools") && !is_tab("Tool Sets") && !is_tab(""));
    }

    #[test]
    fn home_has_autocads_panels_in_order() {
        let labels: Vec<_> = tabs()[0].panels.iter().map(|p| p.label).collect();
        assert_eq!(labels, ["Draw", "Modify", "Annotation", "Layers", "Block", "Properties", "Groups", "Utilities", "Clipboard", "View"]);
    }

    #[test]
    fn small_items_use_rows_1_to_3_and_splits_have_a_valid_default() {
        for p in tabs().iter().flat_map(|t| t.panels) {
            for item in p.items {
                match item {
                    Item::Button { size: Size::Small { .. }, row, .. }
                    | Item::Toggle { size: Size::Small { .. }, row, .. }
                    | Item::Split { size: Size::Small { .. }, row, .. }
                    | Item::Menu { size: Size::Small { .. }, row, .. }
                    | Item::Combo { row, .. }
                    | Item::Slider { row, .. } => {
                        assert!((1..=3).contains(row), "{}: row {row}", p.label)
                    }
                    Item::Button { size: Size::Large, row, .. }
                    | Item::Toggle { size: Size::Large, row, .. }
                    | Item::Split { size: Size::Large, row, .. }
                    | Item::Menu { size: Size::Large, row, .. } => {
                        assert_eq!(*row, 0, "{}: large items use row 0", p.label)
                    }
                    Item::Separator => {}
                }
            }
            for item in p.slideout {
                // Large items fill a slide-out's height too (View ▸ Palettes ▸ Text Window).
                match crate::ribbon::layout::small_row(item) {
                    Some(row) => assert!((1..=4).contains(&row), "{} slide-out: row {row}", p.label),
                    None => assert!(
                        matches!(
                            item,
                            Item::Button { row: 0, .. } | Item::Toggle { row: 0, .. } | Item::Split { row: 0, .. } | Item::Menu { row: 0, .. }
                        ),
                        "{} slide-out: large items use row 0",
                        p.label
                    ),
                }
            }
            for item in p.items.iter().chain(p.slideout) {
                if let Item::Split { menu, .. } | Item::Menu { menu, .. } = item {
                    assert!(menu.iter().any(|e| matches!(e, Entry::Command { .. })), "{}: an empty menu", p.label);
                }
                if let Item::Split { menu, default, .. } = item {
                    assert!(matches!(menu.get(*default), Some(Entry::Command { .. })), "{}: bad split default", p.label);
                    assert!(!matches!(menu.first(), Some(Entry::Separator)), "{}: menu starts with a separator", p.label);
                    assert!(!matches!(menu.last(), Some(Entry::Separator)), "{}: menu ends with a separator", p.label);
                }
            }
        }
    }

    #[test]
    fn home_split_menus_match_autocads_entries() {
        let menu = |label: &str| -> Vec<&'static str> {
            tabs()[0]
                .panels
                .iter()
                .flat_map(|p| p.items.iter().chain(p.slideout))
                .find_map(|i| match i {
                    Item::Split { label: l, menu, default, .. } => {
                        let face = l.or_else(|| match menu.get(*default) {
                            Some(Entry::Command { label, .. }) => Some(*label),
                            _ => None,
                        });
                        (face == Some(label))
                            .then(|| menu.iter().map(|e| if let Entry::Command { label, .. } = e { *label } else { "---" }).collect())
                    }
                    _ => None,
                })
                .unwrap_or_default()
        };
        assert_eq!(menu("Circle"), ["Center, Radius", "Center, Diameter", "---", "2-Point", "3-Point", "---", "Tan, Tan, Radius", "Tan, Tan, Tan"]);
        assert_eq!(menu("Array"), ["Rectangular Array", "Path Array", "Polar Array"]);
        assert_eq!(menu("Center"), ["Center", "Axis, End", "---", "Elliptical Arc"]);
        assert_eq!(menu("Arc").len(), 15);
    }
}
