//! AutoCAD's Application menu: the 424×568 popup under the Application button.
//!
//! - **Search box** at the top right: typing lists the matching commands; a result starts its
//!   command. The search button is disabled until something is typed.
//! - **Left column**: New, Open, Save, Save As, Import, Export, Publish, Print, Drawing
//!   Utilities and Close, 44 px rows in AutoCAD's order with its two separators. An entry's main
//!   part runs its default command; the 20 px ▸ part (hovered or clicked) shows its sub-pane.
//! - **Right pane**: Recent Documents (the recent-files list) or Open Documents (the open
//!   drawings), switched by the two radio icons; a sub-pane or the search results cover it.
//! - **Bottom right**: Options and Exit AsenaCAD (the app's close path, so unsaved drawings ask).
//!
//! Escape or a click outside closes it. Up/Down move between rows, Right opens the highlighted
//! entry's sub-pane, Left goes back, Enter runs.

mod documents;
pub mod model;
mod pane;

use egui::{Color32, Id, Key, Pos2, Rect, Response, Sense, Stroke, Ui, Vec2, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;
use model::{Action, Sub, Top};

/// The menu's size, as AutoCAD's.
pub const SIZE: Vec2 = Vec2::new(424.0, 568.0);
/// A first-level row and a separator between rows.
pub const ROW_H: f32 = 44.0;
const SEPARATOR_H: f32 = 4.0;
/// The first-level list's top-left and width.
const LIST: Vec2 = Vec2::new(1.0, 75.0);
const LIST_W: f32 = 141.0;
/// The ▸ part of a first-level row.
pub const ARROW_W: f32 = 20.0;
/// The search box (the search button sits in its right end).
const SEARCH: Rect = Rect { min: Pos2::new(143.0, 9.0), max: Pos2::new(416.0, 29.0) };
const SEARCH_BUTTON_W: f32 = 18.0;
/// Queries are at most this long.
const QUERY_MAX: usize = 128;
/// The bottom buttons' row and their height.
const BOTTOM_Y: f32 = 534.0;
const BOTTOM_H: f32 = 24.0;
const BOTTOM_RIGHT: f32 = 420.0;

/// What the right side shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Pane {
    #[default]
    Documents,
    /// The sub-pane of first-level entry `n` (an index into [`model::tops`]).
    Sub(usize),
    /// The nested list of entry `m` of first-level entry `n`'s sub-pane.
    Nested(usize, usize),
}

/// The keyboard highlight.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Focus {
    Top(usize),
    /// A row of the sub-pane (or nested list) shown.
    Entry(usize),
    /// A search result.
    Result(usize),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Docs {
    #[default]
    Recent,
    Open,
}

/// How Recent Documents are ordered.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum Order {
    /// Most recently opened first.
    #[default]
    Ordered,
    /// Grouped by file type, most recent first in each group.
    Type,
}

/// How the document lists show their files.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(crate) enum View {
    #[default]
    List,
    Icons,
}

/// A drop-down of the document pane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Dropdown {
    Order,
    View,
}

/// The menu's state, kept in egui's memory. The document pane's choices outlive the menu.
#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub open: bool,
    pub pane: Pane,
    pub docs: Docs,
    pub order: Order,
    pub view: View,
    pub dropdown: Option<Dropdown>,
    pub focus: Option<Focus>,
    pub query: String,
    /// How far the sub-pane's list is scrolled, in points.
    pub scroll: f32,
    /// Set on the frame the menu opens, so the search box takes the keyboard.
    pub opened: bool,
}

impl State {
    /// The state after the menu closes: only the document pane's choices stay.
    fn closed(&self) -> State {
        State { docs: self.docs, order: self.order, view: self.view, ..State::default() }
    }

    fn show_pane(&mut self, pane: Pane) {
        if self.pane != pane {
            self.pane = pane;
            self.scroll = 0.0;
            self.dropdown = None;
        }
    }
}

/// What a click or key asked for; done after the menu is drawn (and closed).
#[derive(Clone, Debug)]
pub(crate) enum Act {
    Run(&'static str, Value),
    Options,
    Exit,
    OpenFile(String),
    Switch(usize),
}

fn state_id() -> Id {
    Id::new("asenacad-appmenu")
}

fn load(ctx: &egui::Context) -> State {
    ctx.data(|d| d.get_temp::<State>(state_id())).unwrap_or_default()
}

fn store(ctx: &egui::Context, st: State) {
    ctx.data_mut(|d| d.insert_temp(state_id(), st));
}

/// Whether the Application menu is open.
pub fn is_open(ctx: &egui::Context) -> bool {
    load(ctx).open
}

/// Every label the menu shows from its tables, for the translation check.
pub fn labels() -> Vec<&'static str> {
    model::labels()
}

/// The menu under the Application `button`: opened and closed by clicking it.
pub fn show(app: &mut CadApp, ui: &Ui, button: &Response) {
    let ctx = ui.ctx().clone();
    let mut st = load(&ctx);
    if button.clicked() {
        st = if st.open { st.closed() } else { State { open: true, opened: true, ..st.closed() } };
    }
    if !st.open {
        store(&ctx, st);
        return;
    }
    let origin = pos2(button.rect.left(), button.rect.bottom());
    let rect = Rect::from_min_size(origin, SIZE);
    let mut act = None;
    keyboard(app, &ctx, &mut st, &mut act);
    if !st.open {
        // Escape closed it.
        store(&ctx, st);
        return;
    }
    egui::Area::new(Id::new("asenacad-appmenu-area")).order(egui::Order::Foreground).fixed_pos(origin).constrain(false).show(&ctx, |ui| {
        ui.set_min_size(SIZE);
        body(app, ui, rect, &mut st, &mut act);
    });
    st.opened = false;
    // A press outside the menu (not on the Application button, which toggles it) closes it.
    let outside = ctx.input(|i| i.pointer.any_pressed() && i.pointer.press_origin().is_some_and(|p| !rect.contains(p) && !button.rect.contains(p)));
    if outside || act.is_some() {
        st = st.closed();
    }
    store(&ctx, st);
    if let Some(act) = act {
        perform(app, act);
    }
}

fn perform(app: &mut CadApp, act: Act) {
    match act {
        Act::Run(id, Value::Null) => crate::menus::activate(app, id),
        Act::Run(id, params) => {
            app.cmd.buffer.clear();
            // Errors are echoed to the command history by `run`.
            let _ = app.run(id, params);
        }
        Act::Options => crate::menus::activate(app, "options"),
        // The app's own close path: drawings with unsaved changes ask first.
        Act::Exit => {
            if !app.request_quit() {
                app.quit_requested = true;
            }
        }
        Act::OpenFile(path) => match app.session.docs.iter().position(|d| d.path.as_deref() == Some(path.as_str())) {
            Some(i) => perform(app, Act::Switch(i)),
            None => app.open_path(&path),
        },
        Act::Switch(i) => {
            app.ui.start_tab = false;
            let _ = app.run("document.switch", json!({ "index": i }));
        }
    }
}

/// The menu's contents in `rect`.
fn body(app: &mut CadApp, ui: &mut Ui, rect: Rect, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let o = rect.min;
    // The whole menu takes the pointer, so nothing under it is clicked through it.
    let back = ui.interact(rect, ui.id().with("appmenu-back"), Sense::click());
    back.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Other, true, crate::tl!("Application Menu")));
    let p = ui.painter();
    p.rect_filled(rect, 0.0, t.panel);
    let pane = Rect::from_min_max(o + vec2(144.0, 39.0), o + vec2(423.0, 527.0));
    p.rect_filled(pane, 0.0, t.ribbon_bg);
    p.rect_stroke(rect, 0.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    search_box(ui, o, st, act);
    radio(ui, o + vec2(91.0, 45.0), Icon::RecentDocs, crate::tl!("Recent Documents"), st, Docs::Recent);
    radio(ui, o + vec2(112.0, 45.0), Icon::OpenDocs, crate::tl!("Open Documents"), st, Docs::Open);
    first_level(app, ui, o, st, act);
    if !st.query.trim().is_empty() {
        pane::results(ui, o, st, act);
    } else {
        match st.pane {
            Pane::Documents => documents::show(app, ui, o, st, act),
            Pane::Sub(_) | Pane::Nested(..) => pane::show(app, ui, o, st, act),
        }
    }
    bottom_buttons(ui, o, act);
    documents::dropdown(ui, o, st);
}

/// AutoCAD's text: the interface font at its size.
fn font(ui: &Ui) -> egui::FontId {
    crate::theme::ribbon_font(ui.ctx())
}

/// Paint `text` from `at` (top-left), wrapped at `width`; `bold` sets it a shade heavier
/// (twice, half a point apart: the interface fonts have no bold face of their own). Returns its
/// size.
pub(crate) fn text(ui: &Ui, at: Pos2, text: &str, font: egui::FontId, color: Color32, width: f32, bold: bool) -> Vec2 {
    let mut job = egui::text::LayoutJob::simple(text.to_string(), font, color, width.max(1.0));
    job.halign = egui::Align::LEFT;
    let galley = ui.painter().layout_job(job);
    let size = galley.size();
    if bold {
        ui.painter().galley(at + vec2(0.5, 0.0), galley.clone(), color);
    }
    ui.painter().galley(at, galley, color);
    size
}

/// The lines `text` takes at `width`.
pub(crate) fn line_count(ui: &Ui, text: &str, font: egui::FontId, width: f32) -> usize {
    let job = egui::text::LayoutJob::simple(text.to_string(), font, Color32::WHITE, width.max(1.0));
    ui.painter().layout_job(job).rows.len().max(1)
}

/// A clickable rect, disabled when `enabled` is false; the hover and press fills are painted.
pub(crate) fn hit(ui: &mut Ui, rect: Rect, id: Id, enabled: bool, highlighted: bool) -> Response {
    let resp = ui.add_enabled_ui(enabled, |ui| ui.interact(rect, id, Sense::click())).inner;
    let t = Tokens::get();
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_pressed);
    } else if (resp.hovered() && enabled) || highlighted {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
    resp
}

/// The search box and its button; typing lists matching commands in place of the right pane.
fn search_box(ui: &mut Ui, o: Pos2, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let rect = SEARCH.translate(o.to_vec2());
    let placeholder = crate::tl!("Search Commands");
    let hint = egui::RichText::new(placeholder).italics().color(t.text_faint);
    let before = st.query.clone();
    let edit = egui::TextEdit::singleline(&mut st.query)
        .id(Id::new("asenacad-appmenu-search"))
        .hint_text(hint)
        .char_limit(QUERY_MAX)
        .font(font(ui))
        .background_color(t.field)
        .margin(vec2(4.0, 2.0))
        .desired_width(rect.width() - SEARCH_BUTTON_W);
    let resp = ui.put(rect, edit);
    ui.ctx().accesskit_node_builder(resp.id, |node| node.set_label(placeholder));
    crate::control::record_widget(ui, &resp, placeholder, "textbox", None);
    if st.opened {
        resp.request_focus();
    }
    if st.query != before {
        st.focus = None;
        st.dropdown = None;
    }
    let typed = !st.query.trim().is_empty();
    let button = Rect::from_min_size(pos2(rect.right() - SEARCH_BUTTON_W - 1.0, rect.top()), vec2(SEARCH_BUTTON_W, rect.height()));
    let b = hit(ui, button, ui.id().with("appmenu-search-button"), typed, false);
    icons::paint(ui.painter(), Rect::from_center_size(button.center(), Vec2::splat(14.0)), Icon::Search, !typed);
    icons::describe_control(ui, &b, crate::tl!("Search"), None, false);
    if b.clicked() {
        *act = pane::search(&st.query).first().map(|r| Act::Run(r.id, Value::Null));
    }
}

/// One of the two radio icons that switch the right pane between Recent and Open Documents.
fn radio(ui: &mut Ui, at: Pos2, icon: Icon, label: &str, st: &mut State, docs: Docs) {
    let t = Tokens::get();
    let rect = Rect::from_min_size(at, Vec2::splat(22.0));
    let on = st.docs == docs;
    let resp = hit(ui, rect, ui.id().with(("appmenu-radio", label)), true, false);
    if on {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_pressed);
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.accent), egui::StrokeKind::Inside);
    }
    icons::paint(ui.painter(), rect.shrink(3.0), icon, false);
    icons::describe_control(ui, &resp, label, Some(on), false);
    let resp = resp.on_hover_text(label);
    if resp.clicked() {
        st.docs = docs;
        st.query.clear();
        st.show_pane(Pane::Documents);
    }
}

/// The y offset (from the menu's top) of each first-level row, separators skipped.
pub(crate) fn row_tops() -> Vec<f32> {
    let mut y = LIST.y;
    let mut out = Vec::new();
    for row in model::ROWS {
        match row {
            model::Row::Top(_) => {
                out.push(y);
                y += ROW_H;
            }
            model::Row::Separator => y += SEPARATOR_H,
        }
    }
    out
}

/// Whether first-level entry `top`'s default command may run now.
fn default_ready(app: &CadApp, ctx: &egui::Context, top: &Top) -> bool {
    top.default.is_none_or(|id| crate::ribbon::avail::availability(ctx, &app.session, id).is_ready())
}

/// The first-level list: separators, then each entry's main part and ▸.
fn first_level(app: &CadApp, ui: &mut Ui, o: Pos2, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let mut y = LIST.y;
    for row in model::ROWS {
        if let model::Row::Separator = row {
            let mid = o.y + y + SEPARATOR_H / 2.0;
            ui.painter().line_segment([pos2(o.x + LIST.x + 10.0, mid), pos2(o.x + LIST.x + LIST_W, mid)], Stroke::new(1.0, t.border));
            y += SEPARATOR_H;
        }
    }
    for (i, (top, y)) in model::tops().zip(row_tops()).enumerate() {
        let row = Rect::from_min_size(o + vec2(LIST.x, y), vec2(LIST_W, ROW_H));
        let has_sub = !top.sub.is_empty();
        let shown = matches!(st.pane, Pane::Sub(n) | Pane::Nested(n, _) if n == i) && st.query.trim().is_empty();
        let main_w = if has_sub { LIST_W - 4.0 - ARROW_W } else { LIST_W - 4.0 };
        let main_rect = Rect::from_min_size(row.min + vec2(3.0, 1.0), vec2(main_w, ROW_H - 2.0));
        let focused = st.focus == Some(Focus::Top(i));
        if shown {
            // The entry whose sub-pane is shown stays highlighted, as in AutoCAD.
            ui.painter().rect_filled(row.shrink2(vec2(0.0, 1.0)), 2.0, t.ribbon_hover);
        }
        let main = hit(ui, main_rect, ui.id().with(("appmenu-top", i)), default_ready(app, ui.ctx(), top), focused);
        let dim = !main.enabled();
        icons::paint(ui.painter(), Rect::from_min_size(row.min + vec2(11.0, 6.0), Vec2::splat(32.0)), top.icon, dim);
        let label = crate::i18n::t(top.label);
        let width = main_w - 50.0;
        let f = font(ui);
        let lines = line_count(ui, label, f.clone(), width);
        let line_h = ui.fonts_mut(|fonts| fonts.row_height(&f));
        let ty = row.center().y - lines as f32 * line_h / 2.0;
        text(ui, pos2(row.left() + 51.0, ty), label, font(ui), if dim { t.text_faint } else { t.text }, width, false);
        icons::describe_control(ui, &main, top.label, None, false);
        if main.clicked() {
            match top.default {
                Some(id) => *act = Some(Act::Run(id, Value::Null)),
                None => open_sub(st, i),
            }
        }
        if main.hovered() && !has_sub && !main.is_pointer_button_down_on() {
            // An entry without a sub-pane brings the documents back.
            st.show_pane(Pane::Documents);
        }
        if has_sub {
            let arrow_rect = Rect::from_min_size(pos2(main_rect.right(), main_rect.top()), vec2(ARROW_W, ROW_H - 2.0));
            let arrow = hit(ui, arrow_rect, ui.id().with(("appmenu-arrow", i)), true, false);
            icons::paint(ui.painter(), Rect::from_center_size(arrow_rect.center(), Vec2::splat(9.0)), Icon::ChevronRight, false);
            let name = crate::tf!("{label} submenu", label = label);
            icons::describe_control(ui, &arrow, &name, None, false);
            if arrow.hovered() || arrow.clicked() {
                if !matches!(st.pane, Pane::Nested(n, _) if n == i) {
                    st.show_pane(Pane::Sub(i));
                }
                if !st.query.is_empty() && arrow.clicked() {
                    st.query.clear();
                }
            }
        }
    }
}

fn open_sub(st: &mut State, i: usize) {
    st.query.clear();
    st.show_pane(Pane::Sub(i));
}

/// Options and Exit AsenaCAD, right-aligned at the bottom.
fn bottom_buttons(ui: &mut Ui, o: Pos2, act: &mut Option<Act>) {
    let t = Tokens::get();
    let mut right = o.x + BOTTOM_RIGHT;
    for (label, which) in [(crate::tl!("Exit AsenaCAD"), Act::Exit), (crate::tl!("Options"), Act::Options)] {
        let f = font(ui);
        let w = ui.fonts_mut(|fonts| fonts.layout_no_wrap(label.to_string(), f, t.text).size().x) + 12.0;
        let rect = Rect::from_min_size(pos2(right - w, o.y + BOTTOM_Y), vec2(w, BOTTOM_H));
        let resp = hit(ui, rect, ui.id().with(("appmenu-bottom", label)), true, false);
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, label, font(ui), t.text);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.clicked() {
            *act = Some(which);
        }
        right = rect.left() - 4.0;
    }
}

/// The sub-pane's rows (separators as `None`) of the pane shown.
pub(crate) fn pane_entries(pane: Pane) -> &'static [Sub] {
    let tops: Vec<&'static Top> = model::tops().collect();
    match pane {
        Pane::Documents => &[],
        Pane::Sub(n) => tops.get(n).map_or(&[], |t| t.sub),
        Pane::Nested(n, m) => match tops.get(n).and_then(|t| t.sub.get(m)) {
            Some(Sub::Entry { action: Action::Nested { entries, .. }, .. }) => entries,
            _ => &[],
        },
    }
}

/// Up/Down/Left/Right/Enter and Escape, read before the widgets so the search box doesn't take
/// the arrows (it keeps Left and Right while it holds text).
fn keyboard(app: &CadApp, ctx: &egui::Context, st: &mut State, act: &mut Option<Act>) {
    use egui::Modifiers;
    let take = |key: Key| ctx.input_mut(|i| i.consume_key(Modifiers::NONE, key));
    if take(Key::Escape) {
        *st = State { open: false, ..st.closed() };
        return;
    }
    if !st.query.trim().is_empty() {
        let n = pane::search(&st.query).len();
        let cur = match st.focus {
            Some(Focus::Result(k)) => Some(k),
            _ => None,
        };
        if n > 0 && take(Key::ArrowDown) {
            st.focus = Some(Focus::Result(cur.map_or(0, |k| (k + 1) % n)));
        }
        if n > 0 && take(Key::ArrowUp) {
            st.focus = Some(Focus::Result(cur.map_or(n - 1, |k| (k + n - 1) % n)));
        }
        if take(Key::Enter) {
            *act = pane::search(&st.query).get(cur.unwrap_or(0)).map(|r| Act::Run(r.id, Value::Null));
        }
        return;
    }
    let tops: Vec<&'static Top> = model::tops().collect();
    let n = tops.len();
    match st.focus {
        Some(Focus::Entry(k)) => {
            let entries = pane_entries(st.pane);
            let rows: Vec<usize> = entries.iter().enumerate().filter(|(_, e)| matches!(e, Sub::Entry { .. })).map(|(i, _)| i).collect();
            let at = rows.iter().position(|r| *r == k).unwrap_or(0);
            if !rows.is_empty() && take(Key::ArrowDown) {
                st.focus = Some(Focus::Entry(rows[(at + 1) % rows.len()]));
            }
            if !rows.is_empty() && take(Key::ArrowUp) {
                st.focus = Some(Focus::Entry(rows[(at + rows.len() - 1) % rows.len()]));
            }
            if take(Key::ArrowLeft) {
                st.focus = match st.pane {
                    Pane::Nested(t, m) => {
                        st.show_pane(Pane::Sub(t));
                        Some(Focus::Entry(m))
                    }
                    Pane::Sub(t) => Some(Focus::Top(t)),
                    Pane::Documents => None,
                };
            }
            let right = take(Key::ArrowRight);
            let enter = take(Key::Enter);
            if let (Some(Sub::Entry { action, .. }), Pane::Sub(t)) = (entries.get(k), st.pane)
                && let Action::Nested { default, .. } = action
                && (right || (enter && default.is_none()))
            {
                st.show_pane(Pane::Nested(t, k));
                st.focus = Some(Focus::Entry(0));
                return;
            }
            if enter {
                *act = pane::entry_act(app, ctx, st.pane, k);
            }
        }
        _ => {
            let cur = match st.focus {
                Some(Focus::Top(i)) => Some(i),
                _ => None,
            };
            if take(Key::ArrowDown) {
                st.focus = Some(Focus::Top(cur.map_or(0, |i| (i + 1) % n)));
            }
            if take(Key::ArrowUp) {
                st.focus = Some(Focus::Top(cur.map_or(n - 1, |i| (i + n - 1) % n)));
            }
            let Some(i) = cur else { return };
            let Some(top) = tops.get(i) else { return };
            if take(Key::ArrowRight) && !top.sub.is_empty() {
                open_sub(st, i);
                st.focus = top.sub.iter().position(|e| matches!(e, Sub::Entry { .. })).map(Focus::Entry);
            }
            if take(Key::Enter) {
                match top.default {
                    Some(id) if default_ready(app, ctx, top) => *act = Some(Act::Run(id, Value::Null)),
                    Some(_) => {}
                    None => {
                        open_sub(st, i);
                        st.focus = Some(Focus::Entry(0));
                    }
                }
            }
        }
    }
}
