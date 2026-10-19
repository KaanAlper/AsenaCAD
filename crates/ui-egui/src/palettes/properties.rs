//! The Properties palette (PROPERTIES, Ctrl+1), laid out as AutoCAD 2026's: the title strip on
//! its left edge, the object list with Toggle value of PICKADD Sysvar, Select Objects and Quick
//! Select, then collapsible groups of rows (a name column and a value column). With nothing
//! selected the rows are the current properties for new objects and the current view; with a
//! selection, the selected objects' properties ([`objects`]). Every value comes from the drawing
//! or the session and every edit runs a command; values AutoCAD shows read-only are greyed.

use asenacad_color::Color;
use egui::accesskit::Role;
use egui::{Align, Id, Layout, Rect, Sense, Stroke, Ui, UiBuilder, pos2, vec2};
use serde_json::{Value, json};

use super::strip;
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

mod objects;

/// The name column's width.
const LABEL_W: f32 = 103.0;
/// A row's (and a group header's) height.
const ROW_H: f32 = 19.0;
/// Space above a group header that follows another group.
const GROUP_GAP: f32 = 4.0;
/// Space left of the groups (AutoCAD's narrow gutter) and right of them (the scroll bar).
const GRID_LEFT: f32 = 20.0;
const GRID_RIGHT: f32 = 14.0;

/// The palette as the docking area shows it: the title strip, the object list and its buttons,
/// and the scrolling groups.
pub fn panel(app: &mut CadApp, ui: &mut Ui) {
    let r = ui.max_rect();
    let strip_rect = Rect::from_min_max(r.min, pos2(r.left() + strip::WIDTH, r.bottom()));
    let s = strip::show(ui, strip_rect, Id::new("properties-palette"), "Properties", Icon::Properties, |ui| ui.button(crate::tl!("Close")).clicked());
    if s.drag.drag_started() {
        craft_ui::docking::begin_drag(ui.ctx(), Id::new("asenacad-docking"), crate::docking::Panel::Properties, strip_rect);
    }
    let body = Rect::from_min_max(pos2(strip_rect.right(), r.top()), r.max);
    ui.scope_builder(UiBuilder::new().max_rect(body), |ui| {
        ui.add_space(12.0);
        header(app, ui);
        ui.add_space(7.0);
        egui::ScrollArea::vertical().id_salt("dock-properties").auto_shrink([false, false]).show(ui, |ui| {
            crate::shortcut_menus::palette(app, ui, grid, crate::shortcut_menus::properties_entries);
        });
    });
    if s.close {
        app.start("propertiesclose");
    }
}

/// The object list and the three buttons, then the groups (AsenaCAD's right-hand palettes).
pub(crate) fn section(app: &mut CadApp, ui: &mut Ui) {
    header(app, ui);
    ui.add_space(4.0);
    grid(app, ui);
}

/// The object list ("No selection", one type, or "All (n)"; its list narrows the selection to one
/// type) and Toggle value of PICKADD Sysvar, Select Objects, Quick Select.
fn header(app: &mut CadApp, ui: &mut Ui) {
    let sel = app.session.selection();
    let pickadd = app.session.settings.pickadd;
    let Ok(d) = app.session.doc() else { return };
    // Selected handles by type, in first-seen order.
    let mut types: Vec<(&'static str, Vec<String>)> = Vec::new();
    for h in &sel {
        if let Some(e) = d.entity(*h) {
            let n = e.kind.type_name();
            match types.iter_mut().find(|(t, _)| *t == n) {
                Some((_, hs)) => hs.push(h.hex()),
                None => types.push((n, vec![h.hex()])),
            }
        }
    }
    let count: usize = types.iter().map(|(_, hs)| hs.len()).sum();
    let shown = match (count, types.as_slice()) {
        (0, _) => crate::tl!("No selection").to_string(),
        (1, [(n, _)]) => crate::i18n::t(n).to_string(),
        (n, [(ty, _)]) => format!("{} ({n})", crate::i18n::t(ty)),
        (n, _) => format!("{} ({n})", crate::tl!("All")),
    };
    let mut run: Option<(&str, Value)> = None;
    let mut start: Option<&str> = None;
    ui.horizontal(|ui| {
        ui.add_space(GRID_LEFT - 1.0);
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        let w = (ui.available_width() - 3.0 * 24.0 - 3.0 - GRID_RIGHT).max(60.0);
        let resp = ui.allocate_ui(vec2(w, 24.0), |ui| value_box(ui, &shown, true, true, 24.0)).inner;
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, true, &shown));
        if count > 0 {
            egui::Popup::from_toggle_button_response(&resp).width(w).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
                for (ty, hs) in &types {
                    if ui.button(format!("{} ({})", crate::i18n::t(ty), hs.len())).clicked() {
                        run = Some(("select", json!({ "handles": hs })));
                    }
                }
            });
        }
        ui.add_space(3.0);
        if icons::button(ui, Icon::PickAdd, 24.0, "Toggle value of PICKADD Sysvar", pickadd).clicked() {
            run = Some(("setvar", json!({ "name": "PICKADD", "value": i32::from(!pickadd) })));
        }
        if icons::button(ui, Icon::SelectObjects, 24.0, "Select Objects", false).clicked() {
            start = Some("select");
        }
        if icons::button(ui, Icon::QuickSelect, 24.0, "Quick Select", false).clicked() {
            start = Some("ui.dialog.qselect");
        }
    });
    if let Some((c, p)) = run {
        let _ = app.run(c, p);
    }
    if let Some(c) = start {
        app.start(c);
    }
}

/// The groups: the selection's ([`objects`]) or, with nothing selected, the current ones.
fn grid(app: &mut CadApp, ui: &mut Ui) {
    let width = (ui.available_width() - GRID_LEFT - GRID_RIGHT).max(120.0);
    ui.horizontal_top(|ui| {
        ui.add_space(GRID_LEFT);
        ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(width);
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            if app.session.selection().is_empty() {
                current(app, ui);
            } else {
                objects::show(app, ui);
            }
        });
    });
}

/// Rows of the grid: group headers, then name/value rows.
pub(super) struct Rows {
    first: bool,
}

impl Rows {
    pub(super) fn new() -> Self {
        Self { first: true }
    }

    /// A group header (click to collapse or expand it); true while the group is open.
    pub(super) fn group(&mut self, ui: &mut Ui, collapsed: &mut Vec<String>, title: &str) -> bool {
        let t = Tokens::get();
        if !std::mem::take(&mut self.first) {
            ui.add_space(GROUP_GAP);
        }
        let key = format!("properties:{title}");
        let mut open = !collapsed.contains(&key);
        let (r, resp) = ui.allocate_exact_size(vec2(ui.available_width(), ROW_H), Sense::click());
        ui.painter().rect_filled(r, 0.0, t.chrome_dark);
        ui.painter().text(
            pos2(r.left() + 6.0, r.center().y),
            egui::Align2::LEFT_CENTER,
            crate::i18n::t(title),
            egui::FontId::proportional(crate::theme::body().size),
            t.text,
        );
        let chevron = if open { Icon::ChevronDown } else { Icon::ChevronRight };
        icons::paint(ui.painter(), Rect::from_center_size(pos2(r.right() - 10.0, r.center().y), vec2(10.0, 10.0)), chevron, false);
        if resp.clicked() {
            open = !open;
            if open {
                collapsed.retain(|k| *k != key);
            } else {
                collapsed.push(key);
            }
        }
        ui.ctx().accesskit_node_builder(resp.id, |node| {
            node.set_role(Role::Heading);
            node.set_label(crate::i18n::t(title));
            node.set_expanded(open);
        });
        crate::control::record_widget(ui, &resp, title, "button", Some(open));
        open
    }
}

/// A row: its name in the left column, `add` fills the value column. `editable` false greys the
/// name and tells accessibility the value is read-only; `shown` is the value's text.
pub(super) fn row(ui: &mut Ui, label: &str, editable: bool, shown: &str, add: impl FnOnce(&mut Ui)) {
    let t = Tokens::get();
    let w = ui.available_width();
    let (r, _) = ui.allocate_exact_size(vec2(w, ROW_H), Sense::hover());
    let name_r = Rect::from_min_size(r.min, vec2(LABEL_W.min(w * 0.5), ROW_H - 1.0));
    let value_r = Rect::from_min_max(pos2(name_r.right() + 1.0, r.top()), pos2(r.right(), r.bottom() - 1.0));
    ui.painter().rect_filled(name_r, 0.0, t.control.gamma_multiply(0.45));
    let text = crate::i18n::t(label);
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), crate::theme::body(), if editable { t.text } else { t.text_dim });
    job.wrap = egui::text::TextWrapping::truncate_at_width(name_r.width() - 16.0);
    let galley = ui.painter().layout_job(job);
    let truncated = galley.elided;
    ui.painter().galley(pos2(name_r.left() + 14.0, name_r.center().y - galley.size().y / 2.0), galley, t.text);
    // Sensed for clicks so accessibility gets the row's place.
    let resp = ui.interact(name_r, ui.id().with(("property-row", label)), Sense::click());
    ui.ctx().accesskit_node_builder(resp.id, |node| {
        node.set_role(Role::RowHeader);
        node.set_label(text);
        node.set_value(shown);
        if !editable {
            node.set_disabled();
        }
    });
    if truncated {
        resp.on_hover_text(text);
    }
    strip::place(ui, value_r, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        ui.spacing_mut().button_padding = vec2(4.0, 0.0);
        ui.spacing_mut().interact_size.y = ROW_H - 1.0;
        ui.with_layout(Layout::left_to_right(Align::Center), add);
    });
}

/// A read-only row: greyed name and value.
pub(super) fn read_only_row(ui: &mut Ui, label: &str, value: &str) {
    row(ui, label, false, value, |ui| read_only(ui, value));
}

/// A value cell's box: `text`, with ▾ for a list; greyed when not `enabled`.
pub(super) fn value_box(ui: &mut Ui, text: &str, combo: bool, enabled: bool, height: f32) -> egui::Response {
    let t = Tokens::get();
    let w = ui.available_width().max(20.0);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, height), if enabled { Sense::click() } else { Sense::hover() });
    let fill = if enabled && resp.hovered() {
        t.control_hover
    } else if enabled {
        t.control
    } else {
        t.panel
    };
    ui.painter().rect_filled(rect, 1.0, fill);
    if !enabled {
        ui.painter().rect_stroke(rect, 1.0, Stroke::new(1.0, t.control), egui::StrokeKind::Inside);
    }
    let right = if combo { 18.0 } else { 4.0 };
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), crate::theme::body(), if enabled { t.text } else { t.text_faint });
    job.wrap = egui::text::TextWrapping::truncate_at_width((rect.width() - 6.0 - right).max(8.0));
    let galley = ui.painter().layout_job(job);
    ui.painter().galley(pos2(rect.left() + 5.0, rect.center().y - galley.size().y / 2.0), galley, t.text);
    if combo {
        icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 9.0, rect.center().y), vec2(10.0, 10.0)), Icon::ChevronDown, !enabled);
    }
    resp
}

/// A list value: a box with ▾ that opens `items`; returns the item picked.
pub(super) fn pick_menu(ui: &mut Ui, current: &str, items: impl IntoIterator<Item = String>) -> Option<String> {
    // Names are the drawing's own; only the ByLayer/ByBlock keywords are translated.
    let shown = |s: &str| if ["ByLayer", "ByBlock"].contains(&s) { crate::i18n::t(s).to_string() } else { s.to_string() };
    let mut out = None;
    menu(ui, &shown(current), |ui| {
        for it in items {
            if ui.button(shown(&it)).clicked() {
                out = Some(it);
                ui.close();
            }
        }
    });
    out
}

/// A box with ▾ showing `text` whose click opens `body`.
pub(super) fn menu(ui: &mut Ui, text: &str, body: impl FnOnce(&mut Ui)) {
    let resp = value_box(ui, text, true, true, ROW_H - 1.0);
    let width = resp.rect.width().max(140.0);
    egui::Popup::from_toggle_button_response(&resp).width(width).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        egui::ScrollArea::vertical().max_height(320.0).show(ui, body);
    });
}

/// An editable numeric/text field that commits on Enter or focus loss.
pub(super) fn edit_field(ui: &mut Ui, id: Id, value: String) -> Option<String> {
    let mut buf = ui.data_mut(|d| d.get_temp::<String>(id)).unwrap_or_else(|| value.clone());
    let w = ui.available_width();
    let te = id.with("te");
    let resp = ui.add_sized([w, ROW_H - 1.0], egui::TextEdit::singleline(&mut buf).id(te).font(crate::theme::body()).margin(vec2(4.0, 1.0)));
    let mut out = None;
    // Keyboard focus from memory: `Response::has_focus` is false while the window itself is
    // unfocused, which dropped the typed text every frame.
    if ui.memory(|m| m.has_focus(te)) {
        ui.data_mut(|d| d.insert_temp(id, buf.clone()));
    } else {
        ui.data_mut(|d| d.remove::<String>(id));
    }
    if resp.lost_focus() && buf != value {
        out = Some(buf);
    }
    out
}

/// A committed numeric field read as the command line reads numbers (`36`, `3'`, `3'-6 1/2"`,
/// `6 1/2`). Text it can't read goes to `bad`, to be reported rather than dropped.
pub(super) fn number(typed: Option<String>, bad: &mut Option<String>) -> Option<f64> {
    let typed = typed?;
    let v = asenacad_engine::units::parse_distance(&typed);
    if v.is_none() {
        *bad = Some(typed);
    }
    v
}

/// Reports a numeric field the user typed that isn't a number; the field shows its value again.
pub(super) fn report_bad(app: &mut CadApp, bad: Option<String>) {
    if let Some(text) = bad {
        app.session.echo(crate::tf!("Not a number or distance: {text}", text = text.trim()));
    }
}

/// A greyed value with no command behind it yet: visibly inert, never a silent dead button.
pub(super) fn unavailable(ui: &mut Ui, text: &str) {
    value_box(ui, crate::i18n::t(text), false, false, ROW_H - 1.0).on_hover_text(crate::tl!("Not available yet"));
}

/// A greyed value computed from the drawing or the view.
pub(super) fn read_only(ui: &mut Ui, text: &str) {
    value_box(ui, crate::i18n::t(text), false, false, ROW_H - 1.0).on_hover_text(crate::tl!("Read-only"));
}

/// The colours the Color lists offer before the true-colour and index pickers.
pub(super) const COLORS: [(&str, i64); 9] =
    [("ByLayer", 256), ("ByBlock", 0), ("Red", 1), ("Yellow", 2), ("Green", 3), ("Cyan", 4), ("Blue", 5), ("Magenta", 6), ("White", 7)];

/// With nothing selected: General, 3D Visualization, Plot style, View and Misc.
fn current(app: &mut CadApp, ui: &mut Ui) {
    let canvas = app.canvas.rect;
    let space = app.session.layout_space();
    let ucs_icon = app.ui.show_ucs_icon;
    let Ok(st) = app.session.state() else { return };
    let d = &st.doc;
    let h = &d.header;
    // Lengths are shown as the drawing's units say (UNITS), and read back in any format.
    let (lu, lp) = (h.i64("LUNITS", 2), h.i64("LUPREC", 4));
    let len = |v: f64| asenacad_engine::units::format_distance(v, lu, lp);
    let mut bad: Option<String> = None;
    let mut action: Option<(&str, Value)> = None;
    let collapsed = &mut app.ui.collapsed_groups;
    let mut rows = Rows::new();
    if rows.group(ui, collapsed, "General") {
        let color = crate::i18n::t(&Color::from_aci(h.i64("CECOLOR", 256) as i16).name()).to_string();
        row(ui, "Color", true, &color, |ui| {
            menu(ui, &color, |ui| {
                for (n, c) in COLORS {
                    if ui.button(crate::i18n::t(n)).clicked() {
                        action = Some(("color", json!({ "color": c })));
                        ui.close();
                    }
                }
            });
        });
        let layer = h.str("CLAYER", "0");
        row(ui, "Layer", true, &layer, |ui| {
            if let Some(l) = pick_menu(ui, &layer, d.layers.iter().map(|l| l.name.clone())) {
                action = Some(("layer.current", json!({ "name": l })));
            }
        });
        let linetype = h.str("CELTYPE", "ByLayer");
        row(ui, "Linetype", true, &linetype, |ui| {
            let items = ["ByLayer", "ByBlock"]
                .map(String::from)
                .into_iter()
                .chain(d.linetypes.iter().map(|l| l.name.clone()).filter(|l| !["ByLayer", "ByBlock"].contains(&l.as_str())));
            if let Some(l) = pick_menu(ui, &linetype, items) {
                action = Some(("linetype", json!({ "current": l })));
            }
        });
        let lts = format!("{:.4}", h.f64("CELTSCALE", 1.0));
        row(ui, "Linetype scale", true, &lts, |ui| {
            if let Some(f) = number(edit_field(ui, ui.id().with("cur_celtscale"), lts.clone()), &mut bad) {
                action = Some(("setvar", json!({ "name": "CELTSCALE", "value": f })));
            }
        });
        let lw = crate::i18n::t(&asenacad_doc::Lineweight::from_dxf(h.i64("CELWEIGHT", -1) as i16).name()).to_string();
        row(ui, "Lineweight", true, &lw, |ui| {
            menu(ui, &lw, |ui| {
                for n in ["ByLayer", "ByBlock", "Default"] {
                    if ui.button(crate::i18n::t(n)).clicked() {
                        action = Some(("lweight", json!({ "lineweight": n })));
                        ui.close();
                    }
                }
                for v in asenacad_doc::Lineweight::STANDARD {
                    if ui.button(format!("{:.2} mm", f64::from(v) / 100.0)).clicked() {
                        action = Some(("lweight", json!({ "lineweight": f64::from(v) / 100.0 })));
                        ui.close();
                    }
                }
            });
        });
        // New objects get ByLayer transparency; a current transparency (CETRANSPARENCY) isn't
        // modelled yet, so the value can't be changed here.
        let tr = asenacad_doc::Common::default().transparency.name();
        row(ui, "Transparency", false, &tr, |ui| unavailable(ui, &tr));
        let thickness = len(h.f64("THICKNESS", 0.0));
        row(ui, "Thickness", true, &thickness, |ui| {
            if let Some(f) = number(edit_field(ui, ui.id().with("cur_thickness"), thickness.clone()), &mut bad) {
                action = Some(("setvar", json!({ "name": "THICKNESS", "value": f })));
            }
        });
    }
    if rows.group(ui, collapsed, "3D Visualization") {
        // AsenaCAD has no materials: the row is there, empty and greyed.
        row(ui, "Material", false, "", |ui| unavailable(ui, ""));
    }
    if rows.group(ui, collapsed, "Plot style") {
        // The current layout's plot style table; named tables give the current plot style.
        let layout = space.layout().and_then(|n| d.layout(n).map(|l| (n.to_string(), l.page.plot_style_table.clone())));
        let table = layout.as_ref().and_then(|(_, t)| asenacad_doc::plot_style_table(d, t));
        match table.as_ref().filter(|t| !t.is_color_dependent()) {
            Some(t) => {
                let cur = h.str("CPLOTSTYLE", "ByLayer");
                row(ui, "Plot style", true, &cur, |ui| {
                    let items = ["ByLayer", "ByBlock"].map(String::from).into_iter().chain(t.styles.iter().map(|s| s.name.clone()));
                    if let Some(n) = pick_menu(ui, &cur, items) {
                        action = Some(("plotstyle", json!({ "name": n })));
                    }
                });
            }
            None => read_only_row(ui, "Plot style", "ByColor"),
        }
        match &layout {
            Some((name, current)) => {
                let shown = if current.trim().is_empty() { "None".to_string() } else { current.clone() };
                row(ui, "Plot style table", true, &shown, |ui| {
                    let items = std::iter::once("None".to_string()).chain(asenacad_doc::plot_style_table_names(d));
                    if let Some(n) = pick_menu(ui, &shown, items) {
                        action = Some(("pagesetup", json!({ "layout": name, "plotStyleTable": n })));
                    }
                });
            }
            // Model space page setups aren't stored yet.
            None => row(ui, "Plot style table", false, "None", |ui| unavailable(ui, "None")),
        }
        let attached = layout.as_ref().map(|(n, _)| n.clone()).unwrap_or_else(|| "Model".into());
        read_only_row(ui, "Plot table attached to", &attached);
        let kind = match &table {
            Some(t) if t.is_color_dependent() => "Color-dependent",
            Some(_) => "Named",
            None => "Not available",
        };
        read_only_row(ui, "Plot table type", kind);
    }
    if rows.group(ui, collapsed, "View") {
        let view = st.view();
        read_only_row(ui, "Center X", &len(view.center.x));
        read_only_row(ui, "Center Y", &len(view.center.y));
        read_only_row(ui, "Center Z", &len(0.0));
        read_only_row(ui, "Height", &len(view.height));
        // The view's width follows the drawing area's shape.
        let width = canvas.filter(|r| r.height() > 0.0).map(|r| len(view.height * f64::from(r.width() / r.height()))).unwrap_or_default();
        read_only_row(ui, "Width", &width);
    }
    if rows.group(ui, collapsed, "Misc") {
        let scale = d.current_anno_scale().name;
        row(ui, "Annotation scale", true, &scale, |ui| {
            if let Some(n) = pick_menu(ui, &scale, d.scales.iter().map(|s| s.name.clone())) {
                action = Some(("setvar", json!({ "name": "CANNOSCALE", "value": n })));
            }
        });
        let on = crate::i18n::t(if ucs_icon { "Yes" } else { "No" }).to_string();
        row(ui, "UCS icon On", true, &on, |ui| {
            menu(ui, &on, |ui| {
                for (n, v) in [("Yes", true), ("No", false)] {
                    if ui.button(crate::i18n::t(n)).clicked() {
                        action = Some(("ui.toggle.ucsicon", json!({ "on": v })));
                        ui.close();
                    }
                }
            });
        });
        // The UCS icon is always drawn at the drawing area's corner, never at the origin.
        read_only_row(ui, "UCS icon at origin", crate::i18n::t("No"));
    }
    if let Some((c, p)) = action {
        let _ = app.run(c, p);
    }
    report_bad(app, bad);
}
