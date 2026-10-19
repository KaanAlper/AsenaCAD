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

/// A thin line between two groups of a panel's columns.
const BAR: Want = want("|", "separator", "", 0);

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
        Item::Separator => ("|".into(), "separator", String::new(), 0, String::new(), vec![], 0, 0.0),
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

mod home_insert_annotate;
mod labels;
mod output_express;
mod parametric_view_manage;
