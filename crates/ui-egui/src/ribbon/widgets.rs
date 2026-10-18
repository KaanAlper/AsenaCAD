//! The ribbon's controls: large and small buttons, toggles, split buttons with their drop-down
//! menus, and the panel's dialog launcher (combo boxes and sliders are in `bind.rs`).
//!
//! Every control records itself (`icons::describe_control` / `control::record_widget`) under its
//! AutoCAD label, is disabled when its command doesn't exist yet or refuses to run now (its
//! tooltip then says which, `avail`), and acts only through commands: a click clears the
//! command-line buffer and starts the command.

use egui::text::LayoutJob;
use egui::{Rect, Response, Sense, Stroke, Ui, Vec2, pos2, vec2};

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
/// A large split with a two-line label: room either side of its widest line, and what its ▾
/// adds to its last line (the gap and the ▾).
const LARGE_SPLIT_PAD: f32 = 12.0;
const LARGE_SPLIT_CHEVRON: f32 = 16.0;
/// The lower part of a large split button that opens its menu (label and ▾).
const LARGE_SPLIT_ARROW_H: f32 = 35.0;
/// The ▾ part of a small split button.
const SMALL_SPLIT_ARROW_W: f32 = 13.0;
/// A large label's lines are at most this wide where a word break allows it: AutoCAD's large
/// buttons are 58–70 px, and "Detect and" (59 px) fits on one line while "Layer Properties"
/// doesn't.
const LARGE_WRAP: f32 = 60.0;
const MENU_W: f32 = 220.0;

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
        // No xref fading is drawn yet: the button only reflects XDWGFADECTL as the drawing has it.
        Toggle::XrefFade => var("XDWGFADECTL").is_some_and(|v| v > 0),
        Toggle::ParametersPalette => app.ui.dialog.as_deref() == Some("parameters"),
        // CCONSTRAINTFORM isn't modelled yet: every constraint is dynamic, AutoCAD's default.
        Toggle::DynamicConstraints => var("CCONSTRAINTFORM").unwrap_or(0) != 1,
        Toggle::AnnotationalConstraints => var("CCONSTRAINTFORM") == Some(1),
        Toggle::UcsIcon => app.ui.show_ucs_icon,
        Toggle::ViewCube => app.ui.show_viewcube,
        Toggle::NavBar => app.ui.show_navbar,
        Toggle::CommandLine => app.ui.show_command_line,
        Toggle::FileTabs => app.ui.show_file_tabs,
        Toggle::LayoutTabs => app.ui.show_layout_tabs,
        Toggle::PropertiesPalette => app.ui.docking.layout.contains(&crate::docking::Panel::Properties),
        Toggle::BlocksPalette => app.ui.dialog.as_deref() == Some("blocks"),
        Toggle::XrefPalette => app.ui.docking.layout.contains(&crate::docking::Panel::ExternalRefs),
        Toggle::NotBuilt => false,
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
        t.ribbon_pressed
    } else if resp.hovered() && resp.enabled() {
        t.ribbon_hover
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
    let mut job = LayoutJob::simple(large_lines(ui, text), crate::theme::ribbon_font(ui.ctx()), text_color(enabled), f32::INFINITY);
    job.halign = egui::Align::Center;
    ui.painter().layout_job(job)
}

/// `text` as AutoCAD lays a large label out: a single word on one line; more words on two,
/// broken at the last space that keeps the first line within [`LARGE_WRAP`] ("Detect and /
/// Convert", "Layer / Properties", "Data / Link"), or after the first word when even that is
/// wider ("Add / Current Scale"). Balancing the two lines would give "Add Current / Scale",
/// which AutoCAD doesn't (`tests/ribbon_tabs/labels.rs` lists every large label of the built tabs).
fn large_lines(ui: &Ui, text: &str) -> String {
    let width = |s: &str| ui.painter().layout_no_wrap(s.to_string(), crate::theme::ribbon_font(ui.ctx()), text_color(true)).size().x;
    let spaces = text.match_indices(' ').map(|(i, _)| i);
    let Some(first) = spaces.clone().next() else { return text.to_string() };
    let at = spaces.take_while(|&i| width(&text[..i]) <= LARGE_WRAP).last().unwrap_or(first);
    format!("{}\n{}", &text[..at], text[at..].trim_start())
}

fn small_label(ui: &Ui, text: &str, enabled: bool) -> std::sync::Arc<egui::Galley> {
    ui.painter().layout_no_wrap(text.to_string(), crate::theme::ribbon_font(ui.ctx()), text_color(enabled))
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
    // Centre-aligned rows are laid out around x = 0. Under a one-line label the ▾ goes below
    // it; a two-line label ("Detect and / Convert") has it right of its last line, and moves
    // the label left as far as the ▾ needs room in the button.
    let (shift, chevron_at) = match galley.rows.as_slice() {
        [_, .., last] if chevron => {
            let row = last.rect();
            let at = rect.center().x + row.right() + 7.0;
            let room = ((rect.width() - galley.size().x) / 2.0).max(0.0);
            let shift = (at + 5.0 - rect.right()).clamp(0.0, room);
            (shift, pos2((at - shift).min(rect.right() - 5.0), top + row.center().y))
        }
        _ => (0.0, pos2(rect.center().x, (bottom + 5.0).min(rect.bottom() - 5.0))),
    };
    ui.painter().galley(pos2(rect.center().x - shift, top), galley, text_color(enabled));
    if chevron {
        paint_chevron(ui, chevron_at, enabled);
    }
}

/// A large button's width: its widest label line with 3 px either side (AutoCAD's "Layer /
/// Properties" is its "Properties" line plus about that), and never narrower than the icon
/// with 3 px either side (View ▸ Viewport Tools' UCS Icon and View Cube are 38 px).
fn large_width(ui: &Ui, label: &str) -> f32 {
    (large_label(ui, label, true).size().x + 6.0).max(LARGE_ICON + 6.0)
}

/// A large split button's width. With a two-line label AutoCAD's button is wider than the
/// label ("Revision / Cloud ▾", "Set / Location ▾") and its ▾ sits right of the last line.
fn large_split_width(ui: &Ui, label: &str) -> f32 {
    let galley = large_label(ui, label, true);
    match galley.rows.as_slice() {
        [_, .., last] => (galley.size().x + LARGE_SPLIT_PAD).max(last.rect().width() + LARGE_SPLIT_CHEVRON).max(LARGE_ICON + 10.0),
        _ => large_width(ui, label),
    }
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
/// which flips the state (opens the palette, toggles PICKSTYLE, …); a toggle with no state yet
/// starts its command like a button (COUNT). Clicking a pressed palette toggle runs the palette's
/// close command instead.
pub fn toggle(ui: &mut Ui, app: &mut CadApp, icon: Icon, label: &str, cmd: &str, size: Size, state: Toggle) -> Response {
    let on = is_on(app, state);
    let resp = plain_button(ui, app, icon, label, cmd, size, Some(on));
    if resp.clicked() {
        app.cmd.buffer.clear();
        match state.close_command().filter(|_| on) {
            // Errors are echoed to the command history by `run`.
            Some(close) => {
                let _ = app.run(close, serde_json::Value::Null);
            }
            // No state to flip (COUNT, with no Count palette yet): start it like a button.
            None if state == Toggle::NotBuilt => start(app, cmd),
            None => {
                let _ = app.run(cmd, serde_json::Value::Null);
            }
        }
    }
    resp
}

/// Draws a button; a plain one (`on: None`) starts `cmd` when clicked, a toggle's click is
/// handled by [`toggle`].
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
    if resp.clicked() && on.is_none() {
        start(app, cmd);
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
        Size::Large => (large_split_width(ui, shown), LARGE_H),
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
        ui.painter().rect_filled(r, 2.0, Tokens::get().ribbon_hover);
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
    if let Some((i, cmd)) = menu_popup(app, &arrow, menu) {
        RibbonState::remember(ui.ctx(), key, i);
        start(app, cmd);
    } else if face.clicked() {
        start(app, cmd);
    }
    face
}

/// The menu `anchor` toggles open: `menu`'s entries; returns the one chosen (index, command).
fn menu_popup(app: &CadApp, anchor: &Response, menu: &[Entry]) -> Option<(usize, &'static str)> {
    let mut chosen = None;
    egui::Popup::from_toggle_button_response(anchor).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
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
    chosen
}

/// A drop-down button: the whole button opens `menu`, whose entries start their commands. It is
/// enabled while any entry's command exists; `key` keeps equal buttons apart.
pub fn menu_button(ui: &mut Ui, app: &mut CadApp, key: &str, icon: Icon, label: &str, size: Size, menu: &[Entry]) -> Response {
    let enabled = menu.iter().any(|e| matches!(e, Entry::Command { cmd, .. } if availability(ui, app, cmd).is_known()));
    let shown = crate::i18n::t(label);
    let (w, h) = match size {
        Size::Large => (large_split_width(ui, shown), LARGE_H),
        Size::Small { label } => (small_width(ui, label.then_some(shown)) + SMALL_SPLIT_ARROW_W, SMALL_H),
    };
    let (_, rect) = ui.allocate_space(vec2(w, h));
    let resp = ui.add_enabled_ui(enabled, |ui| ui.interact(rect, ui.id().with(("ribbon-menu", key)), Sense::click())).inner;
    let open = egui::Popup::is_id_open(ui.ctx(), egui::Popup::default_response_id(&resp));
    paint_state(ui, rect, &resp, open);
    match size {
        Size::Large => paint_large(ui, rect, icon, shown, enabled, true),
        Size::Small { label } => {
            let face = Rect::from_min_max(rect.min, pos2(rect.right() - SMALL_SPLIT_ARROW_W, rect.bottom()));
            paint_small(ui, face, icon, label.then_some(shown), enabled);
            paint_chevron(ui, pos2(rect.right() - SMALL_SPLIT_ARROW_W / 2.0, rect.center().y), enabled);
        }
    }
    icons::describe_control(ui, &resp, label, None, false);
    let resp = tooltips(resp, shown, &Availability::Missing);
    if let Some((_, cmd)) = menu_popup(app, &resp, menu) {
        start(app, cmd);
    }
    resp
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
            match shown_size(size, compact) {
                Size::Large => large_split_width(ui, crate::i18n::t(face)),
                Size::Small { .. } => width(size, face) + SMALL_SPLIT_ARROW_W,
            }
        }
        Item::Menu { label, size, .. } => match shown_size(size, compact) {
            Size::Large => large_split_width(ui, crate::i18n::t(label)),
            Size::Small { .. } => width(size, label) + SMALL_SPLIT_ARROW_W,
        },
        Item::Combo { width, .. } | Item::Slider { width, .. } => width,
        Item::Separator => SEPARATOR_W,
    }
}

/// An in-panel separator's width; its line runs down the middle.
const SEPARATOR_W: f32 = 10.0;

/// A thin vertical line between two groups of a panel's columns, filling the item's rect.
pub fn separator(ui: &mut Ui) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(SEPARATOR_W, LARGE_H), Sense::hover());
    let x = rect.center().x.round() + 0.5;
    ui.painter().vline(x, rect.y_range().shrink(4.0), Stroke::new(1.0, Tokens::get().border));
    resp
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
