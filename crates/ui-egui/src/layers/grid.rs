//! The Layer Properties Manager's grid: AutoCAD's columns in AutoCAD's order (Plot Style only in
//! a layout, VP Freeze and VP Color inside a layout viewport), a row per layer, and each cell's
//! editing: click toggles, the colour, linetype and lineweight pickers, double-click rename and
//! description.

use asenacad_doc::{Handle, Layer, Lineweight};
use egui::accesskit::Role;
use egui::{Color32, Rect, RichText, Sense, Stroke, pos2, vec2};
use serde_json::{Value, json};

use super::LpmState;
use super::pickers::{color_button_sized, elided, lt_sample, lw_sample};
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// The header row's height.
pub(super) const HEADER_H: f32 = 23.0;
/// A layer row's height.
pub(super) const ROW_H: f32 = 18.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Col {
    Status,
    Name,
    On,
    Freeze,
    Lock,
    Plot,
    Color,
    Linetype,
    Lineweight,
    Transparency,
    PlotStyle,
    NewVpFreeze,
    VpFreeze,
    VpColor,
    Description,
}

impl Col {
    /// AutoCAD's column header.
    pub(super) fn name(self) -> &'static str {
        match self {
            Col::Status => "Status",
            Col::Name => "Name",
            Col::On => "On",
            Col::Freeze => "Freeze",
            Col::Lock => "Lock",
            Col::Plot => "Plot",
            Col::Color => "Color",
            Col::Linetype => "Linetype",
            Col::Lineweight => "Lineweight",
            Col::Transparency => "Transparency",
            Col::PlotStyle => "Plot Style",
            Col::NewVpFreeze => "New VP Freeze",
            Col::VpFreeze => "VP Freeze",
            Col::VpColor => "VP Color",
            Col::Description => "Description",
        }
    }

    /// The column's width when the palette opens (AutoCAD's widths).
    pub(super) fn width(self) -> f32 {
        match self {
            Col::Status | Col::On | Col::Freeze | Col::Lock | Col::Plot | Col::NewVpFreeze | Col::VpFreeze => 18.0,
            Col::Name => 75.0,
            Col::Color | Col::VpColor => 48.0,
            Col::Linetype | Col::PlotStyle => 64.0,
            Col::Lineweight => 68.0,
            Col::Transparency => 54.0,
            Col::Description => 102.0,
        }
    }
}

/// The columns shown: Plot Style only in a layout (paper space), the viewport columns only inside
/// a layout viewport.
pub(super) fn columns(paper: bool, viewport: bool) -> Vec<Col> {
    let mut cols =
        vec![Col::Status, Col::Name, Col::On, Col::Freeze, Col::Lock, Col::Plot, Col::Color, Col::Linetype, Col::Lineweight, Col::Transparency];
    if paper {
        cols.push(Col::PlotStyle);
    }
    cols.push(Col::NewVpFreeze);
    if viewport {
        cols.extend([Col::VpFreeze, Col::VpColor]);
    }
    cols.push(Col::Description);
    cols
}

/// What a row needs besides its layer.
pub(super) struct Ctx<'a> {
    pub cur: &'a str,
    pub used: &'a std::collections::HashSet<String>,
    pub vp: Option<&'a (Handle, asenacad_doc::Viewport)>,
    pub linetypes: &'a [asenacad_doc::Linetype],
    pub library: &'a [String],
    /// The layout's named plot styles; `None` in a colour-dependent drawing.
    pub plot_styles: Option<&'a [String]>,
}

/// The header row; clicking Name flips the sort order.
pub(super) fn header(ui: &mut egui::Ui, cols: &[Col], st: &mut LpmState) {
    let t = Tokens::get();
    let total: f32 = cols.iter().map(|c| c.width()).sum();
    let (r, _) = ui.allocate_exact_size(vec2(total, HEADER_H), Sense::hover());
    ui.painter().rect_filled(r, 0.0, t.chrome);
    let mut x = r.left();
    for c in cols {
        let cell = Rect::from_min_size(pos2(x, r.top()), vec2(c.width(), HEADER_H));
        x += c.width();
        // Clickable (Name sorts), so accessibility gets the header's place too.
        let resp = ui.interact(cell, ui.id().with(("lpm-col", c.name())), Sense::click());
        let name = crate::i18n::t(c.name());
        let arrow = if *c == Col::Name { 12.0 } else { 0.0 };
        let galley = elided(ui, name, cell.width() - 6.0 - arrow, t.text);
        let elided_name = galley.elided;
        ui.painter().galley(pos2(cell.left() + 3.0, cell.center().y - galley.size().y / 2.0), galley, t.text);
        if *c == Col::Name {
            let icon = if st.sort_desc { Icon::ChevronDown } else { Icon::ChevronUp };
            icons::paint(ui.painter(), Rect::from_center_size(pos2(cell.right() - 7.0, cell.center().y), vec2(9.0, 9.0)), icon, false);
        }
        ui.painter().line_segment([cell.right_top(), cell.right_bottom()], Stroke::new(1.0, t.border));
        ui.ctx().accesskit_node_builder(resp.id, |node| {
            node.set_role(Role::ColumnHeader);
            node.set_label(name);
        });
        if resp.clicked() {
            st.sort_desc = !st.sort_desc;
        }
        if elided_name {
            resp.on_hover_text(name);
        }
    }
}

/// Status column: a green check for the current layer, a sheet for the others.
fn status_icon(p: &egui::Painter, r: Rect, current: bool, used: bool) {
    if current {
        let g = Color32::from_rgb(0x4c, 0xc2, 0x5a);
        p.line_segment([pos2(r.left() + 3.0, r.center().y), pos2(r.left() + 7.0, r.bottom() - 4.0)], Stroke::new(2.0, g));
        p.line_segment([pos2(r.left() + 7.0, r.bottom() - 4.0), pos2(r.right() - 3.0, r.top() + 3.0)], Stroke::new(2.0, g));
    } else {
        let t = Tokens::get();
        let c = if used { t.icon } else { t.text_faint };
        let pts = vec![
            pos2(r.left() + 2.0, r.center().y + 1.0),
            pos2(r.center().x, r.top() + 4.0),
            pos2(r.right() - 2.0, r.center().y - 1.0),
            pos2(r.center().x, r.bottom() - 3.0),
        ];
        p.add(egui::Shape::closed_line(pts, Stroke::new(1.2, c)));
    }
}

/// A cell's rectangle and a child UI over it.
fn cell(ui: &mut egui::Ui, rect: Rect, add: impl FnOnce(&mut egui::Ui)) {
    crate::palettes::strip::place(ui, rect, |ui| {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        ui.spacing_mut().interact_size = vec2(rect.width(), ROW_H);
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), add);
    });
}

/// One layer's row.
pub(super) fn row(ui: &mut egui::Ui, l: &Layer, cols: &[Col], c: &Ctx, st: &mut LpmState, actions: &mut Vec<(&'static str, Value)>) {
    let t = Tokens::get();
    let name = l.name.clone();
    let total: f32 = cols.iter().map(|c| c.width()).sum();
    let (r, back) = ui.allocate_exact_size(vec2(total, ROW_H), Sense::click());
    let selected = st.selected.as_deref().is_some_and(|s| s.eq_ignore_ascii_case(&name));
    if selected {
        ui.painter().rect_filled(r, 0.0, t.accent.gamma_multiply(0.35));
    } else if back.hovered() {
        ui.painter().rect_filled(r, 0.0, t.control_hover.gamma_multiply(0.3));
    }
    let mut clicked = back.clicked();
    let mut x = r.left();
    for col in cols {
        let rect = Rect::from_min_size(pos2(x, r.top()), vec2(col.width(), ROW_H));
        x += col.width();
        cell(ui, rect, |ui| clicked |= cell_ui(ui, *col, rect, l, c, st, actions));
    }
    if clicked {
        st.selected = Some(name);
    }
}

/// A toggle cell: an icon button named after its column.
fn toggle(ui: &mut egui::Ui, icon: Icon, label: &str) -> egui::Response {
    icons::button(ui, icon, ROW_H, label, false)
}

/// One cell; true when it was clicked (which selects the row).
fn cell_ui(ui: &mut egui::Ui, col: Col, rect: Rect, l: &Layer, c: &Ctx, st: &mut LpmState, actions: &mut Vec<(&'static str, Value)>) -> bool {
    let t = Tokens::get();
    let name = l.name.clone();
    let current = name.eq_ignore_ascii_case(c.cur);
    let is_used = c.used.contains(&name.to_ascii_lowercase());
    match col {
        Col::Status => {
            let resp = ui.interact(rect, ui.id().with(("lpm-status", &name)), Sense::click());
            status_icon(ui.painter(), rect.shrink(1.0), current, is_used);
            let tip = if current {
                "Current layer"
            } else if is_used {
                "Layer contains objects (double-click to make current)"
            } else {
                "Layer is empty (double-click to make current)"
            };
            let label = if current {
                "Current layer"
            } else if is_used {
                "Layer in use"
            } else {
                "Layer is empty"
            };
            resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, crate::i18n::t(label)));
            let resp = resp.on_hover_text(crate::i18n::t(tip));
            if resp.double_clicked() {
                actions.push(("layer.current", json!({ "name": name })));
            }
            resp.clicked()
        }
        Col::Name => name_cell(ui, rect, l, st, actions),
        Col::On => {
            let r = toggle(ui, if l.on { Icon::Bulb } else { Icon::BulbOff }, "On");
            if r.clicked() {
                actions.push(("layer.set", json!({ "name": name, "on": !l.on })));
            }
            r.clicked()
        }
        Col::Freeze => {
            let r = toggle(ui, if l.frozen { Icon::Snowflake } else { Icon::Sun }, "Freeze");
            if r.clicked() {
                actions.push(("layer.set", json!({ "name": name, "frozen": !l.frozen })));
            }
            r.clicked()
        }
        Col::Lock => {
            let r = toggle(ui, if l.locked { Icon::Lock } else { Icon::Unlock }, "Lock");
            if r.clicked() {
                actions.push(("layer.set", json!({ "name": name, "locked": !l.locked })));
            }
            r.clicked()
        }
        Col::Plot => {
            let r = toggle(ui, Icon::Plot, "Plot");
            if !l.plot {
                let x = r.rect.shrink(3.0);
                ui.painter().line_segment([x.left_bottom(), x.right_top()], Stroke::new(1.6, Color32::from_rgb(0xe0, 0x50, 0x50)));
            }
            if r.clicked() {
                actions.push(("layer.set", json!({ "name": name, "plot": !l.plot })));
            }
            r.clicked()
        }
        Col::Color => {
            if let Some(color) = color_button_sized(ui, egui::Id::new(("lpm_color", &name)), l.color, false, rect.size()) {
                actions.push(("layer.set", json!({ "name": name, "color": color.name() })));
            }
            false
        }
        Col::Linetype => {
            linetype_cell(ui, rect, l, c, actions);
            false
        }
        Col::Lineweight => {
            lineweight_cell(ui, rect, l, actions);
            false
        }
        Col::Transparency => {
            // Commits when the drag or edit ends.
            let tid = egui::Id::new(("lpm_tr", &name));
            let mut tr = ui.data_mut(|d| d.get_temp::<u8>(tid)).unwrap_or(l.transparency);
            let r = ui.add_sized(rect.size(), egui::DragValue::new(&mut tr).range(0..=90).speed(0.5));
            if r.dragged() || r.has_focus() {
                ui.data_mut(|d| d.insert_temp(tid, tr));
            }
            if (r.drag_stopped() || r.lost_focus()) && tr != l.transparency {
                actions.push(("layer.set", json!({ "name": name, "transparency": tr })));
                ui.data_mut(|d| d.remove::<u8>(tid));
            } else if !r.dragged() && !r.has_focus() {
                ui.data_mut(|d| d.remove::<u8>(tid));
            }
            false
        }
        Col::PlotStyle => {
            match c.plot_styles {
                // Named plot styles: pick one of the layout's table.
                Some(styles) => {
                    let resp = ui.allocate_exact_size(rect.size(), Sense::click()).1;
                    let g = elided(ui, &l.plot_style, rect.width() - 4.0, t.text);
                    ui.painter().galley(pos2(rect.left() + 3.0, rect.center().y - g.size().y / 2.0), g, t.text);
                    egui::Popup::from_toggle_button_response(&resp).id(egui::Id::new(("lpm_ps", &name))).width(160.0).show(|ui| {
                        for s in styles {
                            if ui.selectable_label(s.eq_ignore_ascii_case(&l.plot_style), s).clicked() {
                                actions.push(("layer.set", json!({ "name": name, "plotStyle": s })));
                            }
                        }
                    });
                }
                // Colour-dependent: the layer plots with its colour's style; read-only.
                None => {
                    let text = match l.color {
                        asenacad_color::Color::Index(i) => format!("Color_{i}"),
                        _ => String::new(),
                    };
                    let g = elided(ui, &text, rect.width() - 4.0, t.text_faint);
                    ui.painter().galley(pos2(rect.left() + 3.0, rect.center().y - g.size().y / 2.0), g, t.text_faint);
                }
            }
            false
        }
        Col::NewVpFreeze => {
            let r = toggle(ui, if l.vp_freeze_new { Icon::Snowflake } else { Icon::Sun }, "New VP Freeze");
            // A small "new" mark distinguishes it from the Freeze column.
            let m = r.rect;
            ui.painter().rect_filled(Rect::from_min_size(m.right_bottom() - vec2(6.0, 6.0), vec2(5.0, 5.0)), 1.0, t.icon_accent);
            if r.clicked() {
                actions.push(("layer.set", json!({ "name": name, "newVpFreeze": !l.vp_freeze_new })));
            }
            r.clicked()
        }
        Col::VpFreeze => {
            let Some((h, v)) = c.vp else { return false };
            let vf = v.frozen_layers.iter().any(|f| f.eq_ignore_ascii_case(&name));
            let r = toggle(ui, if vf { Icon::Snowflake } else { Icon::Sun }, "VP Freeze");
            if r.clicked() {
                let key = if vf { "thaw" } else { "freeze" };
                actions.push(("vplayer", json!({ "handle": h.hex(), key: [name] })));
            }
            r.clicked()
        }
        Col::VpColor => {
            let Some((h, v)) = c.vp else { return false };
            let vc = v.layer_colors.iter().find(|(n, _)| n.eq_ignore_ascii_case(&name)).map(|(_, c)| *c);
            let w = if vc.is_some() { rect.width() - 12.0 } else { rect.width() };
            if let Some(color) = color_button_sized(ui, egui::Id::new(("lpm_vpcolor", &name)), vc.unwrap_or(l.color), false, vec2(w, ROW_H)) {
                actions.push(("vplayer", json!({ "handle": h.hex(), "colors": { name.clone(): color.name() } })));
            }
            if vc.is_some() && ui.small_button("×").on_hover_text(crate::tl!("Remove the viewport override")).clicked() {
                actions.push(("vplayer", json!({ "handle": h.hex(), "colors": { name.clone(): Value::Null } })));
            }
            false
        }
        Col::Description => description_cell(ui, rect, l, st, actions),
    }
}

/// Name: click selects, double-click renames (not 0 or Defpoints), right-click for the menu.
fn name_cell(ui: &mut egui::Ui, rect: Rect, l: &Layer, st: &mut LpmState, actions: &mut Vec<(&'static str, Value)>) -> bool {
    let t = Tokens::get();
    let name = l.name.clone();
    if let Some((n, mut buf)) = st.renaming.clone().filter(|(n, _)| n.eq_ignore_ascii_case(&name)) {
        let id = egui::Id::new(("lpm_rename", &name));
        let r = ui.add_sized(rect.size(), egui::TextEdit::singleline(&mut buf).id(id).margin(vec2(2.0, 0.0)));
        if std::mem::take(&mut st.rename_fresh) {
            // The proposed name is selected: typing replaces it.
            r.request_focus();
            if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), id) {
                let all = egui::text::CCursorRange::two(egui::text::CCursor::new(0), egui::text::CCursor::new(buf.chars().count()));
                state.cursor.set_char_range(Some(all));
                state.store(ui.ctx(), id);
            }
        } else if !r.has_focus() && !r.lost_focus() {
            r.request_focus();
        }
        if r.lost_focus() {
            let cancel = ui.input(|i| i.key_pressed(egui::Key::Escape));
            if !cancel && buf.trim() != name && !buf.trim().is_empty() {
                actions.push(("layer.set", json!({ "name": name, "newName": buf.trim() })));
                st.selected = Some(buf.trim().to_string());
            }
            st.renaming = None;
        } else {
            st.renaming = Some((n, buf));
        }
        return false;
    }
    let r = ui.interact(rect, ui.id().with(("lpm-name", &name)), Sense::click());
    let g = elided(ui, &name, rect.width() - 6.0, t.text);
    ui.painter().galley(pos2(rect.left() + 3.0, rect.center().y - g.size().y / 2.0), g, t.text);
    ui.ctx().accesskit_node_builder(r.id, |node| {
        node.set_role(Role::Cell);
        node.set_label(name.as_str());
    });
    if r.double_clicked() && name != "0" && !name.eq_ignore_ascii_case("Defpoints") {
        st.renaming = Some((name.clone(), name.clone()));
        st.rename_fresh = true;
    }
    r.context_menu(|ui| {
        if ui.button(crate::tl!("Set Current")).clicked() {
            actions.push(("layer.current", json!({ "name": name })));
            ui.close();
        }
        if ui.button(crate::tl!("Rename Layer")).clicked() {
            st.renaming = Some((name.clone(), name.clone()));
            st.rename_fresh = true;
            ui.close();
        }
        if ui.button(crate::tl!("Delete Layer")).clicked() {
            actions.push(("layer.delete", json!({ "name": name })));
            ui.close();
        }
    });
    r.clicked() || r.secondary_clicked()
}

/// Linetype: the layer's linetype; click lists the loaded ones and loads others.
fn linetype_cell(ui: &mut egui::Ui, rect: Rect, l: &Layer, c: &Ctx, actions: &mut Vec<(&'static str, Value)>) {
    let t = Tokens::get();
    let name = l.name.clone();
    let resp = ui.allocate_exact_size(rect.size(), Sense::click()).1;
    if resp.hovered() {
        ui.painter().rect_filled(rect, 2.0, t.control_hover.gamma_multiply(0.5));
    }
    let g = elided(ui, &l.linetype, rect.width() - 6.0, t.text);
    ui.painter().galley(pos2(rect.left() + 3.0, rect.center().y - g.size().y / 2.0), g, t.text);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, true, &l.linetype));
    egui::Popup::from_toggle_button_response(&resp)
        .id(egui::Id::new(("lpm_lt", &name)))
        .width(240.0)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.label(RichText::new(crate::tl!("Loaded linetypes")).small().color(t.text_dim));
            for lt in c.linetypes.iter().filter(|x| !["bylayer", "byblock"].contains(&x.name.to_ascii_lowercase().as_str())) {
                let (r, resp) = ui.allocate_exact_size(vec2(230.0, 20.0), Sense::click());
                if resp.hovered() || lt.name.eq_ignore_ascii_case(&l.linetype) {
                    ui.painter().rect_filled(r, 2.0, t.accent.gamma_multiply(if resp.hovered() { 0.6 } else { 0.3 }));
                }
                lt_sample(ui.painter(), Rect::from_min_size(pos2(r.left() + 4.0, r.top()), vec2(50.0, r.height())), Some(lt), t.text);
                ui.painter().text(pos2(r.left() + 62.0, r.center().y), egui::Align2::LEFT_CENTER, &lt.name, crate::theme::body(), t.text);
                if resp.on_hover_text(&lt.description).clicked() {
                    actions.push(("layer.set", json!({ "name": name, "linetype": lt.name })));
                    ui.close();
                }
            }
            if !c.library.is_empty() {
                ui.separator();
                ui.menu_button(crate::tl!("Load..."), |ui| {
                    egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
                        for n in c.library {
                            if ui.button(n).clicked() {
                                actions.push(("linetype", json!({ "load": n })));
                                actions.push(("layer.set", json!({ "name": name, "linetype": n })));
                                ui.close();
                            }
                        }
                    });
                    ui.separator();
                    if ui.button(crate::tl!("Load all")).clicked() {
                        actions.push(("linetype", json!({ "load": "*" })));
                        ui.close();
                    }
                });
            }
        });
}

/// Lineweight: a sample and the name; click lists Default and the standard weights.
fn lineweight_cell(ui: &mut egui::Ui, rect: Rect, l: &Layer, actions: &mut Vec<(&'static str, Value)>) {
    let t = Tokens::get();
    let name = l.name.clone();
    let resp = ui.allocate_exact_size(rect.size(), Sense::click()).1;
    if resp.hovered() {
        ui.painter().rect_filled(rect, 2.0, t.control_hover.gamma_multiply(0.5));
    }
    lw_sample(ui.painter(), Rect::from_min_size(pos2(rect.left() + 3.0, rect.top()), vec2(18.0, rect.height())), l.lineweight, t.text);
    let shown = crate::i18n::t(&l.lineweight.name()).to_string();
    let g = elided(ui, &shown, rect.width() - 26.0, t.text);
    ui.painter().galley(pos2(rect.left() + 24.0, rect.center().y - g.size().y / 2.0), g, t.text);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ComboBox, true, &shown));
    egui::Popup::from_toggle_button_response(&resp)
        .id(egui::Id::new(("lpm_lw", &name)))
        .width(170.0)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            egui::ScrollArea::vertical().max_height(320.0).show(ui, |ui| {
                let opts = std::iter::once(Lineweight::Default).chain(Lineweight::STANDARD.iter().map(|v| Lineweight::Mm100(*v)));
                for lw in opts {
                    let (r, resp) = ui.allocate_exact_size(vec2(160.0, 20.0), Sense::click());
                    if resp.hovered() || lw == l.lineweight {
                        ui.painter().rect_filled(r, 2.0, t.accent.gamma_multiply(if resp.hovered() { 0.6 } else { 0.3 }));
                    }
                    lw_sample(ui.painter(), Rect::from_min_size(pos2(r.left() + 4.0, r.top()), vec2(40.0, r.height())), lw, t.text);
                    ui.painter().text(
                        pos2(r.left() + 52.0, r.center().y),
                        egui::Align2::LEFT_CENTER,
                        crate::i18n::t(&lw.name()),
                        crate::theme::body(),
                        t.text,
                    );
                    if resp.clicked() {
                        let v = match lw {
                            Lineweight::Mm100(v) => json!(f64::from(v) / 100.0),
                            _ => json!("Default"),
                        };
                        actions.push(("layer.set", json!({ "name": name, "lineweight": v })));
                        ui.close();
                    }
                }
            });
        });
}

/// Description: double-click to edit; commits on focus loss, Escape cancels.
fn description_cell(ui: &mut egui::Ui, rect: Rect, l: &Layer, st: &mut LpmState, actions: &mut Vec<(&'static str, Value)>) -> bool {
    let t = Tokens::get();
    let name = l.name.clone();
    if let Some((n, mut buf)) = st.describing.clone().filter(|(n, _)| n.eq_ignore_ascii_case(&name)) {
        let r = ui.add_sized(rect.size(), egui::TextEdit::singleline(&mut buf).id(egui::Id::new(("lpm_desc", &name))).margin(vec2(2.0, 0.0)));
        if !r.has_focus() && !r.lost_focus() {
            r.request_focus();
        }
        if r.lost_focus() {
            if !ui.input(|i| i.key_pressed(egui::Key::Escape)) && buf != l.description {
                actions.push(("layer.set", json!({ "name": name, "description": buf })));
            }
            st.describing = None;
        } else {
            st.describing = Some((n, buf));
        }
        return false;
    }
    let r = ui.interact(rect, ui.id().with(("lpm-desc", &name)), Sense::click());
    let g = elided(ui, &l.description, rect.width() - 6.0, t.text);
    ui.painter().galley(pos2(rect.left() + 3.0, rect.center().y - g.size().y / 2.0), g, t.text);
    let r = r.on_hover_text(crate::tl!("Double-click to edit the description"));
    if r.double_clicked() {
        st.describing = Some((name, l.description.clone()));
    }
    r.clicked()
}
