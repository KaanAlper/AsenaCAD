//! Panels that open over the drawing: a panel's slide-out (its title "▾" drops the extra items
//! below the panel) and a collapsed panel's popup (its panel button shows the whole panel).
//!
//! At most one is open at a time (AutoCAD's behaviour). Unless pinned, it closes about half a
//! second after the pointer leaves both the panel and what opened below it, at once when the
//! user clicks elsewhere, and on Escape. While one of its own menus (a split button's ▾, a combo
//! box's list) is open the pointer counts as inside: those menus are separate egui popups and
//! often reach past the slide-out.

use egui::{Area, Context, Id, Order, Pos2, Rect, Sense, Ui, Vec2, pos2, vec2};

use super::layout::{self, PANEL_H, SLIDEOUT_BAR_H};
use super::model::{Panel, Tab};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// How long an unpinned slide-out or panel popup stays open after the pointer leaves it.
pub const CLOSE_DELAY: f64 = 0.5;

/// What is open: `panel` is "<tab>/<panel>" (empty when only the dropped-down strip is open). `popup`: a collapsed panel (or a minimized ribbon's
/// panel button) shows its panel below the button; `slideout`: the panel's slide-out is down.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Flyout {
    pub panel: String,
    pub popup: bool,
    pub slideout: bool,
    pub pinned: bool,
    /// The ribbon is minimized to tabs and the selected tab's panels dropped down.
    pub strip: bool,
    /// When the pointer left everything that counts as inside.
    outside_since: Option<f64>,
    /// What counted as inside this frame (panel, popup, slide-out).
    inside: Vec<Rect>,
}

fn id() -> Id {
    Id::new("asenacad-ribbon-flyout")
}

/// The open slide-out or panel popup, if any.
pub fn get(ctx: &Context) -> Option<Flyout> {
    ctx.data_mut(|d| d.get_temp::<Option<Flyout>>(id()).flatten())
}

fn set(ctx: &Context, flyout: Option<Flyout>) {
    ctx.data_mut(|d| d.insert_temp(id(), flyout));
}

pub fn close(ctx: &Context) {
    set(ctx, None);
}

/// Whether `panel`'s slide-out is open (in the strip, or in its popup when `popup`).
pub fn slideout_open(ctx: &Context, panel: &str, popup: bool) -> bool {
    get(ctx).is_some_and(|f| f.panel == panel && f.slideout && f.popup == popup)
}

/// Whether `panel` is shown as a popup.
pub fn popup_open(ctx: &Context, panel: &str) -> bool {
    get(ctx).is_some_and(|f| f.panel == panel && f.popup)
}

/// What stays when a panel's popup or slide-out closes: the dropped-down strip, if open.
fn base(current: Option<&Flyout>) -> Option<Flyout> {
    current.filter(|f| f.strip).map(|_| Flyout { strip: true, ..Flyout::default() })
}

/// The panel title's "▾": opens the slide-out (closing any other flyout) or closes it.
/// Inside a popup it drops the slide-out below the popup's panel.
pub fn toggle_slideout(ctx: &Context, panel: &str, popup: bool) {
    let current = get(ctx);
    let next = match &current {
        Some(f) if f.panel == panel && f.popup == popup && f.slideout => {
            if popup {
                Some(Flyout { slideout: false, ..f.clone() })
            } else {
                base(current.as_ref())
            }
        }
        Some(f) if f.panel == panel && f.popup == popup => Some(Flyout { slideout: true, ..f.clone() }),
        _ => Some(Flyout { panel: panel.to_string(), popup, slideout: true, strip: current.as_ref().is_some_and(|f| f.strip), ..Flyout::default() }),
    };
    set(ctx, next);
}

/// A panel button: shows its panel as a popup, or closes it.
pub fn toggle_popup(ctx: &Context, panel: &str) {
    let current = get(ctx);
    let next = match &current {
        Some(f) if f.panel == panel && f.popup => base(current.as_ref()),
        _ => Some(Flyout { panel: panel.to_string(), popup: true, strip: current.as_ref().is_some_and(|f| f.strip), ..Flyout::default() }),
    };
    set(ctx, next);
}

/// A tab of a ribbon minimized to tabs: drops its panels down, or closes them.
pub fn toggle_strip(ctx: &Context) {
    let open = get(ctx).is_some_and(|f| f.strip);
    set(ctx, (!open).then(|| Flyout { strip: true, ..Flyout::default() }));
}

/// Whether the minimized ribbon's panels are dropped down.
pub fn strip_open(ctx: &Context) -> bool {
    get(ctx).is_some_and(|f| f.strip)
}

/// Count `rect` as inside the open flyout this frame (its panel, popup or slide-out).
pub fn add_inside(ctx: &Context, rect: Rect) {
    ctx.data_mut(|d| {
        if let Some(f) = d.get_temp_mut_or_default::<Option<Flyout>>(id()).as_mut() {
            f.inside.push(rect);
        }
    });
}

fn set_pinned(ctx: &Context, pinned: bool) {
    if let Some(f) = get(ctx) {
        set(ctx, Some(Flyout { pinned, ..f }));
    }
}

/// End of the ribbon's frame: close the flyout when the pointer has been outside long enough,
/// on a click outside, or on Escape. Pinned flyouts stay until their title or button is used.
pub fn track(ctx: &Context) {
    let Some(mut f) = get(ctx) else { return };
    let inside_rects = std::mem::take(&mut f.inside);
    if f.pinned {
        set(ctx, Some(f));
        return;
    }
    let (pointer, pressed, escape, now) = ctx.input(|i| (i.pointer.latest_pos(), i.pointer.any_pressed(), i.key_pressed(egui::Key::Escape), i.time));
    // A split's or combo's own menu is a separate popup: it belongs to the flyout while open.
    let menu_open = egui::Popup::is_any_open(ctx);
    let over = pointer.is_some_and(|p| inside_rects.iter().any(|r| r.contains(p)));
    if escape || (pressed && !over && !menu_open) {
        close(ctx);
        return;
    }
    if over || menu_open {
        f.outside_since = None;
    } else {
        let since = *f.outside_since.get_or_insert(now);
        let left = CLOSE_DELAY - (now - since);
        if left <= 0.0 {
            close(ctx);
            return;
        }
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(left));
    }
    set(ctx, Some(f));
}

/// The slide-out body of `panel` (key `key` = "<tab>/<panel>") in `rect`: its items in up to four
/// rows and the pin button at the bottom-left.
pub fn body(ui: &mut Ui, app: &mut CadApp, rect: Rect, key: &str, panel: &Panel) {
    let t = Tokens::get();
    ui.painter().rect_filled(rect, 0.0, t.ribbon_bg);
    ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    let prefix = format!("{key}/slideout");
    super::render_items(ui, app, &prefix, panel.slideout, false, pos2(rect.left() + layout::PAD, rect.top()));
    let bar = Rect::from_min_max(pos2(rect.left(), rect.bottom() - SLIDEOUT_BAR_H), rect.max);
    ui.painter().rect_filled(bar.shrink(1.0), 0.0, t.ribbon_panel_title);
    let pin_rect = Rect::from_min_size(pos2(bar.left() + 3.0, bar.center().y - 8.0), Vec2::splat(16.0));
    let pinned = get(ui.ctx()).is_some_and(|f| f.pinned);
    let resp = ui.interact(pin_rect, ui.id().with(("ribbon-pin", key)), Sense::click());
    if pinned || resp.hovered() {
        ui.painter().rect_filled(pin_rect, 2.0, if pinned { t.ribbon_pressed } else { t.ribbon_hover });
    }
    icons::paint(ui.painter(), pin_rect.shrink(1.0), if pinned { Icon::Pin } else { Icon::Unpin }, false);
    icons::describe_control(ui, &resp, "Pin", Some(pinned), false);
    if resp.on_hover_text(crate::tl!("Pin")).clicked() {
        set_pinned(ui.ctx(), !pinned);
    }
}

/// The popup of a collapsed (or minimized) panel below `anchor`: the full panel with its title
/// bar and launcher, and its slide-out below it when open.
pub(super) fn panel_popup(app: &mut CadApp, ui: &Ui, anchor: Rect, key: &str, tab: &Tab, panel: &Panel) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    let w = layout::panel_width(ui, tab.label, panel, false);
    let prefix = format!("{key}/slideout");
    let slide = (slideout_open(&ctx, key, true) && !panel.slideout.is_empty()).then(|| layout::slideout_size(ui, &prefix, panel.slideout, w));
    let size = vec2(w.max(slide.map_or(0.0, |s| s.x)), PANEL_H + slide.map_or(0.0, |s| s.y));
    Area::new(Id::new(("asenacad-ribbon-popup", key))).order(Order::Foreground).fixed_pos(pos2(anchor.left(), anchor.bottom())).constrain(true).show(
        &ctx,
        |ui| {
            let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
            ui.painter().rect_filled(rect, 0.0, t.ribbon_bg);
            ui.painter().rect_stroke(rect, 0.0, egui::Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
            let panel_rect = Rect::from_min_size(rect.min, vec2(w, PANEL_H));
            super::show_panel(app, ui, panel_rect, key, panel, false, true);
            if let Some(s) = slide {
                body(ui, app, Rect::from_min_size(pos2(rect.left(), panel_rect.bottom()), s), key, panel);
            }
            add_inside(ui.ctx(), rect);
        },
    );
}

/// A panel's slide-out dropped below the strip at `at`, at least `min_w` wide.
pub(super) fn slideout_area(app: &mut CadApp, ui: &Ui, at: Pos2, min_w: f32, key: &str, panel: &Panel) {
    let ctx = ui.ctx().clone();
    let size = layout::slideout_size(ui, &format!("{key}/slideout"), panel.slideout, min_w);
    Area::new(Id::new(("asenacad-ribbon-slideout", key))).order(Order::Foreground).fixed_pos(at).constrain(true).show(&ctx, |ui| {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        body(ui, app, rect, key, panel);
        add_inside(ui.ctx(), rect);
    });
}
