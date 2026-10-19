//! AutoCAD's title row, 32 px tall: the Application button, the Quick Access Toolbar (`qat`),
//! the window title centred between them and the InfoCenter (search box, Search button, Help).
//!
//! The Application button opens the File menu's entries for now; AutoCAD's full Application
//! menu (recent documents, sub-panes, Options, Exit) comes in M1c. Autodesk's online controls
//! (Sign In, App Store, Stay Connected) are not built.

use egui::{Color32, Painter, Rect, Response, Sense, Stroke, Ui, Vec2, pos2, vec2};
use serde_json::Value;

use super::qat;
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

pub use super::qat::strip as qat_strip;

/// The row's height.
pub const HEIGHT: f32 = 32.0;
/// The Application button: the mark and its ▾.
const APP_BUTTON: Vec2 = Vec2::new(40.0, 24.0);
const MARK: f32 = 22.0;
/// The InfoCenter's search box width and its collapse ▸ toggle's.
const SEARCH_W: f32 = 180.0;
const COLLAPSE_W: f32 = 13.0;
/// The Help split's ▾ part.
const SPLIT_ARROW_W: f32 = 13.0;
const GAP: f32 = 4.0;
const MARGIN: f32 = 6.0;
/// Queries typed into the search box are at most this long.
const QUERY_MAX: usize = 256;

/// The AsenaCAD mark's colours (those of `assets/app-icon/asenacad.svg`), not theme colours:
/// the mark looks the same in every theme, like the app icon.
const MARK_BG: Color32 = Color32::from_rgb(0x0F, 0x2A, 0x44);
const MARK_FG: Color32 = Color32::from_rgb(0xF2, 0xF5, 0xF8);
const MARK_PICK: Color32 = Color32::from_rgb(0xFF, 0x6B, 0x35);

fn search_text_id() -> egui::Id {
    egui::Id::new("asenacad-infocenter-search")
}

fn search_hidden_id() -> egui::Id {
    egui::Id::new("asenacad-infocenter-search-hidden")
}

/// The title row as a top panel of `ui`.
pub fn show(app: &mut CadApp, ui: &mut Ui) {
    let t = Tokens::get();
    egui::Panel::top("asenacad_title_row").exact_size(HEIGHT).frame(egui::Frame::NONE.fill(t.ribbon_tab_row)).show(ui, |ui| {
        let r = ui.max_rect();
        if app.integrated_titlebar {
            // Under everything else: the buttons on top of it keep their clicks.
            super::window_drag(ui, r);
        }
        // With the macOS integrated title bar the traffic lights sit at the left end.
        let left = r.left() + if app.integrated_titlebar { super::MAC_CONTENT_LEFT } else { 4.0 };
        let center_y = r.top() + HEIGHT / 2.0;
        let app_rect = Rect::from_min_size(pos2(left, center_y - APP_BUTTON.y / 2.0), APP_BUTTON);
        application_button(app, ui, app_rect);
        let qat_right = if app.ui.qat_below_ribbon { app_rect.right() } else { qat::bar(app, ui, app_rect.right(), center_y) };
        let info_left = info_center(app, ui, r.right() - MARGIN, center_y);
        title(app, ui, r, qat_right + 2.0 * GAP, info_left - 2.0 * GAP);
    });
}

/// "AsenaCAD  <drawing>.dwg" as AutoCAD shows "Autodesk AutoCAD 2026  Drawing1.dwg".
pub fn title_text(app: &CadApp) -> String {
    match app.session.state() {
        Ok(st) if !app.ui.start_tab => format!("AsenaCAD  {}{}", st.title, if st.title.contains('.') { "" } else { ".dwg" }),
        _ => format!("AsenaCAD  {}", crate::tl!("Start")),
    }
}

/// The title centred in the row, slid or cut with "…" to stay between `min_x` and `max_x`.
fn title(app: &CadApp, ui: &mut Ui, row: Rect, min_x: f32, max_x: f32) {
    let t = Tokens::get();
    let room = max_x - min_x;
    if room < 24.0 {
        return;
    }
    let text = title_text(app);
    let (galley, _) = super::elided(ui.painter(), &text, crate::theme::ribbon_font(ui.ctx()), t.text, room);
    let size = galley.size();
    let x = (row.center().x - size.x / 2.0).clamp(min_x, (max_x - size.x).max(min_x));
    let at = pos2(x, row.center().y - size.y / 2.0);
    ui.painter().galley(at, galley, t.text);
    let resp = ui.interact(Rect::from_min_size(at, size), ui.id().with("title-text"), Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, &text));
    crate::control::record_widget(ui, &resp, &text, "label", None);
}

/// The AsenaCAD mark in `rect` (a square): a drafting "A" on a navy tile with an orange pick
/// box at its apex, after the app icon, simplified so it reads at 22 px.
fn paint_mark(p: &Painter, rect: Rect) {
    let s = rect.width() / 1024.0;
    let at = |x: f32, y: f32| pos2(rect.left() + x * s, rect.top() + y * s);
    p.rect_filled(rect, 184.0 * s, MARK_BG);
    let stroke = Stroke::new((80.0 * s).max(1.6), MARK_FG);
    p.add(egui::Shape::line(vec![at(232.0, 860.0), at(512.0, 250.0), at(792.0, 860.0)], stroke));
    p.line_segment([at(352.0, 620.0), at(672.0, 620.0)], stroke);
    let pick = Stroke::new((24.0 * s).max(1.1), MARK_PICK);
    let c = at(512.0, 220.0);
    let half = (60.0 * s).max(2.6);
    p.rect_stroke(Rect::from_center_size(c, Vec2::splat(2.0 * half)), 0.0, pick, egui::StrokeKind::Middle);
    let arm = (110.0 * s).max(4.5);
    p.line_segment([pos2(c.x, c.y - arm), pos2(c.x, c.y - half)], pick);
    p.line_segment([pos2(c.x - arm, c.y), pos2(c.x - half, c.y)], pick);
    p.line_segment([pos2(c.x + half, c.y), pos2(c.x + arm, c.y)], pick);
}

fn hover_fill(ui: &Ui, rect: Rect, resp: &Response) {
    let t = Tokens::get();
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_pressed);
    } else if resp.hovered() && resp.enabled() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
}

/// The Application button: the mark with a ▾, opening the File menu's entries.
fn application_button(app: &mut CadApp, ui: &mut Ui, rect: Rect) {
    let resp = ui.interact(rect, ui.id().with("application-button"), Sense::click());
    hover_fill(ui, rect, &resp);
    let mark = Rect::from_min_size(pos2(rect.left() + 2.0, rect.center().y - MARK / 2.0), Vec2::splat(MARK));
    paint_mark(ui.painter(), mark);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(mark.right() + 8.0, rect.center().y), Vec2::splat(9.0)), Icon::ChevronDown, false);
    let label = crate::tl!("Application Button");
    icons::describe_control(ui, &resp, label, None, false);
    let resp = resp.on_hover_text(label);
    let tree = crate::menus::tree(app);
    let Some((_, file)) = tree.iter().find(|(name, _)| name == "File") else { return };
    menu_popup(app, &resp, file);
}

/// A popup under `resp` with `entries` (labels, shortcuts, enabled states, submenus); choosing
/// one starts it like the menu bar does.
fn menu_popup(app: &mut CadApp, resp: &Response, entries: &[crate::menus::Entry]) {
    let mut clicked = None;
    egui::Popup::from_toggle_button_response(resp).show(|ui| {
        ui.set_min_width(220.0);
        for e in entries {
            crate::menus::entry_ui(ui, e, &mut clicked);
        }
    });
    if let Some(id) = clicked {
        crate::menus::activate(app, &id);
    }
}

/// A `w`×22 InfoCenter button ending at `right`; the caller paints its glyph.
fn small_button(ui: &mut Ui, right: f32, center_y: f32, w: f32, id: &str, label: &str) -> Response {
    let rect = Rect::from_min_size(pos2(right - w, center_y - qat::BUTTON.y / 2.0), vec2(w, qat::BUTTON.y));
    let resp = ui.interact(rect, ui.id().with(id), Sense::click());
    hover_fill(ui, rect, &resp);
    icons::describe_control(ui, &resp, label, None, false);
    resp
}

/// The InfoCenter laid out leftwards from `right`: Help split, Search button, search box and
/// its hide ▸ toggle. Returns its left edge.
fn info_center(app: &mut CadApp, ui: &mut Ui, right: f32, center_y: f32) -> f32 {
    let ctx = ui.ctx().clone();
    // Help: "?" opens the Command Reference (F1), ▾ lists the Help menu.
    let menu_label = crate::tf!("{label} options", label = crate::tl!("Help"));
    let arrow = small_button(ui, right, center_y, SPLIT_ARROW_W, "help-menu", &menu_label);
    icons::paint(ui.painter(), Rect::from_center_size(arrow.rect.center(), Vec2::splat(9.0)), Icon::ChevronDown, false);
    let arrow = arrow.on_hover_text(&menu_label);
    let face = small_button(ui, arrow.rect.left(), center_y, qat::BUTTON.x, "help", crate::tl!("Help"));
    icons::paint(ui.painter(), Rect::from_center_size(face.rect.center(), Vec2::splat(16.0)), Icon::Help, false);
    let face = face.on_hover_text(format!("{} (F1)", crate::tl!("Help")));
    if face.clicked() {
        crate::menus::activate(app, "ui.dialog.commands");
    }
    let tree = crate::menus::tree(app);
    if let Some((_, help)) = tree.iter().find(|(name, _)| name == "Help") {
        menu_popup(app, &arrow, help);
    }
    // Search.
    let hidden = ctx.data(|d| d.get_temp::<bool>(search_hidden_id())).unwrap_or(false);
    let mut text = ctx.data(|d| d.get_temp::<String>(search_text_id())).unwrap_or_default();
    let search = small_button(ui, face.rect.left() - 2.0 * GAP, center_y, qat::BUTTON.x, "search", crate::tl!("Search"));
    icons::paint(ui.painter(), Rect::from_center_size(search.rect.center(), Vec2::splat(16.0)), Icon::Search, false);
    let mut left = search.rect.left();
    let mut run = search.on_hover_text(crate::tl!("Search")).clicked();
    if !hidden {
        let rect = Rect::from_min_size(pos2(left - GAP - SEARCH_W, center_y - 10.0), vec2(SEARCH_W, 20.0));
        let t = Tokens::get();
        let hint = egui::RichText::new(crate::tl!("Type a keyword or phrase")).italics().color(t.text_faint);
        let edit = egui::TextEdit::singleline(&mut text)
            .id(egui::Id::new("asenacad-infocenter-search-box"))
            .hint_text(hint)
            .char_limit(QUERY_MAX)
            .font(crate::theme::ribbon_font(ui.ctx()))
            .background_color(t.field)
            .margin(vec2(4.0, 2.0))
            .desired_width(SEARCH_W);
        let resp = ui.put(rect, edit);
        let placeholder = crate::tl!("Type a keyword or phrase");
        ctx.accesskit_node_builder(resp.id, |node| node.set_label(placeholder));
        crate::control::record_widget(ui, &resp, placeholder, "textbox", None);
        run |= resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
        ctx.data_mut(|d| d.insert_temp(search_text_id(), text.clone()));
        left = rect.left();
    }
    let label = if hidden { crate::tl!("Show search") } else { crate::tl!("Hide search") };
    let toggle = small_button(ui, left - GAP, center_y, COLLAPSE_W, "search-toggle", label);
    let chevron = if hidden { Icon::ChevronLeft } else { Icon::ChevronRight };
    icons::paint(ui.painter(), Rect::from_center_size(toggle.rect.center(), Vec2::splat(9.0)), chevron, false);
    if toggle.on_hover_text(label).clicked() {
        ctx.data_mut(|d| d.insert_temp(search_hidden_id(), !hidden));
    }
    if run {
        search_for(app, &ctx, &text);
    }
    left - GAP - COLLAPSE_W
}

/// What the InfoCenter search does with `query`: a command name or alias starts that command
/// (AutoCAD searches its help; ours is the Command Reference); anything else opens the Command
/// Reference filtered by it.
pub fn search_for(app: &mut CadApp, ctx: &egui::Context, query: &str) {
    let query: String = query.trim().chars().take(QUERY_MAX).collect();
    if !query.is_empty() && !query.contains(char::is_whitespace) {
        let id = asenacad_engine::cmd::resolve_alias(query.trim_start_matches(['_', '.']));
        if asenacad_engine::find_command(&id).is_some() {
            crate::menus::activate(app, &id);
            return;
        }
    }
    ctx.data_mut(|d| d.insert_temp(crate::dialogs::command_filter_id(), query));
    // Errors are echoed to the command history by `run`.
    let _ = app.run("ui.dialog.commands", Value::Null);
}
