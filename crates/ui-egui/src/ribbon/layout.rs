//! Geometry of the ribbon: the tab row, the panel strip, where each item of a panel goes, each
//! panel's natural width, and which panels shrink or collapse when the window is too narrow.
//!
//! Numbers follow the observed AutoCAD ribbon: a 100 px strip of 97 px panels (72 px content and
//! an 18 px title bar), large items at the content's top, small items in rows at +5/+29/+53,
//! panels packed left to right with 1 px dividers. A narrow window first hides the labels of
//! small items (panel by panel from the right), then collapses whole panels from the right into
//! panel buttons that open the panel as a popup.

use egui::{Pos2, Rect, Ui, pos2, vec2};

use super::model::{Item, Panel, Size};
use super::widgets;

/// The tab row: 20 px tab buttons with 2 px above and below.
pub const TAB_ROW_H: f32 = 24.0;
pub const TAB_H: f32 = 20.0;
/// Horizontal padding inside a tab button around its label.
pub const TAB_PAD: f32 = 10.0;
/// The panel strip below the tab row (Full state).
pub const STRIP_H: f32 = 100.0;
/// A panel: content, a gap, the title bar.
pub const PANEL_H: f32 = 97.0;
pub const PANEL_TOP: f32 = 1.0;
pub const TITLE_H: f32 = 18.0;
/// Where a large item's top is, from the panel's top.
const LARGE_TOP: f32 = 3.0;
/// Where small row 1 is, from the panel's (or slide-out's) top; later rows follow every 24 px.
const SMALL_TOP: f32 = 5.0;
pub const ROW_STEP: f32 = 24.0;
/// The thin line between panels.
pub const DIVIDER: f32 = 1.0;
/// Space at a panel's left and right edges and between its columns and row items.
pub const PAD: f32 = 4.0;
const GAP: f32 = 2.0;
/// A collapsed panel's button.
pub const PANEL_BUTTON_W: f32 = 60.0;
/// The strip in the "Minimize to Panel Buttons" and "Minimize to Panel Titles" states.
pub const BUTTONS_STRIP_H: f32 = 36.0;
pub const TITLES_STRIP_H: f32 = 24.0;
/// The slide-out's bottom bar (pin button).
pub const SLIDEOUT_BAR_H: f32 = 20.0;

/// How a panel is shown in the strip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fit {
    Full,
    /// Small items without their labels.
    Compact,
    /// A panel button opening the panel as a popup.
    Collapsed,
}

/// Which panels collapse when panels of `widths` (each including its divider) must fit in
/// `available`: from the right, one at a time, until the rest plus the panel buttons fit.
/// A NaN or infinite `available` never collapses anything.
pub fn collapse(widths: &[f32], available: f32) -> Vec<bool> {
    let collapsed = PANEL_BUTTON_W + DIVIDER;
    let mut total: f32 = widths.iter().sum();
    let mut out = vec![false; widths.len()];
    for (flag, w) in out.iter_mut().zip(widths).rev() {
        if total <= available || available.is_nan() {
            break;
        }
        total += collapsed - w;
        *flag = true;
    }
    out
}

/// How every panel is shown: all full when they fit; otherwise panels go compact from the
/// right until they fit; if all compact still don't fit, panels collapse from the right.
/// `full` and `compact` are the panels' widths in both forms (each including its divider).
pub fn fit(full: &[f32], compact: &[f32], available: f32) -> Vec<Fit> {
    let mut widths = full.to_vec();
    let mut out = vec![Fit::Full; full.len()];
    for i in (0..widths.len()).rev() {
        if widths.iter().sum::<f32>() <= available || available.is_nan() {
            return out;
        }
        if let (Some(w), Some(c), Some(f)) = (widths.get_mut(i), compact.get(i), out.get_mut(i)) {
            *w = c.min(*w);
            *f = Fit::Compact;
        }
    }
    for (f, collapsed) in out.iter_mut().zip(collapse(&widths, available)) {
        if collapsed {
            *f = Fit::Collapsed;
        }
    }
    out
}

/// Small items' row (1-based), or None for a large item.
pub fn small_row(item: &Item) -> Option<u8> {
    match *item {
        Item::Button { size: Size::Large, .. } | Item::Toggle { size: Size::Large, .. } | Item::Split { size: Size::Large, .. } => None,
        Item::Button { row, .. } | Item::Toggle { row, .. } | Item::Split { row, .. } | Item::Combo { row, .. } | Item::Slider { row, .. } => {
            Some(row)
        }
    }
}

/// The model's columns: a large item is a column of its own; consecutive small items whose row
/// never decreases form a block of rows. Each entry is (index in `list`, row).
fn columns(list: &[Item]) -> Vec<Vec<(usize, Option<u8>)>> {
    let mut blocks: Vec<Vec<(usize, Option<u8>)>> = Vec::new();
    for (i, it) in list.iter().enumerate() {
        let row = small_row(it);
        let starts_block = match (row, blocks.last().and_then(|b| b.last())) {
            (None, _) | (_, None) | (_, Some((_, None))) => true,
            (Some(r), Some((_, Some(prev)))) => r < *prev,
        };
        if starts_block {
            blocks.push(Vec::new());
        }
        if let Some(b) = blocks.last_mut() {
            b.push((i, row));
        }
    }
    blocks
}

/// The key of item `i` of `list`: "<prefix>/<i>". `prefix` is "<tab>/<panel>" for a panel's
/// items and "<tab>/<panel>/slideout" for its slide-out, wherever the panel is shown.
pub fn item_key(prefix: &str, i: usize) -> String {
    format!("{prefix}/{i}")
}

/// Rectangles for the items of `list` laid out from `origin` (the panel's or slide-out's
/// top-left corner, before the left padding), and the width they take.
pub fn place(ui: &Ui, prefix: &str, list: &[Item], compact: bool, origin: Pos2) -> (Vec<(usize, Rect)>, f32) {
    let mut out = Vec::new();
    let mut x = origin.x;
    for (n, column) in columns(list).iter().enumerate() {
        if n > 0 {
            x += GAP;
        }
        let mut column_w: f32 = 0.0;
        let rows = column.iter().filter_map(|(_, r)| *r).max().unwrap_or(0);
        if rows == 0 {
            for (i, _) in column {
                if let Some(it) = list.get(*i) {
                    let w = widgets::item_width(ui, &item_key(prefix, *i), it, compact);
                    out.push((*i, Rect::from_min_size(pos2(x, origin.y + LARGE_TOP), vec2(w, widgets::LARGE_H))));
                    column_w = column_w.max(w);
                }
            }
        }
        for r in 1..=rows {
            let top = origin.y + SMALL_TOP + f32::from(r - 1) * ROW_STEP;
            let mut rx = x;
            for (i, _) in column.iter().filter(|(_, row)| *row == Some(r)) {
                if let Some(it) = list.get(*i) {
                    if rx > x {
                        rx += GAP;
                    }
                    let w = widgets::item_width(ui, &item_key(prefix, *i), it, compact);
                    out.push((*i, Rect::from_min_size(pos2(rx, top), vec2(w, widgets::SMALL_H))));
                    rx += w;
                }
            }
            column_w = column_w.max(rx - x);
        }
        x += column_w;
    }
    (out, x - origin.x)
}

/// How many rows a slide-out uses (1–4).
pub fn slideout_rows(list: &[Item]) -> u8 {
    list.iter().filter_map(small_row).max().unwrap_or(1).max(1)
}

/// A slide-out's size for `list`, at least `min_w` wide.
pub fn slideout_size(ui: &Ui, prefix: &str, list: &[Item], min_w: f32) -> egui::Vec2 {
    let (_, w) = place(ui, prefix, list, false, Pos2::ZERO);
    let rows = f32::from(slideout_rows(list));
    vec2((w + 2.0 * PAD).max(min_w), SMALL_TOP + rows * ROW_STEP + SLIDEOUT_BAR_H)
}

/// The width a panel's title bar needs: its label, the slide-out "▾" and the launcher.
pub fn title_width(ui: &Ui, panel: &Panel) -> f32 {
    let text = ui.painter().layout_no_wrap(crate::i18n::t(panel.label).to_string(), egui::FontId::proportional(12.0), egui::Color32::WHITE);
    let chevron = if panel.slideout.is_empty() { 0.0 } else { 14.0 };
    let launcher = if panel.launcher.is_some() { widgets::LAUNCHER.x + 2.0 } else { 0.0 };
    text.size().x + chevron + launcher + 12.0
}

/// A panel's natural width (without its divider): its columns, or its title if wider.
pub fn panel_width(ui: &Ui, tab: &str, panel: &Panel, compact: bool) -> f32 {
    let (_, content) = place(ui, &format!("{tab}/{}", panel.label), panel.items, compact, Pos2::ZERO);
    (content + 2.0 * PAD).max(title_width(ui, panel))
}

/// The left edge of a panel's content, centred when the title is wider than the columns.
pub fn content_left(rect: Rect, content_w: f32) -> f32 {
    rect.left() + ((rect.width() - content_w) / 2.0).max(PAD)
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLLAPSED: f32 = PANEL_BUTTON_W + DIVIDER;

    #[test]
    fn nothing_collapses_when_everything_fits() {
        assert_eq!(collapse(&[100.0, 200.0, 50.0], 350.0), [false, false, false]);
        assert_eq!(collapse(&[], 10.0), Vec::<bool>::new());
    }

    #[test]
    fn the_rightmost_panel_collapses_first() {
        assert_eq!(collapse(&[100.0, 200.0, 150.0], 300.0 + COLLAPSED), [false, false, true]);
    }

    #[test]
    fn all_but_the_first_collapse_when_only_the_first_fits() {
        assert_eq!(collapse(&[300.0, 200.0, 150.0, 120.0], 300.0 + 3.0 * COLLAPSED), [false, true, true, true]);
        assert_eq!(collapse(&[300.0, 200.0, 150.0], 10.0), [true, true, true], "too narrow for anything");
    }

    #[test]
    fn an_unbounded_width_never_collapses() {
        assert_eq!(collapse(&[1.0e6, 1.0e6], f32::INFINITY), [false, false]);
        assert_eq!(collapse(&[1.0e6, 1.0e6], f32::NAN), [false, false]);
    }

    #[test]
    fn panels_go_compact_from_the_right_before_any_collapses() {
        let full = [200.0, 200.0, 200.0];
        let compact = [150.0, 150.0, 150.0];
        assert_eq!(fit(&full, &compact, 600.0), [Fit::Full; 3]);
        assert_eq!(fit(&full, &compact, 550.0), [Fit::Full, Fit::Full, Fit::Compact]);
        assert_eq!(fit(&full, &compact, 450.0), [Fit::Compact; 3]);
        assert_eq!(fit(&full, &compact, 300.0 + COLLAPSED), [Fit::Compact, Fit::Compact, Fit::Collapsed]);
    }
}
