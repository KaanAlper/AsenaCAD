//! The Layer Properties Manager (LAYER), laid out as AutoCAD 2026's palette: the title strip on
//! its left edge; "Current layer:" and the layer search box; the toolbar (layer filters and layer
//! states, the new / delete / set-current layer tools, Refresh and Settings); the filter tree with
//! Invert; the layer grid ([`grid`]) and the status line. It opens at the drawing area's top
//! left, 650 × 350, and keeps where it was moved and how it was sized. Every change runs an
//! engine command (`layer.*`, `layerstate.*`, `vplayer`, `linetype`).

use asenacad_doc::{Handle, Layer};
use egui::{Id, Pos2, Rect, RichText, Sense, Stroke, Ui, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::palettes::strip;
use crate::theme::Tokens;

mod grid;
mod pickers;

pub use pickers::{color_button, color_button_sized, color_palette, swatch};

/// The palette's size when it first opens (AutoCAD's).
pub const DEFAULT_SIZE: egui::Vec2 = egui::vec2(650.0, 350.0);
/// The smallest size it can be dragged to.
const MIN_SIZE: egui::Vec2 = egui::vec2(420.0, 220.0);
/// The filter tree's width.
const TREE_W: f32 = 95.0;
/// A toolbar button's width and height.
const TOOL: egui::Vec2 = egui::vec2(23.0, 25.0);

/// Palette-local UI state (kept in egui memory between frames).
#[derive(Clone, Debug, Default)]
pub(crate) struct LpmState {
    /// 0 = All, 1 = All Used Layers.
    filter: u8,
    /// Invert: show the layers the filter leaves out.
    invert: bool,
    /// The filter tree folded away («).
    tree_collapsed: bool,
    search: String,
    selected: Option<String>,
    /// (layer, buffer) while renaming.
    renaming: Option<(String, String)>,
    /// The rename has just started: select the whole name.
    rename_fresh: bool,
    /// (layer, buffer) while editing the description.
    describing: Option<(String, String)>,
    sort_desc: bool,
    state_name: String,
    state_sel: Option<String>,
}

/// Lower-case names of layers with objects on them, worked out once per drawing revision (and
/// again on Refresh).
fn used_layers(ctx: &egui::Context, st: &asenacad_engine::DocState) -> std::collections::HashSet<String> {
    let id = Id::new(("lpm-used", st.uid, st.revision));
    if let Some(set) = ctx.data(|d| d.get_temp::<std::sync::Arc<std::collections::HashSet<String>>>(id)) {
        return (*set).clone();
    }
    let d = &st.doc;
    let mut set = std::collections::HashSet::new();
    let ents = d.model.iter().chain(d.layouts.iter().flat_map(|l| l.entities.iter())).chain(d.blocks.values().flat_map(|b| b.entities.iter()));
    for e in ents.take(5_000_000) {
        if !set.contains(&e.common.layer.to_ascii_lowercase()) {
            set.insert(e.common.layer.to_ascii_lowercase());
        }
    }
    ctx.data_mut(|d| d.insert_temp(id, std::sync::Arc::new(set.clone())));
    set
}

/// Forget the used-layer scan (Refresh).
fn refresh(ctx: &egui::Context, st: &asenacad_engine::DocState) {
    ctx.data_mut(|d| d.remove::<std::sync::Arc<std::collections::HashSet<String>>>(Id::new(("lpm-used", st.uid, st.revision))));
}

fn natural_lower(s: &str) -> String {
    s.to_lowercase()
}

/// The filter's name in the tree and the status line.
fn filter_name(filter: u8) -> &'static str {
    if filter == 1 { "All Used Layers" } else { "All" }
}

/// Where the palette is: where it was left, else 650 × 350 at the drawing area's top left.
fn placement(app: &CadApp, ctx: &egui::Context) -> Rect {
    let saved = ctx.data_mut(|d| d.get_persisted::<[f32; 4]>(Id::new("lpm_rect")));
    let screen = ctx.content_rect();
    let rect = match saved {
        Some([x, y, w, h]) if [x, y, w, h].iter().all(|v| v.is_finite()) => Rect::from_min_size(pos2(x, y), vec2(w, h).max(MIN_SIZE)),
        _ => {
            let origin = app.canvas.rect.map_or(screen.min + vec2(24.0, 120.0), |r| r.min + vec2(4.0, 4.0));
            Rect::from_min_size(origin, DEFAULT_SIZE)
        }
    };
    // Keep the title strip on the screen.
    let min = screen.min;
    let max = (screen.max - vec2(strip::WIDTH * 2.0, 40.0)).max(min);
    rect.translate(rect.min.clamp(min, max) - rect.min)
}

/// The Layer Properties Manager palette.
pub fn dialog(app: &mut CadApp, ctx: &egui::Context, open: &mut bool) {
    let t = Tokens::get();
    let sid = Id::new("lpm_state");
    let mut st: LpmState = ctx.data_mut(|d| d.get_temp(sid)).unwrap_or_default();
    let mut actions: Vec<(&str, Value)> = Vec::new();
    let mut rect = placement(app, ctx);
    let paper = app.session.layout_space().is_paper();
    let Ok(dst) = app.session.state() else {
        *open = false;
        return;
    };
    let d = &dst.doc;
    let cur = d.header.str("CLAYER", "0");
    let used = used_layers(ctx, dst);
    let vp: Option<(Handle, asenacad_doc::Viewport)> = dst.active_viewport();
    let linetypes: Vec<asenacad_doc::Linetype> = d.linetypes.clone();
    let library: Vec<String> = asenacad_doc::library::standard_linetypes().into_iter().map(|l| l.name).filter(|n| d.linetype(n).is_none()).collect();
    let states: Vec<String> = d.layer_states.iter().map(|s| s.name.clone()).collect();
    // Named plot styles of the current layout's table (a colour-dependent table has none).
    let layout_table =
        app.session.layout_space().layout().and_then(|n| d.layout(n)).and_then(|l| asenacad_doc::plot_style_table(d, &l.page.plot_style_table));
    let plot_styles: Option<Vec<String>> = layout_table
        .filter(|t| !t.is_color_dependent())
        .map(|t| ["ByLayer", "ByBlock"].map(String::from).into_iter().chain(t.styles.iter().map(|s| s.name.clone())).collect());
    let mut layers: Vec<Layer> = d.layers.clone();
    layers.sort_by_key(|l| natural_lower(&l.name));
    if st.sort_desc {
        layers.reverse();
    }
    let total = layers.len();
    let q = st.search.to_lowercase();
    let shown: Vec<Layer> = layers
        .into_iter()
        .filter(|l| {
            let in_filter = st.filter == 0 || used.contains(&l.name.to_ascii_lowercase());
            in_filter != st.invert
        })
        .filter(|l| q.is_empty() || crate_wild(&q, &l.name.to_lowercase()))
        .collect();
    if st.selected.is_none() {
        st.selected = Some(cur.clone());
    }
    let cols = grid::columns(paper, vp.is_some());
    let gctx = grid::Ctx { cur: &cur, used: &used, vp: vp.as_ref(), linetypes: &linetypes, library: &library, plot_styles: plot_styles.as_deref() };
    let mut moved = egui::Vec2::ZERO;
    let mut resized = egui::Vec2::ZERO;
    let mut do_refresh = false;
    let screen = ctx.content_rect();
    egui::Area::new(Id::new("lpm_window")).order(egui::Order::Middle).fixed_pos(rect.min).movable(false).constrain_to(screen).show(ctx, |ui| {
        ui.set_min_size(rect.size());
        ui.set_max_size(rect.size());
        let pane = Rect::from_min_size(ui.min_rect().min, rect.size());
        ui.allocate_rect(pane, Sense::hover());
        ui.painter().rect_filled(pane, 0.0, t.panel);
        ui.painter().rect_stroke(pane, 0.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
        let strip_rect = Rect::from_min_max(pane.min, pos2(pane.left() + strip::WIDTH, pane.bottom()));
        let s = strip::show(ui, strip_rect, Id::new("lpm-strip"), "Layer Properties Manager", Icon::LayerProps, |ui| {
            ui.button(crate::tl!("Close")).clicked()
        });
        if s.close {
            *open = false;
        }
        if s.drag.dragged() {
            moved = s.drag.drag_delta();
        }
        // The palette's body: AutoCAD leaves a 3-point margin right of the strip, 6 above.
        let body = Rect::from_min_max(pos2(strip_rect.right() + 3.0, pane.top() + 6.0), pane.max - vec2(3.0, 6.0));
        let tree_w = if st.tree_collapsed { 22.0 } else { TREE_W };
        let grid_left = body.left() + tree_w + 4.0;
        top_row(ui, body, &cur, &mut st);
        toolbar(ui, body, grid_left, &states, &d.layers, &mut st, &mut actions, &mut do_refresh);
        let split = Rect::from_min_max(pos2(body.left(), body.top() + 52.0), pos2(body.right(), body.bottom() - 33.0));
        filters(ui, Rect::from_min_max(split.min, pos2(body.left() + tree_w, split.bottom())), vp.as_ref().map(|(h, _)| *h), &mut st);
        let grid_rect = Rect::from_min_max(pos2(grid_left, split.top()), split.max);
        ui.painter().rect_filled(grid_rect, 0.0, t.list_bg);
        strip::place(ui, grid_rect, |ui| {
            ui.set_clip_rect(grid_rect.intersect(ui.clip_rect()));
            ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
            egui::ScrollArea::horizontal().id_salt("lpm_table_h").auto_shrink([false, false]).show(ui, |ui| {
                grid::header(ui, &cols, &mut st);
                egui::ScrollArea::vertical()
                    .id_salt("lpm_table")
                    .auto_shrink([false, false])
                    .max_height(grid_rect.height() - grid::HEADER_H - 14.0)
                    .show(ui, |ui| {
                        for l in &shown {
                            grid::row(ui, l, &cols, &gctx, &mut st, &mut actions);
                        }
                    });
            });
        });
        // The status line.
        let status = crate::tf!(
            "{filter}: {shown} layers displayed of {total} total layers",
            filter = crate::i18n::t(filter_name(st.filter)),
            shown = shown.len(),
            total = total
        );
        let line = Rect::from_min_size(pos2(body.left() + 1.0, body.bottom() - 19.0), vec2(body.width() - 16.0, 17.0));
        strip::place(ui, line, |ui| ui.label(RichText::new(status).color(t.text)));
        // The size grip at the bottom-right corner.
        let corner = Rect::from_min_max(pane.max - vec2(12.0, 12.0), pane.max);
        let grip = ui.interact(corner, Id::new("lpm-resize"), Sense::drag()).on_hover_cursor(egui::CursorIcon::ResizeNwSe);
        for k in [4.0, 8.0] {
            ui.painter().line_segment(
                [pos2(corner.right() - k, corner.bottom() - 1.0), pos2(corner.right() - 1.0, corner.bottom() - k)],
                Stroke::new(1.0, t.text_faint),
            );
        }
        if grip.dragged() {
            resized = grip.drag_delta();
        }
    });
    if moved != egui::Vec2::ZERO || resized != egui::Vec2::ZERO {
        rect = Rect::from_min_size(rect.min + moved, (rect.size() + resized).max(MIN_SIZE));
        ctx.data_mut(|d| d.insert_persisted(Id::new("lpm_rect"), [rect.left(), rect.top(), rect.width(), rect.height()]));
    }
    if do_refresh {
        refresh(ctx, dst);
    }
    ctx.data_mut(|dd| dd.insert_temp(sid, st));
    for (c, p) in actions {
        let _ = app.run(c, p);
    }
}

/// "Current layer: <name>" at the left, the search box (with its magnifier) at the right.
fn top_row(ui: &mut Ui, body: Rect, cur: &str, st: &mut LpmState) {
    let t = Tokens::get();
    let label = Rect::from_min_size(body.min + vec2(2.0, 5.0), vec2(body.width() - 140.0, 17.0));
    strip::place(ui, label, |ui| ui.label(RichText::new(crate::tf!("Current layer: {layer}", layer = cur)).color(t.text)));
    let search = Rect::from_min_max(pos2(body.right() - 125.0, body.top() + 3.0), pos2(body.right(), body.top() + 21.0));
    strip::place(ui, search, |ui| {
        ui.add_sized(
            search.size(),
            egui::TextEdit::singleline(&mut st.search).hint_text(crate::tl!("Search for layer")).margin(vec2(4.0, 1.0)).id(Id::new("lpm-search")),
        );
    });
    icons::paint(ui.painter(), Rect::from_center_size(pos2(search.right() - 9.0, search.center().y), vec2(13.0, 13.0)), Icon::Search, false);
}

/// One toolbar button at `x`; true when clicked.
fn tool(ui: &mut Ui, at: Pos2, icon: Icon, label: &str, enabled: bool) -> bool {
    let mut clicked = false;
    strip::place(ui, Rect::from_min_size(at, TOOL), |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            clicked = icons::button(ui, icon, TOOL.x - 1.0, label, false).clicked();
        });
    });
    clicked
}

/// The toolbar, in AutoCAD's three groups: the filter tree's (New Property Filter, New Group
/// Filter | Layer States Manager), the grid's (New Layer, New Layer VP Frozen in All Viewports,
/// Delete Layer, Set Current) above the grid, and Refresh … Settings at the right.
#[allow(clippy::too_many_arguments)]
fn toolbar(
    ui: &mut Ui,
    body: Rect,
    grid_left: f32,
    states: &[String],
    layers: &[Layer],
    st: &mut LpmState,
    actions: &mut Vec<(&'static str, Value)>,
    refresh: &mut bool,
) {
    let t = Tokens::get();
    let y = body.top() + 28.0;
    // Layer filters aren't modelled yet: both New filter buttons are disabled.
    tool(ui, pos2(body.left(), y), Icon::PropertyFilter, "New Property Filter", false);
    tool(ui, pos2(body.left() + TOOL.x, y), Icon::GroupFilter, "New Group Filter", false);
    let sep = body.left() + 2.0 * TOOL.x + 3.0;
    ui.painter().line_segment([pos2(sep, y + 4.0), pos2(sep, y + TOOL.y - 4.0)], Stroke::new(1.0, t.border));
    let states_at = pos2(body.left() + 2.0 * TOOL.x + 6.0, y);
    let states_clicked = tool(ui, states_at, Icon::LayerStates, "Layer States Manager", true);
    let states_id = Id::new("lpm-states");
    if states_clicked {
        egui::Popup::toggle_id(ui.ctx(), states_id);
    }
    let anchor = Rect::from_min_size(states_at, TOOL);
    egui::Popup::new(states_id, ui.ctx().clone(), anchor, ui.layer_id())
        .open_memory(None)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .width(260.0)
        .show(|ui| layer_states(ui, states, st, actions));
    // The grid's tools, above the grid.
    if tool(ui, pos2(grid_left, y), Icon::LayerNew, "New Layer", true) {
        let mut n = 1;
        while layers.iter().any(|l| l.name.eq_ignore_ascii_case(&format!("Layer{n}"))) && n < 100_000 {
            n += 1;
        }
        let name = format!("Layer{n}");
        actions.push(("layer.new", json!({ "name": name })));
        st.renaming = Some((name.clone(), name.clone()));
        st.rename_fresh = true;
        st.selected = Some(name);
    }
    // Freezing a new layer in every layout viewport has no single command yet.
    tool(ui, pos2(grid_left + TOOL.x, y), Icon::LayerNewVpFrozen, "New Layer VP Frozen in All Viewports", false);
    let has = st.selected.is_some();
    if tool(ui, pos2(grid_left + 2.0 * TOOL.x, y), Icon::LayerDelete, "Delete Layer", has)
        && let Some(s) = &st.selected
    {
        actions.push(("layer.delete", json!({ "name": s })));
    }
    if tool(ui, pos2(grid_left + 3.0 * TOOL.x, y), Icon::MakeCurrent, "Set Current", has)
        && let Some(s) = &st.selected
    {
        actions.push(("layer.current", json!({ "name": s })));
    }
    // Refresh and Settings at the right end (AutoCAD has a third, greyed button between them
    // that was not identified; its place is left empty).
    let right = body.right() - 3.0 * TOOL.x - 4.0;
    if tool(ui, pos2(right, y), Icon::Refresh, "Refresh", true) {
        *refresh = true;
    }
    tool(ui, pos2(right + 2.0 * TOOL.x, y), Icon::Gear, "Settings", false);
}

/// The Layer States Manager: the saved states, Restore, Delete, Rename, and a new state's name.
fn layer_states(ui: &mut Ui, states: &[String], st: &mut LpmState, actions: &mut Vec<(&'static str, Value)>) {
    let t = Tokens::get();
    ui.label(RichText::new(crate::tl!("Layer States Manager")).strong());
    egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
        for s in states {
            if ui.selectable_label(st.state_sel.as_deref() == Some(s), s).clicked() {
                st.state_sel = Some(s.clone());
            }
        }
        if states.is_empty() {
            ui.label(RichText::new(crate::tl!("No saved layer states")).color(t.text_dim));
        }
    });
    ui.separator();
    ui.add(egui::TextEdit::singleline(&mut st.state_name).hint_text(crate::tl!("New state name")).desired_width(240.0));
    ui.horizontal(|ui| {
        let named = !st.state_name.trim().is_empty();
        if ui.add_enabled(named, egui::Button::new(crate::tl!("Save State"))).clicked() {
            let n = st.state_name.trim().to_string();
            actions.push(("layerstate.save", json!({ "name": n })));
            st.state_sel = Some(n);
            st.state_name.clear();
        }
        let has = st.state_sel.is_some();
        if ui.add_enabled(has, egui::Button::new(crate::tl!("Restore"))).clicked()
            && let Some(s) = &st.state_sel
        {
            actions.push(("layerstate.restore", json!({ "name": s })));
        }
        if ui.add_enabled(has, egui::Button::new(crate::tl!("Delete"))).clicked()
            && let Some(s) = st.state_sel.take()
        {
            actions.push(("layerstate.delete", json!({ "name": s })));
        }
        if ui.add_enabled(has && named, egui::Button::new(crate::tl!("Rename"))).clicked()
            && let Some(s) = st.state_sel.take()
        {
            let to = st.state_name.trim().to_string();
            actions.push(("layerstate.rename", json!({ "from": s, "to": to })));
            st.state_sel = Some(to);
            st.state_name.clear();
        }
    });
}

/// The filter tree (All ▸ All Used Layers) with its « fold button and Invert below it.
fn filters(ui: &mut Ui, rect: Rect, viewport: Option<Handle>, st: &mut LpmState) {
    let t = Tokens::get();
    ui.painter().rect_filled(rect, 0.0, t.list_bg);
    if st.tree_collapsed {
        let at = pos2(rect.left(), rect.top() + 1.0);
        strip::place(ui, Rect::from_min_size(at, vec2(22.0, 22.0)), |ui| {
            if icons::button(ui, Icon::DoubleChevronRight, 20.0, "Expand the filter tree", false).clicked() {
                st.tree_collapsed = false;
            }
        });
        return;
    }
    ui.painter().text(pos2(rect.left() + 4.0, rect.top() + 14.0), egui::Align2::LEFT_CENTER, crate::tl!("Filters"), crate::theme::body(), t.text);
    strip::place(ui, Rect::from_min_size(pos2(rect.left() + 68.0, rect.top() + 1.0), vec2(26.0, 25.0)), |ui| {
        if icons::button(ui, Icon::DoubleChevronLeft, 22.0, "Collapse the filter tree", false).clicked() {
            st.tree_collapsed = true;
        }
    });
    ui.painter().line_segment([pos2(rect.left(), rect.top() + 29.0), pos2(rect.right(), rect.top() + 29.0)], Stroke::new(1.0, t.border));
    for (i, indent) in [(0u8, 6.0), (1, 24.0)] {
        let row = Rect::from_min_size(pos2(rect.left() + indent, rect.top() + 33.0 + f32::from(i) * 18.0), vec2(rect.width() - indent - 2.0, 18.0));
        let resp = ui.interact(row, Id::new(("lpm-filter", i)), Sense::click());
        let name = crate::i18n::t(filter_name(i));
        if st.filter == i {
            ui.painter().rect_filled(row, 2.0, t.accent.gamma_multiply(0.45));
        } else if resp.hovered() {
            ui.painter().rect_filled(row, 2.0, t.control_hover.gamma_multiply(0.5));
        }
        icons::paint(ui.painter(), Rect::from_min_size(row.min + vec2(1.0, 2.0), vec2(14.0, 14.0)), Icon::LayerProps, false);
        let g = pickers::elided(ui, name, row.width() - 20.0, t.text);
        ui.painter().galley(pos2(row.left() + 18.0, row.center().y - g.size().y / 2.0), g, t.text);
        resp.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::SelectableLabel, true, st.filter == i, name));
        if resp.clicked() {
            st.filter = i;
        }
    }
    if let Some(h) = viewport {
        ui.painter().text(
            pos2(rect.left() + 6.0, rect.top() + 33.0 + 2.0 * 18.0 + 10.0),
            egui::Align2::LEFT_CENTER,
            crate::tf!("Viewport {id}", id = h.hex()),
            crate::theme::small(),
            t.text_dim,
        );
    }
    let foot = Rect::from_min_max(pos2(rect.left(), rect.bottom() - 25.0), rect.max);
    ui.painter().line_segment([foot.left_top(), foot.right_top()], Stroke::new(1.0, t.border));
    strip::place(ui, foot.shrink2(vec2(5.0, 2.0)), |ui| {
        ui.checkbox(&mut st.invert, crate::tl!("Invert"));
    });
}

/// Layer search: plain text matches anywhere; `*` / `?` are wildcards.
fn crate_wild(q: &str, name: &str) -> bool {
    if !q.contains('*') && !q.contains('?') {
        return name.contains(q);
    }
    wild(q.as_bytes(), name.as_bytes())
}

fn wild(p: &[u8], t: &[u8]) -> bool {
    let (mut pi, mut ti, mut star, mut mark) = (0usize, 0usize, None::<usize>, 0usize);
    while ti < t.len() {
        match p.get(pi) {
            Some(b'*') => {
                star = Some(pi);
                pi += 1;
                mark = ti;
            }
            Some(&c) if c == b'?' || Some(&c) == t.get(ti) => {
                pi += 1;
                ti += 1;
            }
            _ => match star {
                Some(s) => {
                    pi = s + 1;
                    mark += 1;
                    ti = mark;
                }
                None => return false,
            },
        }
    }
    p.iter().skip(pi).all(|c| *c == b'*')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_search() {
        assert!(crate_wild("wal", "walls"));
        assert!(crate_wild("w*s", "walls"));
        assert!(crate_wild("?alls", "walls"));
        assert!(!crate_wild("x*", "walls"));
    }

    /// Issue #29: just opening the palette must not grow it frame after frame.
    #[test]
    fn dialog_keeps_its_height() {
        let mut session = asenacad_engine::Session::new();
        session.open_drawing(asenacad_engine::sample::default_sample(), "Bracket", None);
        let mut app = CadApp::new(session, crate::Services::default());
        let ctx = egui::Context::default();
        let screen = Rect::from_min_size(egui::Pos2::ZERO, vec2(1600.0, 1000.0));
        let mut heights = Vec::new();
        for _ in 0..30 {
            let input = egui::RawInput { screen_rect: Some(screen), ..Default::default() };
            let mut out = ctx.run_ui(input, |ui| {
                let mut open = true;
                dialog(&mut app, ui.ctx(), &mut open);
            });
            out.textures_delta.clear();
            heights.push(ctx.memory(|m| m.area_rect(Id::new("lpm_window"))).map_or(0.0, |r| r.height()));
        }
        let (first, last) = (heights[5], heights[29]);
        assert!(first > 0.0 && (first - DEFAULT_SIZE.y).abs() < 0.5 && (last - first).abs() < 0.5, "palette height drifted: {heights:?}");
    }
}
