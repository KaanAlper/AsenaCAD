//! The command line bar's controls, left to right as in AutoCAD: the drag grip, close "×" and
//! Customize (wrench) on a lighter block, the recent-commands button (prompt icon and ▾), the
//! input field and the history toggle "▴" at the right end. [`crate::cmdline`] draws the input
//! field's prompt and typing; this lays the bar out and handles its buttons.

use egui::{Rect, Sense, Stroke, pos2, vec2};
use serde_json::Value;

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// The bar's height (AutoCAD's 27 px), its share of the drawing area's width (1138 of 1898 px)
/// and its gap above the drawing area's bottom edge.
pub const BAR_H: f32 = 27.0;
const BAR_SHARE: f32 = 0.6;
const BAR_GAP: f32 = 3.0;
const BAR_MIN_W: f32 = 360.0;
const GRIP_W: f32 = 10.0;
const BUTTON: f32 = 20.0;
const RECENT: egui::Vec2 = egui::Vec2 { x: 32.0, y: 22.0 };
const HISTORY: egui::Vec2 = egui::Vec2 { x: 15.0, y: 25.0 };
/// How many recent commands the recent-commands button lists.
const RECENT_MAX: usize = 10;

pub const GRIP: &str = "Command Line Grip";
pub const CLOSE: &str = "Close Command Line";
pub const CUSTOMIZE: &str = "Customize";
pub const RECENT_COMMANDS: &str = "Recent Commands";
pub const INPUT: &str = "Type a command";
pub const HISTORY_TOGGLE: &str = "Command History";
const NO_RECENT: &str = "No recent commands";

/// The labels the bar shows, for the translation catalogs.
pub fn labels() -> impl Iterator<Item = &'static str> {
    [GRIP, CLOSE, CUSTOMIZE, RECENT_COMMANDS, INPUT, HISTORY_TOGGLE, NO_RECENT].into_iter()
}

/// Where the bar sits over the drawing area `canvas`: centred, 60% of its width, just above its
/// bottom edge.
pub fn bar_rect(canvas: Rect) -> Rect {
    let w = (canvas.width() * BAR_SHARE).max(BAR_MIN_W).min((canvas.width() - 8.0).max(0.0));
    Rect::from_min_size(pos2(canvas.center().x - w / 2.0, canvas.bottom() - BAR_H - BAR_GAP), vec2(w, BAR_H))
}

/// The bar's parts.
#[derive(Clone, Copy, Debug)]
pub struct Parts {
    pub bar: Rect,
    /// The lighter block holding the grip, close and Customize.
    pub block: Rect,
    pub grip: Rect,
    pub close: Rect,
    pub customize: Rect,
    pub recent: Rect,
    /// Where the prompt and the typed text go.
    pub input: Rect,
    pub history: Rect,
}

impl Parts {
    pub fn new(bar: Rect) -> Self {
        let cy = bar.center().y;
        let grip = Rect::from_min_size(pos2(bar.left() + 1.0, bar.top()), vec2(GRIP_W, bar.height()));
        let close = Rect::from_center_size(pos2(grip.right() + 2.0 + BUTTON / 2.0, cy), vec2(BUTTON, BUTTON));
        let customize = Rect::from_center_size(pos2(close.right() + 2.0 + BUTTON / 2.0, cy), vec2(BUTTON, BUTTON));
        let block = Rect::from_min_max(bar.min, pos2(customize.right() + 3.0, bar.bottom()));
        let recent = Rect::from_min_size(pos2(block.right() + 3.0, cy - RECENT.y / 2.0), RECENT);
        let history = Rect::from_min_size(pos2(bar.right() - HISTORY.x - 1.0, cy - HISTORY.y / 2.0), HISTORY);
        let input = Rect::from_min_max(pos2(recent.right() + 4.0, bar.top()), pos2((history.left() - 2.0).max(recent.right() + 4.0), bar.bottom()));
        Parts { bar, block, grip, close, customize, recent, input, history }
    }
}

/// The bar's background, block and button faces.
pub fn paint(p: &egui::Painter, parts: &Parts) {
    let t = Tokens::get();
    p.rect_filled(parts.bar, 0.0, t.cmd_bg);
    p.rect_filled(parts.block, 0.0, t.cmd_block);
    p.rect_stroke(parts.bar, 0.0, Stroke::new(1.0, t.cmd_border), egui::StrokeKind::Inside);
    // The grip: two columns of dots.
    let g = parts.grip.center();
    for row in -3..=3 {
        for col in [-1.5, 1.5] {
            p.circle_filled(pos2(g.x + col, g.y + row as f32 * 3.0), 0.8, t.text_faint);
        }
    }
}

fn hover_fill(ui: &egui::Ui, resp: &egui::Response) {
    let t = Tokens::get();
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(resp.rect, 2.0, t.ribbon_pressed);
    } else if resp.hovered() {
        ui.painter().rect_filled(resp.rect, 2.0, t.ribbon_hover);
    }
}

/// The bar's buttons (their hover fills and glyphs), on top of the command line's own widget.
pub fn controls(app: &mut CadApp, ui: &mut egui::Ui, parts: &Parts) {
    let grip = ui.interact(parts.grip, ui.id().with("cmdbar_grip"), Sense::hover());
    icons::describe_control(ui, &grip, GRIP, None, false);
    let close = ui.interact(parts.close, ui.id().with("cmdbar_close"), Sense::click());
    hover_fill(ui, &close);
    icons::paint(ui.painter(), Rect::from_center_size(parts.close.center(), vec2(13.0, 13.0)), Icon::Close, false);
    icons::describe_control(ui, &close, CLOSE, None, false);
    if close.on_hover_text(crate::tl!("Close")).clicked() {
        let _ = app.run("commandlinehide", Value::Null);
    }
    customize(app, ui, parts);
    recent(app, ui, parts);
    let input = ui.interact(parts.input, ui.id().with("cmdbar_input"), Sense::hover());
    let label = crate::i18n::t(INPUT);
    input.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::TextEdit, true, label));
    crate::control::record_widget(ui, &input, label, "textedit", None);
    let history = ui.interact(parts.history, ui.id().with("cmdbar_history"), Sense::click());
    hover_fill(ui, &history);
    let chevron = if app.cmd.expanded { Icon::ChevronDown } else { Icon::ChevronUp };
    icons::paint(ui.painter(), Rect::from_center_size(parts.history.center(), vec2(11.0, 11.0)), chevron, false);
    icons::describe_control(ui, &history, HISTORY_TOGGLE, Some(app.cmd.expanded), false);
    if history.on_hover_text(crate::tl!("Command History")).clicked() {
        let _ = app.run("ui.toggle.history", Value::Null);
    }
}

/// Customize (wrench): the command line's settings (the history's height) and Options.
fn customize(app: &mut CadApp, ui: &mut egui::Ui, parts: &Parts) {
    let resp = ui.interact(parts.customize, ui.id().with("cmdbar_customize"), Sense::click());
    hover_fill(ui, &resp);
    icons::paint(ui.painter(), Rect::from_center_size(parts.customize.center(), vec2(15.0, 15.0)), Icon::Wrench, false);
    icons::describe_control(ui, &resp, CUSTOMIZE, None, false);
    let resp = resp.on_hover_text(crate::tl!("Customize"));
    let mut chosen = None;
    egui::Popup::from_toggle_button_response(&resp).align(egui::RectAlign::TOP_START).show(|ui| {
        ui.set_min_width(200.0);
        crate::context_menu::groups_ui(ui, &crate::shortcut_menus::command_line_customize_entries(app), &mut chosen);
    });
    if let Some(id) = chosen {
        crate::shortcut_menus::choose(app, ui.ctx(), &id);
    }
}

/// The recent-commands button: the commands used lately, newest first; choosing one runs it.
fn recent(app: &mut CadApp, ui: &mut egui::Ui, parts: &Parts) {
    let resp = ui.interact(parts.recent, ui.id().with("cmdbar_recent"), Sense::click());
    hover_fill(ui, &resp);
    let r = parts.recent;
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.left() + 11.0, r.center().y), vec2(17.0, 17.0)), Icon::Prompt, false);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(r.right() - 7.0, r.center().y), vec2(9.0, 9.0)), Icon::ChevronDown, false);
    icons::describe_control(ui, &resp, RECENT_COMMANDS, None, false);
    let resp = resp.on_hover_text(crate::tl!("Recent Commands"));
    let mut run = None;
    egui::Popup::from_toggle_button_response(&resp).align(egui::RectAlign::TOP_START).show(|ui| {
        ui.set_min_width(160.0);
        // Read from the history only while the list is open.
        let commands = app.session.recent_commands(RECENT_MAX);
        if commands.is_empty() {
            ui.add_enabled(false, egui::Button::new(crate::tl!("No recent commands")));
        }
        for id in &commands {
            let label = id.to_ascii_uppercase();
            let b = ui.button(&label);
            crate::control::record_widget(ui, &b, &label, "button", None);
            if b.clicked() {
                run = Some(id.clone());
                ui.close();
            }
        }
    });
    if let Some(id) = run {
        app.cmd.buffer.clear();
        app.start(&id);
    }
}
