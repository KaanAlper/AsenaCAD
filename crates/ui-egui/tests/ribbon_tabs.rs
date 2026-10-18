//! Every ribbon tab against AutoCAD 2026's (Windows) layout: panels in order, and per panel the
//! items in order with their control type, size and row, split-menu entries, slide-out items and
//! the dialog launcher. The expected data is transcribed from our own notes of AutoCAD's ribbon.

use std::sync::{Arc, Mutex};

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::layout;
use cadcraft_ui_egui::ribbon::model::{Entry, Item, Panel, Size, Tab, tabs};
use cadcraft_ui_egui::{CadApp, Services};
use egui::vec2;
use egui_kittest::Harness;

/// What one item must be. `kind`: "button", "split", "combo", "slider", "toggle".
pub struct Want {
    /// The item's accessible name: a split's face label (its fixed label, else its default
    /// entry's), a combo's or slider's control name.
    pub label: &'static str,
    pub kind: &'static str,
    /// "large", "small" (icon only), "small+label".
    pub size: &'static str,
    pub row: u8,
    /// Split menus: the entries' labels in order, "---" for a separator. Empty otherwise.
    pub menu: &'static [&'static str],
}

pub struct WantPanel<'a> {
    pub label: &'static str,
    pub items: &'a [Want],
    pub slideout: &'a [Want],
    /// The command the dialog launcher runs, if the panel has one.
    pub launcher: Option<&'static str>,
}

const fn b(label: &'static str, size: &'static str, row: u8) -> Want {
    Want { label, kind: "button", size, row, menu: &[] }
}

const fn tg(label: &'static str, size: &'static str, row: u8) -> Want {
    Want { label, kind: "toggle", size, row, menu: &[] }
}

const fn sp(label: &'static str, size: &'static str, row: u8, menu: &'static [&'static str]) -> Want {
    Want { label, kind: "split", size, row, menu }
}

const fn cb(label: &'static str, row: u8) -> Want {
    Want { label, kind: "combo", size: "small", row, menu: &[] }
}

const fn sl(label: &'static str, row: u8) -> Want {
    Want { label, kind: "slider", size: "small", row, menu: &[] }
}

fn tab(label: &str) -> &'static Tab {
    tabs().iter().find(|t| t.label == label).unwrap_or_else(|| panic!("no tab {label}"))
}

fn size_name(size: Size) -> &'static str {
    match size {
        Size::Large => "large",
        Size::Small { label: false } => "small",
        Size::Small { label: true } => "small+label",
    }
}

fn entry_label(e: &Entry) -> String {
    match e {
        Entry::Command { label, .. } => label.to_string(),
        Entry::Separator => "---".into(),
    }
}

fn describe(item: &Item) -> (String, &'static str, &'static str, u8, Vec<String>) {
    match item {
        Item::Button { label, size, row, .. } => (label.to_string(), "button", size_name(*size), *row, vec![]),
        Item::Split { label, size, row, menu, default } => {
            let face = label.map(str::to_string).or_else(|| menu.get(*default).map(entry_label)).unwrap_or_default();
            (face, "split", size_name(*size), *row, menu.iter().map(entry_label).collect())
        }
        Item::Combo { kind, row, .. } => (kind.label().to_string(), "combo", "small", *row, vec![]),
        Item::Slider { kind, row, .. } => (kind.label().to_string(), "slider", "small", *row, vec![]),
        Item::Toggle { label, size, row, .. } => (label.to_string(), "toggle", size_name(*size), *row, vec![]),
    }
}

fn check_items(where_: &str, got: &[Item], want: &[Want]) {
    let got: Vec<_> = got.iter().map(describe).collect();
    let names: Vec<_> = got.iter().map(|g| g.0.clone()).collect();
    let want_names: Vec<_> = want.iter().map(|w| w.label).collect();
    assert_eq!(names, want_names, "{where_}: items in AutoCAD's order");
    for (g, w) in got.iter().zip(want) {
        assert_eq!((g.1, g.2, g.3), (w.kind, w.size, w.row), "{where_} ▸ {}: kind/size/row", w.label);
        let menu: Vec<_> = w.menu.iter().map(|s| s.to_string()).collect();
        assert_eq!(g.4, menu, "{where_} ▸ {}: menu entries", w.label);
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
        assert_eq!(launcher(p), w.launcher, "{at}: dialog launcher");
    }
}

fn launcher(p: &Panel) -> Option<&'static str> {
    p.launcher
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

#[test]
fn the_harness_accepts_the_home_clipboard_panel() {
    // Clipboard is the smallest Home panel; this proves the harness against data M1a verified.
    let home = tab("Home");
    let clip = home.panels.iter().find(|p| p.label == "Clipboard").expect("Clipboard");
    check_items(
        "Home ▸ Clipboard",
        clip.items,
        &[
            sp("Paste", "large", 0, &["Paste", "Paste as Block", "Paste as Hyperlink", "Paste to Original Coordinates", "Paste Special"]),
            b("Cut", "small", 1),
            b("Copy Clip", "small", 2),
        ],
    );
    let props = home.panels.iter().find(|p| p.label == "Properties").expect("Properties");
    assert_eq!(launcher(props), Some("properties"));
    check_items(
        "Home ▸ Properties (slide-out)",
        props.slideout,
        &[cb("Plot Style", 1), cb("Transparency Mode", 2), sl("Transparency", 2), b("List", "small+label", 3)],
    );
    let groups = home.panels.iter().find(|p| p.label == "Groups").expect("Groups");
    check_items("Home ▸ Groups (slide-out)", groups.slideout, &[b("Group Manager", "small+label", 1), tg("Group Bounding Box", "small+label", 2)]);
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
                    // AutoCAD's Insert is a gallery; here it is a button running INSERT.
                    b("Insert", "large", 0),
                    sp("Detect and Convert", "large", 0, &["Detect and Convert", "Search and Convert"]),
                    sp("Edit Attribute", "small+label", 1, &["Single", "Multiple"]),
                    cb("Attribute Display", 2),
                    b("Replace", "small+label", 3),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Block Definition",
                items: &[
                    sp("Create Block", "large", 0, &["Create Block", "Write Block"]),
                    b("Define Attributes", "large", 0),
                    b("Manage Attributes", "large", 0),
                    b("Block Editor", "large", 0),
                ],
                slideout: &[b("Set Base Point", "small+label", 1), b("Synchronize", "small+label", 2)],
                launcher: None,
            },
            WantPanel {
                label: "Reference",
                items: &[
                    b("Attach", "large", 0),
                    b("Clip", "large", 0),
                    b("Adjust", "large", 0),
                    b("Underlay Layers", "small+label", 1),
                    cb("Frames", 2),
                    cb("Snap to Underlays", 3),
                ],
                slideout: &[b("Edit Reference", "small+label", 1), tg("Xref Fading", "small", 2), sl("Xref fading", 2)],
                launcher: Some("xref"),
            },
            WantPanel {
                label: "Import",
                items: &[
                    sp("PDF Import", "large", 0, &["PDF Import", "DGN Import", "Import"]),
                    b("Recognize SHX Text", "small+label", 1),
                    b("Recognition Settings", "small+label", 2),
                    b("Combine Text", "large", 0),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Data",
                items: &[
                    b("Field", "large", 0),
                    b("Update Fields", "small+label", 1),
                    b("OLE Object", "small+label", 2),
                    b("Hyperlink", "small+label", 3),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Linking & Extraction",
                items: &[
                    b("Data Link", "large", 0),
                    b("Download from Source", "small+label", 1),
                    b("Upload to Source", "small+label", 2),
                    // AutoCAD's label has two spaces.
                    b("Extract  Data", "small+label", 3),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Location",
                // The screenshot shows a ▾ under Set Location: a split button.
                items: &[sp("Set Location", "large", 0, &["From Map", "From File"])],
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
