//! Every ribbon tab against AutoCAD 2026's (Windows) layout: panels in order, and per panel the
//! items in order with their control type, size, row and command, split and drop-down menu
//! entries with their commands, combo and slider kinds and widths, slide-out items and the dialog
//! launcher with its name. The expected data is transcribed from our own notes of AutoCAD's
//! ribbon.

use std::sync::{Arc, Mutex};

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::layout;
use cadcraft_ui_egui::ribbon::model::{Entry, Item, Panel, Size, Tab, tabs};
use cadcraft_ui_egui::{CadApp, Services};
use egui::vec2;
use egui_kittest::Harness;

/// What one item must be.
pub struct Want {
    /// The item's accessible name: a split's face label (its fixed label, else its default
    /// entry's), a combo's or slider's control name.
    pub label: &'static str,
    /// "button", "toggle", "split", "menu" (a drop-down button), "combo", "slider".
    pub kind: &'static str,
    /// Buttons, toggles, splits and menus: "large", "small" (icon only), "small+label".
    /// Combos and sliders: their `Combo` / `Slider` kind.
    pub size: &'static str,
    pub row: u8,
    /// Buttons and toggles: the command they run. Empty otherwise.
    pub cmd: &'static str,
    /// Splits and menus: (label, command) of each entry in order, ("---", "") for a separator.
    pub menu: &'static [(&'static str, &'static str)],
    /// Splits: the entry the face starts with.
    pub default: usize,
    /// Combos and sliders: their width.
    pub width: f32,
}

pub struct WantPanel<'a> {
    pub label: &'static str,
    pub items: &'a [Want],
    pub slideout: &'a [Want],
    /// The dialog launcher, if the panel has one: (command, accessible name).
    pub launcher: Option<(&'static str, &'static str)>,
}

const fn want(label: &'static str, kind: &'static str, size: &'static str, row: u8) -> Want {
    Want { label, kind, size, row, cmd: "", menu: &[], default: 0, width: 0.0 }
}

const fn b(label: &'static str, size: &'static str, row: u8, cmd: &'static str) -> Want {
    Want { cmd, ..want(label, "button", size, row) }
}

const fn tg(label: &'static str, size: &'static str, row: u8, cmd: &'static str) -> Want {
    Want { cmd, ..want(label, "toggle", size, row) }
}

const fn sp(label: &'static str, size: &'static str, row: u8, menu: &'static [(&'static str, &'static str)], default: usize) -> Want {
    Want { menu, default, ..want(label, "split", size, row) }
}

const fn mn(label: &'static str, size: &'static str, row: u8, menu: &'static [(&'static str, &'static str)]) -> Want {
    Want { menu, ..want(label, "menu", size, row) }
}

const fn cb(label: &'static str, kind: &'static str, row: u8, width: f32) -> Want {
    Want { width, ..want(label, "combo", kind, row) }
}

const fn sl(label: &'static str, kind: &'static str, row: u8, width: f32) -> Want {
    Want { width, ..want(label, "slider", kind, row) }
}

const SEP: (&str, &str) = ("---", "");

fn tab(label: &str) -> &'static Tab {
    tabs().iter().find(|t| t.label == label).unwrap_or_else(|| panic!("no tab {label}"))
}

fn size_name(size: Size) -> String {
    match size {
        Size::Large => "large",
        Size::Small { label: false } => "small",
        Size::Small { label: true } => "small+label",
    }
    .into()
}

fn entries(menu: &[Entry]) -> Vec<(String, String)> {
    menu.iter()
        .map(|e| match e {
            Entry::Command { label, cmd, .. } => (label.to_string(), cmd.to_string()),
            Entry::Separator => ("---".into(), String::new()),
        })
        .collect()
}

/// An item as the harness compares it: (label, kind, size, row, command, menu, default, width).
type Got = (String, &'static str, String, u8, String, Vec<(String, String)>, usize, f32);

fn describe(item: &Item) -> Got {
    match item {
        Item::Button { label, size, row, cmd, .. } => (label.to_string(), "button", size_name(*size), *row, cmd.to_string(), vec![], 0, 0.0),
        Item::Toggle { label, size, row, cmd, .. } => (label.to_string(), "toggle", size_name(*size), *row, cmd.to_string(), vec![], 0, 0.0),
        Item::Split { label, size, row, menu, default } => {
            let face = label.map(str::to_string).or_else(|| entries(menu).get(*default).map(|e| e.0.clone())).unwrap_or_default();
            (face, "split", size_name(*size), *row, String::new(), entries(menu), *default, 0.0)
        }
        Item::Menu { label, size, row, menu, .. } => (label.to_string(), "menu", size_name(*size), *row, String::new(), entries(menu), 0, 0.0),
        Item::Combo { kind, row, width } => (kind.label().to_string(), "combo", format!("{kind:?}"), *row, String::new(), vec![], 0, *width),
        Item::Slider { kind, row, width } => (kind.label().to_string(), "slider", format!("{kind:?}"), *row, String::new(), vec![], 0, *width),
    }
}

fn expected(w: &Want) -> Got {
    let menu = w.menu.iter().map(|(l, c)| (l.to_string(), c.to_string())).collect();
    (w.label.to_string(), w.kind, w.size.to_string(), w.row, w.cmd.to_string(), menu, w.default, w.width)
}

fn check_items(where_: &str, got: &[Item], want: &[Want]) {
    let got: Vec<_> = got.iter().map(describe).collect();
    let names: Vec<_> = got.iter().map(|g| g.0.clone()).collect();
    let want_names: Vec<_> = want.iter().map(|w| w.label).collect();
    assert_eq!(names, want_names, "{where_}: items in AutoCAD's order");
    for (g, w) in got.iter().zip(want) {
        assert_eq!(*g, expected(w), "{where_} ▸ {}: (label, kind, size, row, command, menu, default, width)", w.label);
    }
}

pub fn check_tab(label: &str, want: &[WantPanel]) {
    let t = tab(label);
    let got: Vec<_> = t.panels.iter().map(|p| p.label).collect();
    let want_names: Vec<_> = want.iter().map(|p| p.label).collect();
    assert_eq!(got, want_names, "{label}: panels in AutoCAD's order");
    for (p, w) in t.panels.iter().zip(want) {
        let at = format!("{label} ▸ {}", w.label);
        check_items(&at, p.items, w.items);
        check_items(&format!("{at} (slide-out)"), p.slideout, w.slideout);
        assert_eq!(launcher(p), w.launcher, "{at}: dialog launcher (command, name)");
    }
}

fn launcher(p: &Panel) -> Option<(&'static str, &'static str)> {
    p.launcher.zip(p.launcher_label())
}

/// Each panel's natural width (full labels, no divider) in the ribbon font, as the strip lays
/// it out.
fn panel_widths(label: &str) -> Vec<(&'static str, f32)> {
    let t = tab(label);
    let out = Arc::new(Mutex::new(Vec::new()));
    let sink = Arc::clone(&out);
    let app = CadApp::new(Session::default(), Services::default());
    let mut h = Harness::builder().with_size(vec2(1906.0, 300.0)).build_ui_state(
        move |ui, _app: &mut CadApp| {
            ui.style_mut().override_font_id = Some(cadcraft_ui_egui::theme::ribbon_font(ui.ctx()));
            let widths = t.panels.iter().map(|p| (p.label, layout::panel_width(ui, t.label, p, false))).collect();
            if let Ok(mut o) = sink.lock() {
                *o = widths;
            }
        },
        app,
    );
    h.run_steps(2);
    out.lock().map(|o| o.clone()).unwrap_or_default()
}

/// The tab's natural width is within ±4 % of AutoCAD's (`autocad`: its panels' widths, dividers
/// included, in order). Returns our widths (with dividers) next to AutoCAD's.
fn check_total_width(label: &str, autocad: &[(&str, f32)]) -> Vec<(&'static str, f32, f32)> {
    let ours = panel_widths(label);
    let names: Vec<_> = ours.iter().map(|(n, _)| *n).collect();
    let want: Vec<_> = autocad.iter().map(|(n, _)| *n).collect();
    assert_eq!(names, want, "{label}: panel widths listed in order");
    let pairs: Vec<_> = ours.iter().zip(autocad).map(|((n, o), (_, a))| (*n, o + layout::DIVIDER, *a)).collect();
    let total: f32 = pairs.iter().map(|p| p.1).sum();
    let expected: f32 = autocad.iter().map(|(_, w)| w).sum();
    assert!((total - expected).abs() <= 0.04 * expected, "{label}: natural width {total:.0} vs AutoCAD's {expected} (±4 %): {pairs:?}");
    pairs
}

/// [`check_total_width`], and no panel is more than 8 % off its own AutoCAD width.
fn check_widths(label: &str, autocad: &[(&str, f32)]) {
    let pairs = check_total_width(label, autocad);
    let off: Vec<_> = pairs.iter().filter(|(_, o, a)| (o - a).abs() > 0.08 * a).map(|(n, o, a)| format!("{n}: {o:.0} vs {a}")).collect();
    assert!(off.is_empty(), "{label}: panels more than 8 % off AutoCAD's width: {off:?}");
}

/// The block gallery's stand-in list (both Home and Insert).
const INSERT_GALLERY: &[(&str, &str)] = &[("More Options...", "insert")];

#[test]
fn the_harness_accepts_home_panels() {
    // Data M1a verified, through the same harness.
    let home = tab("Home");
    let panel = |l: &str| home.panels.iter().find(|p| p.label == l).unwrap_or_else(|| panic!("{l}"));
    let paste = &[
        ("Paste", "pasteclip"),
        ("Paste as Block", "pasteblock"),
        ("Paste as Hyperlink", "pastehlnk"),
        ("Paste to Original Coordinates", "pasteorig"),
        ("Paste Special", "pastespec"),
    ];
    check_items(
        "Home ▸ Clipboard",
        panel("Clipboard").items,
        &[sp("Paste", "large", 0, paste, 0), b("Cut", "small", 1, "cutclip"), b("Copy Clip", "small", 2, "copyclip")],
    );
    let props = panel("Properties");
    assert_eq!(launcher(props), Some(("properties", "Properties")));
    check_items(
        "Home ▸ Properties (slide-out)",
        props.slideout,
        &[
            cb("Plot Style", "PlotStyle", 1, 200.0),
            cb("Transparency Mode", "Transparency", 2, 35.0),
            sl("Transparency", "Transparency", 2, 158.0),
            b("List", "small+label", 3, "list"),
        ],
    );
    check_items(
        "Home ▸ Groups (slide-out)",
        panel("Groups").slideout,
        &[b("Group Manager", "small+label", 1, "classicgroup"), tg("Group Bounding Box", "small+label", 2, "groupdisplaymode")],
    );
    assert_eq!(launcher(panel("View")), Some(("viewstd", "Drafting Standard")));
    let block = panel("Block");
    let insert = block.items.first().map(describe);
    assert_eq!(insert, Some(expected(&mn("Insert", "large", 0, INSERT_GALLERY))));
}

#[test]
fn home_is_as_wide_as_autocads() {
    // The panel widths of AutoCAD's Home tab at 1906 px. M1a tuned the whole tab; its three
    // narrowest panels (Groups, Utilities, Clipboard) are 8–14 px off on their own.
    check_total_width(
        "Home",
        &[
            ("Draw", 224.0),
            ("Modify", 250.0),
            ("Annotation", 189.0),
            ("Layers", 273.0),
            ("Block", 240.0),
            ("Properties", 265.0),
            ("Groups", 69.0),
            ("Utilities", 97.0),
            ("Clipboard", 91.0),
            ("View", 64.0),
        ],
    );
}

// ---- Insert ----

#[test]
fn insert_matches_autocad() {
    check_tab(
        "Insert",
        &[
            WantPanel {
                label: "Block",
                items: &[
                    // AutoCAD's Insert is a gallery of the drawing's blocks (a drop-down button).
                    mn("Insert", "large", 0, INSERT_GALLERY),
                    sp("Detect and Convert", "large", 0, &[("Detect and Convert", "bdetect"), ("Search and Convert", "bconvert")], 0),
                    sp("Edit Attribute", "small+label", 1, &[("Single", "attedit"), ("Multiple", "-attedit")], 0),
                    cb("Retain Attribute Display", "AttributeDisplay", 2, 165.0),
                    b("Replace", "small+label", 3, "breplace"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Block Definition",
                items: &[
                    sp("Create Block", "large", 0, &[("Create Block", "block"), ("Write Block", "wblock")], 0),
                    b("Define Attributes", "large", 0, "attdef"),
                    b("Manage Attributes", "large", 0, "battman"),
                    b("Block Editor", "large", 0, "bedit"),
                ],
                slideout: &[b("Set Base Point", "small+label", 1, "base"), b("Synchronize", "small+label", 2, "attsync")],
                launcher: None,
            },
            WantPanel {
                label: "Reference",
                items: &[
                    b("Attach", "large", 0, "attach"),
                    b("Clip", "large", 0, "clip"),
                    b("Adjust", "large", 0, "adjust"),
                    b("Underlay Layers", "small+label", 1, "ulayers"),
                    cb("*Frames vary*", "XrefFrames", 2, 114.0),
                    cb("Snap to Underlays ON", "UnderlaySnap", 3, 157.0),
                ],
                slideout: &[
                    b("Edit Reference", "small+label", 1, "refedit"),
                    tg("Xref Fading Toggle Button", "small", 2, "xdwgfadectl"),
                    sl("Xref fading", "XrefFade", 2, 200.0),
                ],
                launcher: Some(("xref", "External References")),
            },
            WantPanel {
                label: "Import",
                items: &[
                    sp("PDF Import", "large", 0, &[("PDF Import", "pdfimport"), ("DGN Import", "dgnimport"), ("Import", "import")], 0),
                    b("Recognize SHX Text", "small+label", 1, "pdfshxtext"),
                    b("Recognition Settings", "small+label", 2, "pdfshxtext.settings"),
                    b("Combine Text", "large", 0, "txt2mtxt"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Data",
                items: &[
                    b("Field", "large", 0, "field"),
                    b("Update Fields", "small+label", 1, "updatefield"),
                    b("OLE Object", "small+label", 2, "insertobj"),
                    b("Hyperlink", "small+label", 3, "hyperlink"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Linking & Extraction",
                items: &[
                    b("Data Link", "large", 0, "datalink"),
                    b("Download from Source", "small+label", 1, "datalinkupdate"),
                    b("Upload to Source", "small+label", 2, "datalinkupdate.write"),
                    // AutoCAD's label has two spaces.
                    b("Extract  Data", "small+label", 3, "dataextraction"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Location",
                // A drop-down button (inventory §6, observed after the first pass).
                items: &[mn(
                    "Set Location",
                    "large",
                    0,
                    &[
                        ("From Esri Maps", "geographiclocation.esri"),
                        ("From File", "geographiclocation.file"),
                        SEP,
                        ("From Bing Maps", "geographiclocation.bing"),
                    ],
                )],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn insert_is_as_wide_as_autocads() {
    check_widths(
        "Insert",
        &[
            ("Block", 288.0),
            ("Block Definition", 215.0),
            ("Reference", 282.0),
            ("Import", 270.0),
            ("Data", 145.0),
            ("Linking & Extraction", 196.0),
            ("Location", 69.0),
        ],
    );
}

// ---- Annotate ----

const DIM_LINEAR: &[(&str, &str)] = &[
    ("Linear", "dimlinear"),
    ("Aligned", "dimaligned"),
    ("Angular", "dimangular"),
    SEP,
    ("Arc Length", "dimarc"),
    ("Radius", "dimradius"),
    ("Diameter", "dimdiameter"),
    SEP,
    ("Jogged", "dimjogged"),
    ("Ordinate", "dimordinate"),
];

#[test]
fn annotate_matches_autocad() {
    check_tab(
        "Annotate",
        &[
            WantPanel {
                label: "Text",
                items: &[
                    sp("Multiline Text", "large", 0, &[("Multiline Text", "mtext"), ("Single Line", "text")], 0),
                    b("Check Spelling", "small", 1, "spell"),
                    b("Text Align", "small", 2, "textalign"),
                    b("Justify", "small", 3, "justifytext"),
                    cb("Text Style", "TextStyle", 1, 200.0),
                    cb("Find Text", "FindText", 2, 200.0),
                    cb("Annotation Text Height", "TextHeight", 3, 200.0),
                ],
                slideout: &[b("Scale", "small+label", 1, "scaletext"), cb("Text Layer Gallery", "TextLayer", 2, 200.0)],
                launcher: Some(("style", "Text Style...")),
            },
            WantPanel {
                label: "Dimensions",
                items: &[
                    b("Dimension", "large", 0, "dim"),
                    cb("Dimension Style", "DimStyle", 1, 200.0),
                    b("Break", "small", 1, "dimbreak"),
                    b("Adjust Space", "small", 1, "dimspace"),
                    b("Jog Line", "small", 1, "dimjogline"),
                    cb("Dim Layer Gallery", "DimLayer", 2, 200.0),
                    b("Inspect", "small", 2, "diminspect"),
                    b("Update", "small", 2, "dimstyle.update"),
                    b("Reassociate", "small", 2, "dimreassociate"),
                    sp("Linear", "small+label", 3, DIM_LINEAR, 0),
                    b("Quick", "small+label", 3, "qdim"),
                    sp("Continue", "small+label", 3, &[("Continue", "dimcontinue"), ("Baseline", "dimbaseline")], 0),
                ],
                slideout: &[
                    b("Tolerance", "small", 1, "tolerance"),
                    b("Oblique", "small", 1, "dimedit.oblique"),
                    b("Text Angle", "small", 1, "dimtedit.angle"),
                    b("Left Justify", "small", 1, "dimtedit.left"),
                    b("Center Justify", "small", 1, "dimtedit.center"),
                    b("Right Justify", "small", 1, "dimtedit.right"),
                    b("Override", "small", 1, "dimoverride"),
                ],
                launcher: Some(("dimstyle", "Dimension, Dimension Style...")),
            },
            WantPanel {
                label: "Centerlines",
                items: &[b("Center Mark", "large", 0, "centermark"), b("Centerline", "large", 0, "centerline")],
                slideout: &[cb("Center Layer Gallery", "CenterLayer", 1, 200.0)],
                launcher: None,
            },
            WantPanel {
                label: "Leaders",
                items: &[
                    b("Multileader", "large", 0, "mleader"),
                    cb("Multileader Style", "MLeaderStyle", 1, 200.0),
                    cb("Mleader Layer Gallery", "MLeaderLayer", 2, 200.0),
                    b("Align", "small", 3, "mleaderalign"),
                    b("Collect", "small", 3, "mleadercollect"),
                    b("Add Leader", "small", 3, "mleaderedit.add"),
                    b("Remove Leader", "small", 3, "mleaderedit.remove"),
                ],
                slideout: &[],
                launcher: Some(("mleaderstyle", "Multileader Style Manager")),
            },
            WantPanel {
                label: "Tables",
                items: &[
                    b("Table", "large", 0, "table"),
                    cb("Table Style", "TableStyle", 1, 200.0),
                    b("Download from Source", "small", 2, "datalinkupdate"),
                    b("Extract Data", "small+label", 2, "dataextraction"),
                    b("Upload to Source", "small", 3, "datalinkupdate.write"),
                    b("Link Data", "small+label", 3, "datalink"),
                ],
                slideout: &[],
                launcher: Some(("tablestyle", "Table Style...")),
            },
            WantPanel {
                label: "Markup",
                items: &[
                    b("Wipeout", "large", 0, "wipeout"),
                    sp(
                        "Revision Cloud",
                        "large",
                        0,
                        &[("Rectangular", "revcloud"), ("Polygonal", "revcloud.polygonal"), ("Freehand", "revcloud.freehand")],
                        0,
                    ),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Annotation Scaling",
                items: &[
                    sp(
                        "Add Current Scale",
                        "large",
                        0,
                        &[("Add Current Scale", "aiobjectscaleadd"), ("Delete Current Scale", "aiobjectscaleremove")],
                        0,
                    ),
                    b("Scale List", "small+label", 1, "scalelistedit"),
                    b("Add/Delete Scales", "small+label", 2, "objectscale"),
                    b("Sync Scale Positions", "small+label", 3, "annoreset"),
                ],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn annotate_is_as_wide_as_autocads() {
    check_widths(
        "Annotate",
        &[
            ("Text", 300.0),
            ("Dimensions", 341.0),
            ("Centerlines", 106.0),
            ("Leaders", 272.0),
            ("Tables", 244.0),
            ("Markup", 122.0),
            ("Annotation Scaling", 234.0),
        ],
    );
}

// ---- Parametric ----

#[test]
fn parametric_matches_autocad() {
    check_tab(
        "Parametric",
        &[
            WantPanel {
                label: "Geometric",
                items: &[
                    b("Auto Constrain", "large", 0, "autoconstrain"),
                    b("Coincident", "small", 1, "gccoincident"),
                    b("Collinear", "small", 1, "gccollinear"),
                    b("Concentric", "small", 1, "gcconcentric"),
                    b("Fix", "small", 1, "gcfix"),
                    b("Parallel", "small", 2, "gcparallel"),
                    b("Perpendicular", "small", 2, "gcperpendicular"),
                    b("Horizontal", "small", 2, "gchorizontal"),
                    b("Vertical", "small", 2, "gcvertical"),
                    b("Tangent", "small", 3, "gctangent"),
                    b("Smooth", "small", 3, "gcsmooth"),
                    b("Symmetric", "small", 3, "gcsymmetric"),
                    b("Equal", "small", 3, "gcequal"),
                    b("Show/Hide", "small+label", 1, "constraintbar"),
                    b("Show All", "small+label", 2, "constraintbar.showall"),
                    b("Hide All", "small+label", 3, "constraintbar.hideall"),
                ],
                slideout: &[],
                launcher: Some(("constraintsettings", "Constraint Settings, Geometric")),
            },
            WantPanel {
                label: "Dimensional",
                items: &[
                    sp("Linear", "large", 0, &[("Linear", "dclinear"), ("Horizontal", "dchorizontal"), ("Vertical", "dcvertical")], 0),
                    b("Aligned", "large", 0, "dcaligned"),
                    b("Radius", "small", 1, "dcradius"),
                    b("Diameter", "small", 1, "dcdiameter"),
                    b("Angular", "small", 2, "dcangular"),
                    b("Convert", "small", 2, "dcconvert"),
                    b("Show/Hide", "small+label", 1, "dcdisplay"),
                    b("Show All", "small+label", 2, "dcdisplay.showall"),
                    b("Hide All", "small+label", 3, "dcdisplay.hideall"),
                ],
                slideout: &[
                    tg("Dynamic Constraint Mode", "small+label", 1, "cconstraintform.dynamic"),
                    tg("Annotational Constraint Mode", "small+label", 2, "cconstraintform.annotational"),
                ],
                launcher: Some(("constraintsettings", "Constraint Settings, Dimensional")),
            },
            WantPanel {
                label: "Manage",
                items: &[b("Delete Constraints", "large", 0, "delconstraint"), tg("Parameters Manager", "large", 0, "parameters")],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn parametric_is_as_wide_as_autocads() {
    check_widths("Parametric", &[("Geometric", 260.0), ("Dimensional", 256.0), ("Manage", 136.0)]);
}

// ---- Large labels ----

/// The texts the ribbon paints with `tab` open at 1906 px, as laid out ("\n" = line break).
fn painted_labels(tab: &str) -> Vec<String> {
    fn collect(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(t) => out.push(t.galley.text().to_string()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| collect(s, out)),
            _ => {}
        }
    }
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.ribbon_tab = tab.into();
    let mut h = Harness::builder().with_size(vec2(1906.0, 300.0)).build_ui_state(|ui, app: &mut CadApp| cadcraft_ui_egui::ribbon::show(app, ui), app);
    h.run_steps(3);
    let mut out = Vec::new();
    h.output().shapes.iter().for_each(|s| collect(&s.shape, &mut out));
    out
}

#[test]
fn large_labels_wrap_as_in_autocads_screenshots() {
    // Every large label of each tab as AutoCAD 2026 shows it at 1906 px: one word on one line;
    // more words on two, broken at the last space that keeps the first line short ("Detect and /
    // Convert", but "Add / Current Scale").
    for (tab, labels) in [
        (
            "Home",
            &[
                "Line",
                "Polyline",
                "Circle",
                "Arc",
                "Text",
                "Dimension",
                "Layer\nProperties",
                "Insert",
                "Detect and\nConvert",
                "Match\nProperties",
                "Group",
                "Measure",
                "Paste",
                "Base",
            ][..],
        ),
        (
            "Insert",
            &[
                "Insert",
                "Detect and\nConvert",
                "Create\nBlock",
                "Define\nAttributes",
                "Manage\nAttributes",
                "Block\nEditor",
                "Attach",
                "Clip",
                "Adjust",
                "PDF\nImport",
                "Combine\nText",
                "Field",
                "Data\nLink",
                "Set\nLocation",
            ][..],
        ),
        (
            "Annotate",
            &[
                "Multiline\nText",
                "Dimension",
                "Center\nMark",
                "Centerline",
                "Multileader",
                "Table",
                "Wipeout",
                "Revision\nCloud",
                "Add\nCurrent Scale",
            ][..],
        ),
        ("Parametric", &["Auto\nConstrain", "Linear", "Aligned", "Delete\nConstraints", "Parameters\nManager"][..]),
    ] {
        let painted = painted_labels(tab);
        let missing: Vec<_> = labels.iter().filter(|l| !painted.iter().any(|p| p == *l)).collect();
        assert!(missing.is_empty(), "{tab}: not painted as AutoCAD wraps them: {missing:?}");
    }
}
