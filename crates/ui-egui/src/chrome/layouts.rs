//! The layout bar at the left of the status bar row, in AutoCAD's order: the "≡" layout list,
//! the Model tab, one tab per layout (in tab order) and "+" for a new layout after the last one.
//! Tabs are slanted like the document tabs, with a "/" between inactive ones; the current tab
//! is filled and joins the band along the bar's top edge.

use egui::{Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use super::{FILE_BUTTON_H, FILE_MENU_W, FILE_PLUS_W, FILE_TAB_PAD, FILE_TAB_SLANT, elided, tab_separator, tab_shape, visible_tabs};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// Widest layout tab name before it is cut with "…".
const TAB_TEXT: f32 = 150.0;
/// The "more layouts" button shown when some layouts don't fit.
const MORE_W: f32 = 20.0;
/// The layout list button's label (also the overflow menu's).
pub const LIST: &str = "Layout list";
pub const NEW: &str = "New layout";
pub const MORE: &str = "More layouts";

/// The layout last current in paper space, which MODEL switches back to from model space.
fn last_layout_id() -> egui::Id {
    egui::Id::new("asenacad-last-layout")
}

/// The tab names: "Model" first, then the layouts in tab order.
fn tab_names(app: &CadApp) -> Vec<String> {
    let mut tabs: Vec<String> = vec!["Model".into()];
    if let Ok(st) = app.session.state() {
        let mut ls: Vec<_> = st.doc.layouts.iter().collect();
        ls.sort_by_key(|l| l.tab_order);
        tabs.extend(ls.iter().map(|l| l.name.clone()));
    }
    tabs
}

fn shown_name(i: usize, name: &str) -> String {
    if i == 0 { crate::tl!("Model").to_string() } else { name.to_string() }
}

/// The layout MODEL switches to from model space: the last one that was current, else the first.
pub fn return_layout(app: &CadApp, ctx: &egui::Context) -> Option<String> {
    let tabs = tab_names(app);
    let last: Option<String> = ctx.data(|d| d.get_temp(last_layout_id()));
    last.filter(|l| tabs.iter().skip(1).any(|t| t == l)).or_else(|| tabs.get(1).cloned())
}

/// The tab width of `name` (the "Model" tab: index 0).
fn tab_width(ui: &egui::Ui, i: usize, name: &str) -> f32 {
    let font = crate::theme::ribbon_font(ui.ctx());
    elided(ui.painter(), &shown_name(i, name), font, Tokens::get().text, TAB_TEXT).0.size().x + FILE_TAB_SLANT + 2.0 * FILE_TAB_PAD
}

/// The narrowest the layout bar gets: ≡, Model, the current layout, the "more" button and "+".
pub fn min_width(app: &CadApp, ui: &egui::Ui) -> f32 {
    let tabs = tab_names(app);
    let mut w = 4.0 + FILE_MENU_W + 4.0 + tab_width(ui, 0, "Model") + FILE_PLUS_W + FILE_TAB_SLANT;
    if tabs.len() > 1 {
        w += MORE_W;
    }
    if let asenacad_doc::Space::Paper(name) = app.session.layout_space()
        && let Some(i) = tabs.iter().position(|t| *t == name)
    {
        w += tab_width(ui, i, &name);
    }
    w
}

/// Draw the layout bar in `row` (the bar below its top band), keeping the tabs left of `limit`.
/// Returns where the bar ends.
pub fn show(app: &mut CadApp, ui: &mut egui::Ui, row: Rect, band_top: f32, limit: f32) -> f32 {
    let t = Tokens::get();
    let tabs = tab_names(app);
    let cur = app.session.state().map(|s| match &s.space {
        asenacad_doc::Space::Model => "Model".to_string(),
        asenacad_doc::Space::Paper(n) => n.clone(),
    });
    if let Ok(name) = cur.as_ref()
        && name != "Model"
    {
        ui.ctx().data_mut(|d| d.insert_temp(last_layout_id(), name.clone()));
    }
    let active_tab = tabs.iter().position(|n| cur.as_deref().is_ok_and(|c| c == n));
    let top = row.top();
    let bottom = (top + FILE_BUTTON_H + 2.0).min(row.bottom());
    let menu = Rect::from_min_size(pos2(row.left() + 4.0, row.center().y - FILE_BUTTON_H / 2.0), vec2(FILE_MENU_W, FILE_BUTTON_H));
    let list_button = menu_button(ui, menu, "layout-list", Icon::Menu, LIST);
    let font = crate::theme::ribbon_font(ui.ctx());
    let p = ui.painter().clone();
    let labels: Vec<_> = tabs.iter().enumerate().map(|(i, n)| elided(&p, &shown_name(i, n), font.clone(), t.text, TAB_TEXT)).collect();
    let widths: Vec<f32> = labels.iter().map(|(g, _)| g.size().x + FILE_TAB_SLANT + 2.0 * FILE_TAB_PAD).collect();
    // Model and "+" always show; the layouts share what is left.
    let mut x = menu.right() + 4.0;
    let room = limit - x - widths.first().copied().unwrap_or(0.0) - FILE_PLUS_W;
    let layouts = widths.get(1..).unwrap_or_default();
    let active_layout = active_tab.and_then(|a| a.checked_sub(1));
    let mut shown = visible_tabs(layouts, active_layout, room);
    let hidden = shown.len() < layouts.len();
    if hidden {
        shown = visible_tabs(layouts, active_layout, room - MORE_W);
    }
    let mut switch = None;
    let clip = Rect::from_min_max(pos2(row.left(), band_top), pos2(limit.max(row.left()), row.bottom()));
    let tp = p.with_clip_rect(clip);
    let mut prev_active = false;
    for (i, (name, (g, cut))) in tabs.iter().zip(labels).enumerate() {
        if i > 0 && !shown.contains(&(i - 1)) {
            continue;
        }
        let w = g.size().x + FILE_TAB_SLANT + 2.0 * FILE_TAB_PAD;
        let active = active_tab == Some(i);
        let resp = ui.interact(Rect::from_min_max(pos2(x, top), pos2(x + w, bottom)).intersect(clip), ui.id().with(("lt", i)), Sense::click());
        if active {
            // The current tab joins the band above it.
            tp.add(egui::Shape::convex_polygon(tab_shape(x, band_top, bottom, w), t.status_band, Stroke::NONE));
        } else {
            if !prev_active {
                tab_separator(&tp, x, top, bottom);
            }
            if resp.hovered() {
                tp.add(egui::Shape::convex_polygon(tab_shape(x, top, bottom, w), t.ribbon_hover, Stroke::NONE));
            }
        }
        let color = if active { t.text } else { t.text_dim };
        let text = pos2(x + FILE_TAB_SLANT / 2.0 + FILE_TAB_PAD, (top + bottom) / 2.0 - g.size().y / 2.0);
        // The current tab's name is bright, the others dimmed.
        tp.galley(text, g, color);
        icons::describe_control(ui, &resp, name, Some(active), true);
        let resp = if cut { resp.on_hover_text(shown_name(i, name)) } else { resp };
        if resp.clicked() {
            switch = Some(name.clone());
        }
        crate::shortcut_menus::attach(app, &resp, |app| crate::shortcut_menus::layout_tab_entries(app, name));
        prev_active = active;
        x += w;
    }
    // The layouts that didn't fit, behind a button after the last tab shown.
    let more = hidden.then(|| {
        let r = Rect::from_min_size(pos2(x + 2.0, row.center().y - 8.0), vec2(MORE_W - 4.0, 16.0));
        x += MORE_W;
        menu_button(ui, r, "layout-more", Icon::ChevronDown, MORE)
    });
    // "+": a new layout, then the bar's closing "/".
    let plus = Rect::from_min_size(pos2(x, row.center().y - FILE_BUTTON_H / 2.0), vec2(FILE_PLUS_W, FILE_BUTTON_H));
    let presp = ui.interact(plus, ui.id().with("layout+"), Sense::click());
    if !prev_active || more.is_some() {
        tab_separator(&p, x, top, bottom);
    }
    if presp.hovered() {
        p.rect_filled(plus.shrink2(vec2(FILE_TAB_SLANT, 0.0)), 2.0, t.ribbon_hover);
    }
    icons::paint(&p, Rect::from_center_size(plus.center(), vec2(16.0, 16.0)), Icon::Plus, false);
    tab_separator(&p, plus.right(), top, bottom);
    icons::describe_control(ui, &presp, NEW, None, false);
    if presp.on_hover_text(crate::tl!("New layout")).clicked()
        && let Ok(v) = app.run("layout.new", json!({}))
        && let Some(n) = v.get("name").and_then(serde_json::Value::as_str)
    {
        switch = Some(n.to_string());
    }
    for resp in std::iter::once(&list_button).chain(&more) {
        let mut chosen = None;
        egui::Popup::from_toggle_button_response(resp).width(200.0).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
            egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                for (i, name) in tabs.iter().enumerate() {
                    let r = ui.add(egui::Button::selectable(active_tab == Some(i), shown_name(i, name)).truncate());
                    crate::control::record_widget(ui, &r, &shown_name(i, name), "button", Some(active_tab == Some(i)));
                    if r.clicked() {
                        switch = Some(name.clone());
                        ui.close();
                    }
                }
            });
            // The layout commands, as on a tab's menu (New Layout, From Template...).
            let groups = crate::shortcut_menus::layout_tab_entries(app, "Model");
            if !groups.is_empty() {
                ui.separator();
                crate::context_menu::groups_ui(ui, &groups, &mut chosen);
            }
        });
        if let Some(id) = chosen {
            crate::shortcut_menus::choose(app, ui.ctx(), &id);
        }
    }
    if let Some(n) = switch {
        let _ = app.run("layout.set", json!({ "name": n }));
    }
    plus.right() + FILE_TAB_SLANT
}

/// A small icon button of the layout bar.
fn menu_button(ui: &mut egui::Ui, rect: Rect, id: &str, icon: Icon, label: &str) -> egui::Response {
    let t = Tokens::get();
    let resp = ui.interact(rect, ui.id().with(id), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
    let side = if icon == Icon::Menu { 18.0 } else { 14.0 };
    icons::paint(ui.painter(), Rect::from_center_size(rect.center(), vec2(side, side)), icon, false);
    icons::describe_control(ui, &resp, label, None, false);
    resp.on_hover_text(crate::i18n::t(label))
}
