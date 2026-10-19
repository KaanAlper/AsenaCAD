//! The Application menu's sub-panes (a framed panel over the right pane: a heading line, then
//! entries with a 32 px icon, a bold label and our own description) and its search results.

use egui::{Pos2, Rect, Stroke, Ui, Vec2, pos2, vec2};
use serde_json::Value;

use super::model::{self, Action, Sub};
use super::{Act, Focus, Pane, State, hit, line_count, text};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::ribbon::avail::Availability;
use crate::theme::Tokens;

/// The sub-pane's frame and its list, relative to the menu's top-left.
const FRAME: Rect = Rect { min: Pos2::new(136.0, 42.0), max: Pos2::new(415.0, 524.0) };
const LIST: Rect = Rect { min: Pos2::new(140.0, 82.0), max: Pos2::new(411.0, 518.0) };
/// An entry's icon and text columns, from the list's left.
const ICON_X: f32 = 13.0;
const TEXT_X: f32 = 55.0;
const TEXT_W: f32 = 207.0;
/// The shortest entry; each line of label and description adds this.
const ENTRY_MIN_H: f32 = 52.0;
const LINE_H: f32 = 13.0;
const SEPARATOR_H: f32 = 4.0;
/// The scroll arrows shown when the entries don't fit.
const SCROLL_H: f32 = 10.0;
/// A search result row.
const RESULT_H: f32 = 26.0;
/// At most this many results are listed.
const RESULTS_MAX: usize = 16;

fn description_font() -> egui::FontId {
    egui::FontId::proportional(11.0)
}

/// The height of a sub-pane entry whose description takes `lines` lines.
pub(crate) fn entry_height(lines: usize) -> f32 {
    (20.0 + LINE_H * (1 + lines) as f32).max(ENTRY_MIN_H)
}

/// The sub-pane's heading line and rows.
fn heading_and_rows(pane: Pane) -> (&'static str, &'static [Sub]) {
    let tops: Vec<&'static model::Top> = model::tops().collect();
    let heading = match pane {
        Pane::Sub(n) => tops.get(n).map_or("", |t| t.heading),
        Pane::Nested(n, m) => match tops.get(n).and_then(|t| t.sub.get(m)) {
            Some(Sub::Entry { description, .. }) => description,
            _ => "",
        },
        Pane::Documents => "",
    };
    (heading, super::pane_entries(pane))
}

/// Whether the entry's command exists and may run now (online entries never may).
fn availability(app: &CadApp, ctx: &egui::Context, action: &Action) -> Availability {
    match action {
        Action::Command(id, _) => {
            let a = crate::ribbon::avail::availability(ctx, &app.session, id);
            // Save Layout as a Drawing works on a layout only, as in AutoCAD.
            if *id == model::SAVE_LAYOUT && a.is_ready() && matches!(app.session.layout_space(), asenacad_doc::Space::Model) {
                return Availability::Refused("Not available in model space".into());
            }
            a
        }
        Action::Nested { .. } => Availability::Ready,
        Action::Online => Availability::Missing,
    }
}

/// What choosing row `k` of `pane` does (the nested list's own ▸ is handled by the caller).
pub(crate) fn entry_act(app: &CadApp, ctx: &egui::Context, pane: Pane, k: usize) -> Option<Act> {
    let Some(Sub::Entry { action, .. }) = super::pane_entries(pane).get(k) else { return None };
    if !availability(app, ctx, action).is_ready() {
        return None;
    }
    match action {
        Action::Command(id, params) => Some(Act::Run(id, model::params(*params))),
        Action::Nested { default: Some(id), .. } => Some(Act::Run(id, Value::Null)),
        Action::Nested { default: None, .. } | Action::Online => None,
    }
}

/// The sub-pane of `st.pane`.
pub(super) fn show(app: &CadApp, ui: &mut Ui, o: Pos2, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let frame = FRAME.translate(o.to_vec2());
    let list = LIST.translate(o.to_vec2());
    let p = ui.painter();
    p.rect_filled(frame, 0.0, t.panel);
    p.rect_stroke(frame, 0.0, Stroke::new(2.0, t.chrome_dark), egui::StrokeKind::Inside);
    let (heading, rows) = heading_and_rows(st.pane);
    let mut x = frame.left() + 14.0;
    if let Pane::Nested(n, _) = st.pane {
        // Back to the sub-pane the nested list came from.
        let back = Rect::from_min_size(pos2(frame.left() + 6.0, frame.top() + 10.0), vec2(16.0, 20.0));
        let resp = hit(ui, back, ui.id().with("appmenu-back-arrow"), true, false);
        icons::paint(ui.painter(), Rect::from_center_size(back.center(), Vec2::splat(9.0)), Icon::ChevronLeft, false);
        icons::describe_control(ui, &resp, crate::tl!("Back"), None, false);
        if resp.clicked() {
            st.show_pane(Pane::Sub(n));
        }
        x = back.right() + 4.0;
    }
    let width = frame.right() - 8.0 - x;
    text(ui, pos2(x, frame.top() + 12.0), crate::i18n::t(heading), super::font(ui), t.text, width, true);
    let rule = frame.top() + 34.0;
    ui.painter().line_segment([pos2(frame.left() + 8.0, rule), pos2(frame.right() - 8.0, rule)], Stroke::new(1.0, t.border));

    // Row heights, then the scroll offset that keeps the highlighted row in view.
    let heights: Vec<f32> = rows
        .iter()
        .map(|r| match r {
            Sub::Entry { description, .. } => entry_height(line_count(ui, crate::i18n::t(description), description_font(), TEXT_W)),
            Sub::Separator => SEPARATOR_H,
        })
        .collect();
    let total: f32 = heights.iter().sum();
    let overflow = (total - list.height()).max(0.0);
    if ui.rect_contains_pointer(list) && overflow > 0.0 {
        st.scroll -= ui.input(|i| i.smooth_scroll_delta.y);
    }
    if let Some(Focus::Entry(k)) = st.focus {
        let top: f32 = heights.iter().take(k).sum();
        let h = heights.get(k).copied().unwrap_or(0.0);
        st.scroll = st.scroll.min(top).max(top + h - list.height());
    }
    st.scroll = st.scroll.clamp(0.0, overflow);

    let clip = ui.painter().clip_rect().intersect(list);
    let mut y = list.top() - st.scroll;
    for (k, (row, h)) in rows.iter().zip(&heights).enumerate() {
        let rect = Rect::from_min_size(pos2(list.left(), y), vec2(list.width(), *h));
        y += h;
        if !rect.intersects(list) {
            continue;
        }
        match row {
            Sub::Separator => {
                ui.painter()
                    .with_clip_rect(clip)
                    .line_segment([pos2(rect.left() + 10.0, rect.center().y), pos2(rect.right(), rect.center().y)], Stroke::new(1.0, t.border));
            }
            Sub::Entry { icon, label, description, action } => {
                if let Some(a) = entry(app, ui, st, rect, clip, k, (*icon, label, description, action)) {
                    *act = Some(a);
                }
            }
        }
    }
    scroll_arrows(ui, list, overflow, st);
}

/// One sub-pane entry at `rect` (clipped to `clip`); returns what clicking it asks for.
fn entry(app: &CadApp, ui: &mut Ui, st: &mut State, rect: Rect, clip: Rect, k: usize, row: (Icon, &str, &str, &Action)) -> Option<Act> {
    let t = Tokens::get();
    let (icon, label, description, action) = row;
    let availability = availability(app, ui.ctx(), action);
    let enabled = availability.is_ready();
    let nested = matches!(action, Action::Nested { .. });
    let main_w = if nested { rect.width() - 23.0 } else { rect.width() - 4.0 };
    let main_rect = Rect::from_min_size(rect.min + vec2(3.0, 1.0), vec2(main_w, rect.height() - 2.0)).intersect(clip);
    let focused = st.focus == Some(Focus::Entry(k));
    let resp = ui.scope(|ui| {
        ui.set_clip_rect(clip);
        hit(ui, main_rect, ui.id().with(("appmenu-entry", k, label)), enabled, focused)
    });
    let resp = resp.inner;
    let p = ui.painter().with_clip_rect(clip);
    let icon_rect = Rect::from_min_size(pos2(rect.left() + ICON_X, rect.center().y - 16.0), Vec2::splat(32.0));
    icons::paint(&p, icon_rect, icon, !enabled);
    let (fg, dim) = if enabled { (t.text, t.text_dim) } else { (t.text_faint, t.text_faint) };
    let label_t = crate::i18n::t(label);
    let width = if nested { TEXT_W - 19.0 } else { TEXT_W };
    let top = rect.top() + 10.0;
    ui.scope(|ui| {
        ui.set_clip_rect(clip);
        let size = text(ui, pos2(rect.left() + TEXT_X, top), label_t, super::font(ui), fg, width, true);
        text(ui, pos2(rect.left() + TEXT_X, top + size.y.max(LINE_H)), crate::i18n::t(description), description_font(), dim, width, false);
    });
    icons::describe_control(ui, &resp, label, None, false);
    let resp = if enabled { resp } else { resp.on_disabled_hover_text(availability.disabled_text()) };
    let mut out = None;
    if resp.clicked() {
        match action {
            Action::Nested { default: None, .. } => open_nested(st, k),
            _ => out = entry_act(app, ui.ctx(), st.pane, k),
        }
    }
    if nested {
        let arrow_rect = Rect::from_min_size(pos2(main_rect.right(), rect.top() + 1.0), vec2(19.0, rect.height() - 2.0)).intersect(clip);
        let arrow = hit(ui, arrow_rect, ui.id().with(("appmenu-nested-arrow", k, label)), true, false);
        icons::paint(&p, Rect::from_center_size(arrow_rect.center(), Vec2::splat(9.0)), Icon::ChevronRight, false);
        let name = crate::tf!("{label} submenu", label = label_t);
        icons::describe_control(ui, &arrow, &name, None, false);
        if arrow.clicked() || arrow.hovered() {
            open_nested(st, k);
        }
    }
    out
}

fn open_nested(st: &mut State, k: usize) {
    if let Pane::Sub(n) = st.pane {
        st.show_pane(Pane::Nested(n, k));
        if st.focus.is_some() {
            st.focus = Some(Focus::Entry(0));
        }
    }
}

/// The ▴ and ▾ that scroll an overflowing list by one line's worth.
fn scroll_arrows(ui: &mut Ui, list: Rect, overflow: f32, st: &mut State) {
    if overflow <= 0.0 {
        return;
    }
    let step = 3.0 * LINE_H;
    for (down, y) in [(false, list.top()), (true, list.bottom() - SCROLL_H)] {
        let shown = if down { st.scroll < overflow } else { st.scroll > 0.0 };
        if !shown {
            continue;
        }
        let rect = Rect::from_min_size(pos2(list.left(), y), vec2(list.width(), SCROLL_H));
        let t = Tokens::get();
        let label = if down { crate::tl!("Scroll down") } else { crate::tl!("Scroll up") };
        let resp = hit(ui, rect, ui.id().with(("appmenu-scroll", down)), true, false);
        ui.painter().rect_filled(rect, 0.0, t.panel);
        let icon = if down { Icon::ChevronDown } else { crate::icons::Icon::ChevronUp };
        icons::paint(ui.painter(), Rect::from_center_size(rect.center(), Vec2::splat(8.0)), icon, false);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.clicked() || resp.hovered() {
            let delta = if resp.clicked() { step } else { 2.0 };
            st.scroll = (st.scroll + if down { delta } else { -delta }).clamp(0.0, overflow);
            ui.ctx().request_repaint();
        }
    }
}

/// A command the search found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Found {
    pub id: &'static str,
    pub label: &'static str,
}

/// The commands matching `query`, best first: the command (or alias) of that name, then names
/// starting with it, then labels with a word starting with it, then labels containing it.
/// Placeholder commands AsenaCAD only names are left out, and so are the command-line forms
/// (`-PLOTTERMANAGER`) unless the query starts with a dash.
pub(crate) fn search(query: &str) -> Vec<Found> {
    let q: String = query.trim().chars().take(super::QUERY_MAX).collect::<String>().to_lowercase();
    if q.is_empty() {
        return Vec::new();
    }
    let exact = asenacad_engine::cmd::resolve_alias(q.trim_start_matches(['_', '.']));
    let mut ranked: Vec<(u8, &'static str, &'static str)> = asenacad_engine::command_specs()
        .iter()
        // Command-line forms (-PLOT…) only when asked for by name.
        .filter(|c| !c.is_stub() && !c.id.contains('.') && !c.label.is_empty() && (!c.id.starts_with('-') || q.starts_with('-')))
        .filter_map(|c| {
            let label = c.label.to_lowercase();
            let rank = if c.id == exact {
                0
            } else if c.id.starts_with(&q) {
                1
            } else if label.split(|ch: char| !ch.is_alphanumeric()).any(|w| w.starts_with(&q)) {
                2
            } else if label.contains(&q) {
                3
            } else {
                return None;
            };
            Some((rank, c.id, c.label))
        })
        .collect();
    ranked.sort_by(|a, b| a.0.cmp(&b.0).then(a.2.len().cmp(&b.2.len())).then(a.1.cmp(b.1)));
    let mut out: Vec<Found> = Vec::new();
    for (_, id, label) in ranked {
        if !out.iter().any(|f| f.label == label) {
            out.push(Found { id, label });
        }
        if out.len() == RESULTS_MAX {
            break;
        }
    }
    out
}

/// The search results in place of the right pane.
pub(super) fn results(ui: &mut Ui, o: Pos2, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let frame = FRAME.translate(o.to_vec2());
    ui.painter().rect_filled(frame, 0.0, t.panel);
    ui.painter().rect_stroke(frame, 0.0, Stroke::new(2.0, t.chrome_dark), egui::StrokeKind::Inside);
    let found = search(&st.query);
    let heading = if found.is_empty() { crate::tl!("No commands found") } else { crate::tl!("Commands") };
    text(ui, pos2(frame.left() + 14.0, frame.top() + 12.0), heading, super::font(ui), t.text, frame.width() - 28.0, true);
    let rule = frame.top() + 34.0;
    ui.painter().line_segment([pos2(frame.left() + 8.0, rule), pos2(frame.right() - 8.0, rule)], Stroke::new(1.0, t.border));
    let mut y = rule + 4.0;
    for (k, f) in found.iter().enumerate() {
        let rect = Rect::from_min_size(pos2(frame.left() + 4.0, y), vec2(frame.width() - 8.0, RESULT_H));
        y += RESULT_H;
        if rect.bottom() > frame.bottom() - 4.0 {
            break;
        }
        let focused = st.focus == Some(Focus::Result(k));
        let resp = hit(ui, rect, ui.id().with(("appmenu-result", f.id)), true, focused);
        let label = crate::i18n::t(f.label);
        let name = f.id.to_ascii_uppercase();
        let name_w = ui.fonts_mut(|fo| fo.layout_no_wrap(name.clone(), description_font(), t.text_dim).size().x);
        text(ui, pos2(rect.left() + 10.0, rect.center().y - 7.0), label, super::font(ui), t.text, rect.width() - name_w - 30.0, true);
        ui.painter().text(pos2(rect.right() - 8.0, rect.center().y), egui::Align2::RIGHT_CENTER, &name, description_font(), t.text_dim);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.clicked() {
            *act = Some(Act::Run(f.id, Value::Null));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_ranks_the_named_command_first_and_skips_placeholders() {
        let found = search("circle");
        assert_eq!(found.first().map(|f| f.id), Some("circle"));
        assert_eq!(search(" C ").first().map(|f| f.id), Some("circle"), "an alias finds its command");
        assert!(search("plot").iter().any(|f| f.id == "plot"));
        assert!(search("").is_empty() && search("   ").is_empty());
        assert!(!search("3dorbit").iter().any(|f| f.id == "3dorbit"), "a placeholder isn't offered");
        assert!(search("zzzzqqq").is_empty());
        assert!(search(&"x".repeat(10_000)).len() <= RESULTS_MAX);
    }

    #[test]
    fn entry_heights_grow_by_a_line() {
        assert_eq!(entry_height(1), ENTRY_MIN_H);
        assert_eq!(entry_height(2), 59.0);
        assert_eq!(entry_height(3), 72.0);
    }
}
