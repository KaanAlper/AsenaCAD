//! The ribbon's controls: large and small buttons, toggles, split buttons with their drop-down
//! menus, and the panel's dialog launcher (combo boxes and sliders are in `bind.rs`).
//!
//! Every control records itself (`icons::describe_control` / `control::record_widget`) under its
//! AutoCAD label, is disabled when its command doesn't exist yet or refuses to run now (its
//! tooltip then says which, `avail`), and acts only through commands: a click clears the
//! command-line buffer and starts the command.

use egui::text::LayoutJob;
use egui::{FontId, Rect, Response, Sense, Stroke, Ui, Vec2, pos2, vec2};

use super::avail::{self, Availability};
use super::model::{Entry, Item, Size, Toggle};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// Content height of a panel: a large button fills it.
pub const LARGE_H: f32 = 72.0;
/// A small item's row height.
pub const SMALL_H: f32 = 22.0;
const LARGE_ICON: f32 = 32.0;
const SMALL_ICON: f32 = 16.0;
/// Icon-only small buttons are this wide.
const SMALL_W: f32 = 24.0;
/// The lower part of a large split button that opens its menu (label and ▾).
const LARGE_SPLIT_ARROW_H: f32 = 35.0;
/// The ▾ part of a small split button.
const SMALL_SPLIT_ARROW_W: f32 = 13.0;
/// Large labels wrap at this width, to at most two lines ("Layer / Properties").
const LARGE_WRAP: f32 = 58.0;
const MENU_W: f32 = 220.0;
const FONT: f32 = 12.0;

/// Per-split memory: which menu entry each split button's face shows and runs. Each split's
/// entry is kept on its own in egui memory (temporary data, for the life of the window), keyed
/// by panel and item position.
#[derive(Clone, Copy, Debug, Default)]
pub struct RibbonState;

impl RibbonState {
    fn id(key: &str) -> egui::Id {
        egui::Id::new(("ribbon-split-mem", key))
    }

    /// The entry the split `key` shows (its last-used one), or `default`.
    pub fn split(ctx: &egui::Context, key: &str, default: usize) -> usize {
        ctx.data_mut(|d| d.get_temp::<usize>(Self::id(key))).unwrap_or(default)
    }

    fn remember(ctx: &egui::Context, key: &str, entry: usize) {
        ctx.data_mut(|d| d.insert_temp(Self::id(key), entry));
    }
}

/// Whether `cmd` exists and may run now (the same check the menu bar's `menus::tree` makes),
/// and why not; looked up once per pass for the whole ribbon.
fn availability(ui: &Ui, app: &CadApp, cmd: &str) -> Availability {
    avail::availability(ui.ctx(), &app.session, cmd)
}

/// The tooltips of an item: `shown` while enabled, why it is disabled otherwise. Both are built
/// only when shown.
fn tooltips(resp: Response, shown: &str, why: &Availability) -> Response {
    resp.on_hover_ui(|ui| {
        ui.label(shown);
    })
    .on_disabled_hover_ui(|ui| {
        ui.label(why.disabled_text());
    })
}

/// Whether a toggle button shows pressed.
pub fn is_on(app: &CadApp, state: Toggle) -> bool {
    let var = |name: &str| cadcraft_engine::sysvars::get(&app.session, name).and_then(|v| v.as_i64());
    match state {
        Toggle::LayerPalette => app.ui.dialog.as_deref() == Some("layers"),
        // No fading is drawn yet: the button only reflects LAYLOCKFADECTL as the drawing has it.
        Toggle::LockedLayerFade => var("LAYLOCKFADECTL").is_some_and(|v| v > 0),
        Toggle::GroupSelection => app.session.settings.pickstyle & 1 != 0,
        Toggle::GroupBox => var("GROUPDISPLAYMODE") == Some(2),
        Toggle::QuickCalcPalette => app.ui.docking.layout.contains(&crate::docking::Panel::QuickCalc),
    }
}

fn start(app: &mut CadApp, cmd: &str) {
    app.cmd.buffer.clear();
    app.start(cmd);
}

/// Hover, pressed and "on" backgrounds.
fn paint_state(ui: &Ui, rect: Rect, resp: &Response, on: bool) {
    let t = Tokens::get();
    let fill = if on || resp.is_pointer_button_down_on() {
        t.select_bg
    } else if resp.hovered() && resp.enabled() {
        t.control_hover
    } else {
        return;
    };
    ui.painter().rect_filled(rect, 2.0, fill);
    if on {
        ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    }
}

fn text_color(enabled: bool) -> egui::Color32 {
    let t = Tokens::get();
    if enabled { t.text } else { t.text_faint }
}

fn large_label(ui: &Ui, text: &str, enabled: bool) -> std::sync::Arc<egui::Galley> {
    let mut job = LayoutJob::simple(text.to_string(), FontId::proportional(FONT), text_color(enabled), LARGE_WRAP);
    job.wrap.max_rows = 2;
    job.halign = egui::Align::Center;
    ui.painter().layout_job(job)
}

fn small_label(ui: &Ui, text: &str, enabled: bool) -> std::sync::Arc<egui::Galley> {
    ui.painter().layout_no_wrap(text.to_string(), FontId::proportional(FONT), text_color(enabled))
}

/// Reserve `size` in a scope that is disabled when `enabled` is false.
fn reserve(ui: &mut Ui, size: Vec2, enabled: bool, sense: Sense) -> Response {
    ui.add_enabled_ui(enabled, |ui| ui.allocate_exact_size(size, sense).1).inner
}

fn paint_chevron(ui: &Ui, center: egui::Pos2, enabled: bool) {
    icons::paint(ui.painter(), Rect::from_center_size(center, Vec2::splat(10.0)), Icon::ChevronDown, !enabled);
}

/// Paint a large face (icon above, label below) into `rect`.
fn paint_large(ui: &Ui, rect: Rect, icon: Icon, label: &str, enabled: bool, chevron: bool) {
    let icon_rect = Rect::from_center_size(pos2(rect.center().x, rect.top() + 4.0 + LARGE_ICON / 2.0), Vec2::splat(LARGE_ICON));
    icons::paint(ui.painter(), icon_rect, icon, !enabled);
    let galley = large_label(ui, label, enabled);
    let top = rect.top() + 4.0 + LARGE_ICON + 3.0;
    let bottom = top + galley.size().y;
    // Centre-aligned rows are laid out around x = 0.
    ui.painter().galley(pos2(rect.center().x, top), galley, text_color(enabled));
    if chevron {
        paint_chevron(ui, pos2(rect.center().x, (bottom + 5.0).min(rect.bottom() - 5.0)), enabled);
    }
}

fn large_width(ui: &Ui, label: &str) -> f32 {
    (large_label(ui, label, true).size().x + 10.0).max(LARGE_ICON + 10.0)
}

/// Paint a small face (16 px icon, optionally the label right of it) into `rect`.
fn paint_small(ui: &Ui, rect: Rect, icon: Icon, label: Option<&str>, enabled: bool) {
    let icon_rect = Rect::from_center_size(pos2(rect.left() + SMALL_W / 2.0, rect.center().y), Vec2::splat(SMALL_ICON));
    icons::paint(ui.painter(), icon_rect, icon, !enabled);
    if let Some(label) = label {
        let galley = small_label(ui, label, enabled);
        let y = rect.center().y - galley.size().y / 2.0;
        ui.painter().galley(pos2(rect.left() + SMALL_W, y), galley, text_color(enabled));
    }
}

fn small_width(ui: &Ui, label: Option<&str>) -> f32 {
    label.map_or(SMALL_W, |l| SMALL_W + small_label(ui, l, true).size().x + 5.0)
}

/// A large button: 32 px icon centred at the top, the label under it (up to two lines).
pub fn large_button(ui: &mut Ui, app: &mut CadApp, icon: Icon, label: &str, cmd: &str) -> Response {
    plain_button(ui, app, icon, label, cmd, Size::Large, None)
}

/// A small 22 px button; `show_label` puts the label right of the icon.
pub fn small_button(ui: &mut Ui, app: &mut CadApp, icon: Icon, label: &str, cmd: &str, show_label: bool) -> Response {
    plain_button(ui, app, icon, label, cmd, Size::Small { label: show_label }, None)
}

/// A button that shows pressed while `state` is on. Clicking runs `cmd` without parameters,
/// which flips the state (opens the palette, toggles PICKSTYLE, …).
pub fn toggle(ui: &mut Ui, app: &mut CadApp, icon: Icon, label: &str, cmd: &str, size: Size, state: Toggle) -> Response {
    let on = is_on(app, state);
    plain_button(ui, app, icon, label, cmd, size, Some(on))
}

fn plain_button(ui: &mut Ui, app: &mut CadApp, icon: Icon, label: &str, cmd: &str, size: Size, on: Option<bool>) -> Response {
    let why = availability(ui, app, cmd);
    let enabled = why.is_ready();
    let shown = crate::i18n::t(label);
    let (w, h) = match size {
        Size::Large => (large_width(ui, shown), LARGE_H),
        Size::Small { label } => (small_width(ui, label.then_some(shown)), SMALL_H),
    };
    let resp = reserve(ui, vec2(w, h), enabled, Sense::click());
    let rect = resp.rect;
    paint_state(ui, rect, &resp, on.unwrap_or(false));
    match size {
        Size::Large => paint_large(ui, rect, icon, shown, enabled, false),
        Size::Small { label } => paint_small(ui, rect, icon, label.then_some(shown), enabled),
    }
    icons::describe_control(ui, &resp, label, on, false);
    let resp = tooltips(resp, shown, &why);
    if resp.clicked() {
        match on {
            Some(_) => {
                app.cmd.buffer.clear();
                // Errors are echoed to the command history by `run`.
                let _ = app.run(cmd, serde_json::Value::Null);
            }
            None => start(app, cmd),
        }
    }
    resp
}

/// The accessibility and automation description of a split button's face: a button to
/// AccessKit, recorded once as a "splitbutton" for the control channel.
fn describe_face(ui: &Ui, face: &Response, shown: &str) {
    face.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, face.enabled(), shown));
    if face.has_focus() {
        ui.painter().rect_stroke(face.rect.shrink(2.0), 2.0, Stroke::new(1.0, Tokens::get().accent), egui::StrokeKind::Inside);
    }
    crate::control::record_widget(ui, face, shown, "splitbutton", None);
}

/// A split button: the face runs the remembered entry of `menu` (initially `default`); the ▾
/// part opens the menu, and choosing an entry runs it and makes it the face's. `label: Some`
/// keeps that label on the face (only the icon follows the entry); `None` shows the entry's.
/// `key` names the split for the memory (panel and position).
pub fn split_button(ui: &mut Ui, app: &mut CadApp, key: &str, label: Option<&str>, size: Size, menu: &[Entry], default: usize) -> Response {
    let (icon, entry_label, cmd) = split_face(ui.ctx(), key, menu, default);
    let face_label = label.unwrap_or(entry_label);
    let shown = crate::i18n::t(face_label);
    let face_why = availability(ui, app, cmd);
    let face_enabled = face_why.is_ready();
    let menu_enabled = menu.iter().any(|e| matches!(e, Entry::Command { cmd, .. } if availability(ui, app, cmd).is_known()));
    let (w, h) = match size {
        Size::Large => (large_width(ui, shown), LARGE_H),
        Size::Small { label } => (small_width(ui, label.then_some(shown)) + SMALL_SPLIT_ARROW_W, SMALL_H),
    };
    // Space only: the face and the ▾ part are the two interactive (and accessible) widgets.
    let (_, r) = ui.allocate_space(vec2(w, h));
    let (face_rect, arrow_rect) = match size {
        Size::Large => (
            Rect::from_min_max(r.min, pos2(r.right(), r.bottom() - LARGE_SPLIT_ARROW_H)),
            Rect::from_min_max(pos2(r.left(), r.bottom() - LARGE_SPLIT_ARROW_H), r.max),
        ),
        Size::Small { .. } => (
            Rect::from_min_max(r.min, pos2(r.right() - SMALL_SPLIT_ARROW_W, r.bottom())),
            Rect::from_min_max(pos2(r.right() - SMALL_SPLIT_ARROW_W, r.top()), r.max),
        ),
    };
    let id = ui.id().with(("ribbon-split", key));
    let face = ui.add_enabled_ui(face_enabled, |ui| ui.interact(face_rect, id.with("face"), Sense::click())).inner;
    let arrow = ui.add_enabled_ui(menu_enabled, |ui| ui.interact(arrow_rect, id.with("menu"), Sense::click())).inner;
    if (face.hovered() && face.enabled()) || (arrow.hovered() && arrow.enabled()) {
        ui.painter().rect_filled(r, 2.0, Tokens::get().control_hover);
    }
    paint_state(ui, face_rect, &face, false);
    paint_state(ui, arrow_rect, &arrow, false);
    match size {
        Size::Large => paint_large(ui, r, icon, shown, face_enabled, true),
        Size::Small { label } => {
            paint_small(ui, face_rect, icon, label.then_some(shown), face_enabled);
            paint_chevron(ui, arrow_rect.center(), menu_enabled);
        }
    }
    describe_face(ui, &face, shown);
    let face = tooltips(face, shown, &face_why);
    let menu_label = crate::tf!("{label} options", label = shown);
    icons::describe_control(ui, &arrow, &menu_label, None, false);
    let arrow = tooltips(arrow, &menu_label, &Availability::Missing);
    let mut chosen = None;
    egui::Popup::from_toggle_button_response(&arrow).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
        ui.set_min_width(MENU_W);
        for (i, e) in menu.iter().enumerate() {
            match e {
                Entry::Separator => {
                    ui.separator();
                }
                Entry::Command { icon, label, cmd } => {
                    if menu_entry(ui, app, *icon, label, cmd).clicked() {
                        chosen = Some((i, *cmd));
                    }
                }
            }
        }
    });
    if let Some((i, cmd)) = chosen {
        RibbonState::remember(ui.ctx(), key, i);
        start(app, cmd);
    } else if face.clicked() {
        start(app, cmd);
    }
    face
}

/// The entry a split's face shows and runs: its icon, label and command.
pub fn split_face(ctx: &egui::Context, key: &str, menu: &[Entry], default: usize) -> (Icon, &'static str, &'static str) {
    let current = RibbonState::split(ctx, key, default);
    match menu.get(current).or_else(|| menu.get(default)) {
        Some(Entry::Command { icon, label, cmd }) => (*icon, *label, *cmd),
        _ => (Icon::Menu, "", ""),
    }
}

/// `size` with small items' labels hidden when `compact` (a narrow ribbon).
pub fn shown_size(size: Size, compact: bool) -> Size {
    match size {
        Size::Small { label } => Size::Small { label: label && !compact },
        Size::Large => Size::Large,
    }
}

/// The width `item` (named `key`) takes in a panel; `compact` hides small items' labels.
pub fn item_width(ui: &Ui, key: &str, item: &Item, compact: bool) -> f32 {
    let width = |size: Size, label: &str| match shown_size(size, compact) {
        Size::Large => large_width(ui, crate::i18n::t(label)),
        Size::Small { label: shown } => small_width(ui, shown.then(|| crate::i18n::t(label))),
    };
    match *item {
        Item::Button { label, size, .. } | Item::Toggle { label, size, .. } => width(size, label),
        Item::Split { label, size, menu, default, .. } => {
            let face = label.unwrap_or(split_face(ui.ctx(), key, menu, default).1);
            let arrow = if matches!(size, Size::Small { .. }) { SMALL_SPLIT_ARROW_W } else { 0.0 };
            width(size, face) + arrow
        }
        Item::Combo { width, .. } | Item::Slider { width, .. } => width,
    }
}

/// One row of a split button's menu: icon and label.
fn menu_entry(ui: &mut Ui, app: &CadApp, icon: Icon, label: &str, cmd: &str) -> Response {
    let why = availability(ui, app, cmd);
    let enabled = why.is_ready();
    let shown = crate::i18n::t(label);
    let resp = reserve(ui, vec2(MENU_W, SMALL_H + 2.0), enabled, Sense::click());
    paint_state(ui, resp.rect, &resp, false);
    let icon_rect = Rect::from_center_size(pos2(resp.rect.left() + 14.0, resp.rect.center().y), Vec2::splat(SMALL_ICON));
    icons::paint(ui.painter(), icon_rect, icon, !enabled);
    let galley = small_label(ui, shown, enabled);
    let y = resp.rect.center().y - galley.size().y / 2.0;
    ui.painter().galley(pos2(resp.rect.left() + 30.0, y), galley, text_color(enabled));
    icons::describe_control(ui, &resp, label, None, false);
    resp.on_disabled_hover_ui(|ui| {
        ui.label(why.disabled_text());
    })
}

/// The 14×18 "↘" dialog launcher at the right end of a panel's title bar, placed at `rect`.
/// `panel` (the panel's key) keeps launchers of equal commands apart; `label` is the dialog it
/// opens.
pub fn launcher(ui: &mut Ui, app: &mut CadApp, rect: Rect, panel: &str, label: &str, cmd: &str) -> Response {
    let why = availability(ui, app, cmd);
    let enabled = why.is_ready();
    let id = ui.id().with(("ribbon-launcher", panel, cmd));
    let resp = ui.add_enabled_ui(enabled, |ui| ui.interact(rect, id, Sense::click())).inner;
    paint_state(ui, rect, &resp, false);
    icons::paint(ui.painter(), Rect::from_center_size(rect.center(), Vec2::splat(12.0)), Icon::Launcher, !enabled);
    icons::describe_control(ui, &resp, label, None, false);
    let resp = tooltips(resp, crate::i18n::t(label), &why);
    if resp.clicked() {
        start(app, cmd);
    }
    resp
}

/// A command's display label, if the engine has one (looked up once per pass).
pub fn command_label(ui: &Ui, app: &CadApp, cmd: &str) -> Option<&'static str> {
    avail::command(ui.ctx(), &app.session, cmd).label
}

/// The launcher's size.
pub const LAUNCHER: Vec2 = Vec2::new(14.0, 18.0);
