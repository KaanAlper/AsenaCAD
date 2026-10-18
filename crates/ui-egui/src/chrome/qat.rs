//! AutoCAD's Quick Access Toolbar: a row of 24×22 command buttons in the title row (or, with
//! "Show Below the Ribbon", in a strip under the ribbon), Undo and Redo as split buttons with
//! their history lists, and the Customize ▾ menu that shows and hides the items.
//!
//! Which items show is `UiState::qat_shown` (command ids, kept in AutoCAD's order) and where the
//! toolbar sits is `UiState::qat_below_ribbon`; both change only through the `ui.qat` command.
//! Autodesk's online items (Open from / Save to Web & Mobile, Share) are not built.

use egui::{Rect, Response, Sense, Stroke, Ui, UiBuilder, Vec2, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::ribbon::avail;
use crate::ribbon::model::Combo;
use crate::theme::Tokens;

/// A QAT button.
pub const BUTTON: Vec2 = Vec2::new(24.0, 22.0);
/// The ▾ part of the Undo and Redo split buttons.
const SPLIT_ARROW_W: f32 = 13.0;
/// The Customize ▾ button's width.
const CUSTOMIZE_W: f32 = 16.0;
/// The Layer drop-down's text width.
const LAYER_W: f32 = 140.0;
/// Menus' row width and height.
const MENU_W: f32 = 210.0;
const MENU_ROW_H: f32 = 22.0;
/// The Undo and Redo lists show at most this many steps.
const HISTORY_MAX: usize = 25;
/// Height of the strip the QAT shows in below the ribbon.
pub const STRIP_H: f32 = 26.0;

/// How a QAT item is shown.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Button,
    /// Split: the face undoes one step, ▾ lists the undo history.
    Undo,
    /// Split like Undo, disabled when there is nothing to redo.
    Redo,
    /// The Layers panel's current-layer drop-down.
    Layer,
}

/// One item of the Customize ▾ list: `cmd` is the command it runs and its id in `qat_shown`.
#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub cmd: &'static str,
    pub label: &'static str,
    pub icon: Icon,
    pub kind: Kind,
    /// Shown in a new workspace (AutoCAD's defaults).
    pub default: bool,
}

const fn item(cmd: &'static str, label: &'static str, icon: Icon, kind: Kind, default: bool) -> Item {
    Item { cmd, label, icon, kind, default }
}

/// The Customize ▾ list in AutoCAD's order (its two Web & Mobile items left out). Items whose
/// command AsenaCAD lacks are listed disabled.
pub const ITEMS: &[Item] = &[
    item("new", "New", Icon::New, Kind::Button, true),
    item("open", "Open", Icon::Open, Kind::Button, true),
    item("qsave", "Save", Icon::Save, Kind::Button, true),
    item("saveas", "Save As...", Icon::SaveAs, Kind::Button, true),
    item("plot", "Plot", Icon::Plot, Kind::Button, true),
    item("u", "Undo", Icon::Undo, Kind::Undo, true),
    item("redo", "Redo", Icon::Redo, Kind::Redo, true),
    item("publish", "Batch Plot", Icon::Publish, Kind::Button, false),
    item("layer", "Layer", Icon::LayerProps, Kind::Layer, false),
    item("matchprop", "Match Properties", Icon::MatchProp, Kind::Button, false),
    item("preview", "Plot Preview", Icon::Preview, Kind::Button, false),
    item("properties", "Properties", Icon::Properties, Kind::Button, false),
    item("render", "Render", Icon::Sun, Kind::Button, false),
    item("sheetset", "Sheet Set Manager", Icon::Publish, Kind::Button, false),
    item("wscurrent", "Workspace", Icon::Workspace, Kind::Button, false),
];

const CUSTOMIZE: &str = "Customize Quick Access Toolbar";
const MORE_COMMANDS: &str = "More Commands...";
const SHOW_MENU_BAR: &str = "Show Menu Bar";
const HIDE_MENU_BAR: &str = "Hide Menu Bar";
const SHOW_BELOW: &str = "Show Below the Ribbon";
const SHOW_ABOVE: &str = "Show Above the Ribbon";

/// Every label the QAT shows, for the translation coverage check.
pub fn labels() -> impl Iterator<Item = &'static str> {
    ITEMS.iter().map(|i| i.label).chain([CUSTOMIZE, MORE_COMMANDS, SHOW_MENU_BAR, HIDE_MENU_BAR, SHOW_BELOW, SHOW_ABOVE])
}

/// The items a new workspace shows.
pub fn default_shown() -> Vec<String> {
    ITEMS.iter().filter(|i| i.default).map(|i| i.cmd.to_string()).collect()
}

fn is_shown(app: &CadApp, cmd: &str) -> bool {
    app.ui.qat_shown.iter().any(|c| c == cmd)
}

/// Whether AsenaCAD has `cmd` at all.
fn exists(cmd: &str) -> bool {
    cadcraft_engine::find_command(cmd).is_some() || crate::menus::is_ui_command(cmd)
}

/// `ui.qat`: `{"item": id|label, "on"?: bool}` shows or hides an item (toggles without `on`),
/// `{"below": bool}` moves the toolbar under the ribbon or back; always reports the state.
pub fn command(app: &mut CadApp, params: &Value) -> Result<Value, String> {
    if let Some(below) = params.get("below") {
        app.ui.qat_below_ribbon = below.as_bool().ok_or("`below` must be true or false")?;
    }
    if let Some(name) = params.get("item") {
        let name = name.as_str().ok_or("`item` must be a string")?;
        let found = ITEMS.iter().find(|i| i.cmd.eq_ignore_ascii_case(name) || i.label.eq_ignore_ascii_case(name));
        let Some(it) = found else {
            return Err(format!("unknown Quick Access Toolbar item \"{}\"", name.chars().take(40).collect::<String>()));
        };
        let on = match params.get("on") {
            Some(on) => on.as_bool().ok_or("`on` must be true or false")?,
            None => !is_shown(app, it.cmd),
        };
        if on && !exists(it.cmd) {
            return Err(format!("{} is not available yet", it.label));
        }
        // Rebuilt in AutoCAD's order; ids no item has (an old or hand-edited workspace) drop out.
        app.ui.qat_shown = ITEMS.iter().filter(|i| if i.cmd == it.cmd { on } else { is_shown(app, i.cmd) }).map(|i| i.cmd.to_string()).collect();
    }
    let shown: Vec<&str> = ITEMS.iter().filter(|i| is_shown(app, i.cmd)).map(|i| i.cmd).collect();
    Ok(json!({ "items": shown, "below": app.ui.qat_below_ribbon }))
}

/// The QAT under the ribbon, when "Show Below the Ribbon" is on: a top panel of `ui`.
pub fn strip(app: &mut CadApp, ui: &mut Ui) {
    if !app.ui.qat_below_ribbon {
        return;
    }
    let t = Tokens::get();
    egui::Panel::top("asenacad_qat_strip").exact_size(STRIP_H).frame(egui::Frame::NONE.fill(t.chrome)).show(ui, |ui| {
        let r = ui.max_rect();
        bar(app, ui, r.left() + 4.0, r.center().y);
    });
}

/// The shown items and the Customize ▾ button from `left`, centred on `center_y`; returns the
/// toolbar's right edge.
pub fn bar(app: &mut CadApp, ui: &mut Ui, left: f32, center_y: f32) -> f32 {
    let top = center_y - BUTTON.y / 2.0;
    let mut x = left;
    let shown: Vec<&Item> = ITEMS.iter().filter(|i| is_shown(app, i.cmd)).collect();
    for it in shown {
        let at = pos2(x, top);
        x = match it.kind {
            Kind::Button => button(app, ui, it, at),
            Kind::Undo => history(app, ui, it, at, true),
            Kind::Redo => history(app, ui, it, at, false),
            Kind::Layer => layer(app, ui, at),
        };
    }
    customize(app, ui, pos2(x, top))
}

fn start(app: &mut CadApp, cmd: &str) {
    app.cmd.buffer.clear();
    app.start(cmd);
}

/// An interactive rect that is disabled unless `enabled`.
fn interact(ui: &mut Ui, rect: Rect, id: egui::Id, enabled: bool) -> Response {
    ui.add_enabled_ui(enabled, |ui| ui.interact(rect, id, Sense::click())).inner
}

fn paint_state(ui: &Ui, rect: Rect, resp: &Response) {
    let t = Tokens::get();
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 2.0, t.select_bg);
    } else if resp.hovered() && resp.enabled() {
        ui.painter().rect_filled(rect, 2.0, t.control_hover);
    }
}

fn icon_rect(rect: Rect) -> Rect {
    Rect::from_center_size(rect.center(), Vec2::splat(16.0))
}

/// The tooltip: the label with the command's shortcut, as AutoCAD shows "Save (Ctrl+S)".
fn tip(ui: &Ui, it: &Item) -> String {
    let label = crate::i18n::t(it.label);
    match cadcraft_engine::find_command(it.cmd).and_then(|c| c.shortcut) {
        Some(sc) => format!("{label} ({})", crate::menus::shortcut_label(sc, ui.ctx().os().is_mac())),
        None => label.to_string(),
    }
}

fn button(app: &mut CadApp, ui: &mut Ui, it: &Item, at: egui::Pos2) -> f32 {
    let rect = Rect::from_min_size(at, BUTTON);
    let why = avail::availability(ui.ctx(), &app.session, it.cmd);
    let resp = interact(ui, rect, ui.id().with(("qat", it.cmd)), why.is_ready());
    paint_state(ui, rect, &resp);
    icons::paint(ui.painter(), icon_rect(rect), it.icon, !resp.enabled());
    icons::describe_control(ui, &resp, it.label, None, false);
    let tip = tip(ui, it);
    let resp = resp.on_hover_text(tip).on_disabled_hover_text(why.disabled_text());
    if resp.clicked() {
        start(app, it.cmd);
    }
    rect.right()
}

/// Undo or Redo: the face runs one step; ▾ lists the steps and choosing one undoes (redoes) it
/// and every step before it in the list.
fn history(app: &mut CadApp, ui: &mut Ui, it: &Item, at: egui::Pos2, undo: bool) -> f32 {
    let face_rect = Rect::from_min_size(at, BUTTON);
    let arrow_rect = Rect::from_min_size(pos2(face_rect.right(), at.y), vec2(SPLIT_ARROW_W, BUTTON.y));
    let labels = history_labels(app, undo);
    let why = avail::availability(ui.ctx(), &app.session, it.cmd);
    // AutoCAD greys Redo out when there is nothing to redo; Undo stays enabled.
    let face_enabled = why.is_ready() && (undo || !labels.is_empty());
    let id = ui.id().with(("qat", it.cmd));
    let face = interact(ui, face_rect, id.with("face"), face_enabled);
    let arrow = interact(ui, arrow_rect, id.with("menu"), why.is_ready() && !labels.is_empty());
    if (face.hovered() && face.enabled()) || (arrow.hovered() && arrow.enabled()) {
        ui.painter().rect_filled(face_rect.union(arrow_rect), 2.0, Tokens::get().control_hover);
    }
    paint_state(ui, face_rect, &face);
    paint_state(ui, arrow_rect, &arrow);
    icons::paint(ui.painter(), icon_rect(face_rect), it.icon, !face.enabled());
    let chevron = Rect::from_center_size(arrow_rect.center(), Vec2::splat(9.0));
    icons::paint(ui.painter(), chevron, Icon::ChevronDown, !arrow.enabled());
    icons::describe_control(ui, &face, it.label, None, false);
    let disabled = if !why.is_ready() {
        why.disabled_text()
    } else if undo {
        crate::tl!("Nothing to undo").to_string()
    } else {
        crate::tl!("Nothing to redo").to_string()
    };
    let face = face.on_hover_text(tip(ui, it)).on_disabled_hover_text(&disabled);
    let menu_label = crate::tf!("{label} options", label = crate::i18n::t(it.label));
    icons::describe_control(ui, &arrow, &menu_label, None, false);
    let arrow = arrow.on_hover_text(&menu_label).on_disabled_hover_text(&disabled);
    let mut steps = None;
    egui::Popup::from_toggle_button_response(&arrow).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
        ui.set_min_width(MENU_W);
        for (i, label) in labels.iter().enumerate() {
            let resp = ui.add(egui::Button::new(label.as_str()).min_size(vec2(MENU_W, MENU_ROW_H)));
            crate::control::record_widget(ui, &resp, label, "button", None);
            if resp.clicked() {
                steps = Some(i + 1);
            }
        }
    });
    if let Some(count) = steps {
        app.cmd.buffer.clear();
        // Errors are echoed to the command history by `run`.
        let _ = app.run(if undo { "undo" } else { "redo" }, json!({ "count": count }));
    } else if face.clicked() {
        start(app, it.cmd);
    }
    arrow_rect.right()
}

/// The history list's entries, the next step first: each step's command label, translated.
fn history_labels(app: &CadApp, undo: bool) -> Vec<String> {
    let Ok(st) = app.session.state() else { return Vec::new() };
    let stack = if undo { &st.undo } else { &st.redo };
    stack.iter().rev().take(HISTORY_MAX).map(|s| crate::i18n::t(&s.label).to_string()).collect()
}

/// The current layer drop-down (the Layers panel's).
fn layer(app: &mut CadApp, ui: &mut Ui, at: egui::Pos2) -> f32 {
    let rect = Rect::from_min_size(at, vec2(LAYER_W + 24.0, BUTTON.y));
    let builder = UiBuilder::new().max_rect(rect).id_salt("qat-layer").layout(egui::Layout::left_to_right(egui::Align::Center));
    let resp = ui.scope_builder(builder, |ui| crate::ribbon::combo(ui, app, Combo::Layer, LAYER_W)).inner;
    resp.rect.right().max(rect.left()) + 2.0
}

/// Our Customize glyph: a bar over a ▾.
fn paint_customize(ui: &Ui, rect: Rect) {
    let t = Tokens::get();
    let c = rect.center();
    ui.painter().line_segment([pos2(c.x - 4.0, c.y - 3.5), pos2(c.x + 4.0, c.y - 3.5)], Stroke::new(1.2, t.icon));
    let tri = vec![pos2(c.x - 4.0, c.y - 1.0), pos2(c.x + 4.0, c.y - 1.0), pos2(c.x, c.y + 3.5)];
    ui.painter().add(egui::Shape::convex_polygon(tri, t.icon, Stroke::NONE));
}

enum Choice {
    Item(&'static str),
    MenuBar,
    Below(bool),
}

/// Customize ▾: the items with check marks, then More Commands…, Show/Hide Menu Bar and Show
/// Below/Above the Ribbon.
fn customize(app: &mut CadApp, ui: &mut Ui, at: egui::Pos2) -> f32 {
    let rect = Rect::from_min_size(at, vec2(CUSTOMIZE_W, BUTTON.y));
    let resp = interact(ui, rect, ui.id().with("qat-customize"), true);
    paint_state(ui, rect, &resp);
    paint_customize(ui, rect);
    icons::describe_control(ui, &resp, CUSTOMIZE, None, false);
    let resp = resp.on_hover_text(crate::i18n::t(CUSTOMIZE));
    let mut choice = None;
    egui::Popup::from_toggle_button_response(&resp).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
        ui.set_min_width(MENU_W);
        ui.spacing_mut().item_spacing.y = 0.0;
        for it in ITEMS {
            if check_row(ui, it.label, Some(is_shown(app, it.cmd)), exists(it.cmd)).clicked() {
                choice = Some(Choice::Item(it.cmd));
            }
        }
        ui.add_space(3.0);
        ui.separator();
        ui.add_space(3.0);
        // AutoCAD's own customization dialog (CUI) is not built yet.
        check_row(ui, MORE_COMMANDS, None, false);
        let menu = app.ui.in_window_menu;
        if check_row(ui, if menu { HIDE_MENU_BAR } else { SHOW_MENU_BAR }, None, true).clicked() {
            choice = Some(Choice::MenuBar);
        }
        let below = app.ui.qat_below_ribbon;
        if check_row(ui, if below { SHOW_ABOVE } else { SHOW_BELOW }, None, true).clicked() {
            choice = Some(Choice::Below(!below));
        }
    });
    // Errors are echoed to the command history by `run`.
    let _ = match choice {
        Some(Choice::Item(cmd)) => app.run("ui.qat", json!({ "item": cmd })),
        Some(Choice::MenuBar) => app.run("ui.toggle.menubar", Value::Null),
        Some(Choice::Below(below)) => app.run("ui.qat", json!({ "below": below })),
        None => Ok(Value::Null),
    };
    rect.right()
}

/// One menu row: a check mark column (`checked: Some` for the items) and the label.
fn check_row(ui: &mut Ui, label: &str, checked: Option<bool>, enabled: bool) -> Response {
    let t = Tokens::get();
    let resp = ui.add_enabled_ui(enabled, |ui| ui.allocate_exact_size(vec2(MENU_W, MENU_ROW_H), Sense::click()).1).inner;
    let rect = resp.rect;
    if resp.hovered() && resp.enabled() {
        ui.painter().rect_filled(rect, 2.0, t.control_hover);
    }
    let color = if enabled { t.text } else { t.text_faint };
    if checked == Some(true) {
        let c = pos2(rect.left() + 12.0, rect.center().y);
        let points = vec![pos2(c.x - 4.0, c.y), pos2(c.x - 1.0, c.y + 3.0), pos2(c.x + 4.5, c.y - 3.5)];
        ui.painter().add(egui::Shape::line(points, Stroke::new(1.5, color)));
    }
    let galley = ui.painter().layout_no_wrap(crate::i18n::t(label).to_string(), egui::FontId::proportional(12.0), color);
    ui.painter().galley(pos2(rect.left() + 26.0, rect.center().y - galley.size().y / 2.0), galley, color);
    icons::describe_control(ui, &resp, label, checked, false);
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_autocads_first_items_without_the_web_ones() {
        assert_eq!(default_shown(), ["new", "open", "qsave", "saveas", "plot", "u", "redo"]);
        assert!(ITEMS.iter().all(|i| !i.label.contains("Web") && i.label != "Share"));
    }

    #[test]
    fn the_qat_command_toggles_items_in_autocads_order_and_validates() {
        let mut app = CadApp::new(cadcraft_engine::Session::default(), Default::default());
        let r = app.run("ui.qat", json!({"item": "Plot Preview"})).unwrap();
        assert_eq!(r["items"], json!(["new", "open", "qsave", "saveas", "plot", "u", "redo", "preview"]));
        app.run("ui.qat", json!({"item": "publish", "on": true})).unwrap();
        let r = app.run("ui.qat", json!({"item": "new", "on": false})).unwrap();
        assert_eq!(r["items"], json!(["open", "qsave", "saveas", "plot", "u", "redo", "publish", "preview"]));
        assert!(app.run("ui.qat", json!({"item": "render", "on": true})).unwrap_err().contains("not available yet"));
        assert!(app.run("ui.qat", json!({"item": "nope"})).is_err());
        assert!(app.run("ui.qat", json!({"item": 3})).is_err());
        assert!(app.run("ui.qat", json!({"below": "yes"})).is_err());
        assert_eq!(app.run("ui.qat", json!({"below": true})).unwrap()["below"], json!(true));
        // Ids no item has drop out on the next change.
        app.ui.qat_shown.push("bogus".into());
        app.run("ui.qat", json!({"item": "open", "on": true})).unwrap();
        assert!(!app.ui.qat_shown.iter().any(|c| c == "bogus"));
        app.run("ui.resetpalettes", Value::Null).unwrap();
        assert_eq!(app.ui.qat_shown, default_shown());
        assert!(!app.ui.qat_below_ribbon, "Reset Palettes restores AutoCAD's QAT");
    }
}
