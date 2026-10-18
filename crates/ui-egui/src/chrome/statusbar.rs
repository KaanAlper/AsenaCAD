//! The status bar row at the bottom of the window, as in AutoCAD's Drafting & Annotation
//! workspace: a band along its top edge separating it from the drawing area, the layout bar on
//! the left ([`super::layouts`]) and the status bar items on the right, ending with the
//! Customization "≡" menu that shows or hides each item (`ui.statusbar`, saved with the
//! workspace in [`crate::UiState::status_shown`]).
//!
//! Items appear in the order of AutoCAD's Customization list ([`ITEMS`]); the ones AutoCAD hides
//! by default start hidden. Every item acts through a command; an item whose command AsenaCAD
//! doesn't have yet shows disabled, saying so.

use egui::{Rect, Response, Sense, Ui, pos2, vec2};
use serde_json::{Value, json};

use super::statusmenus::{self, Menu};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::ribbon::avail::{self, Availability};
use crate::theme::Tokens;

/// The status bar row's height (AutoCAD's 33 px), the band along its top included.
pub const HEIGHT: f32 = 33.0;
pub const BAND_H: f32 = 3.0;
/// An item's icon slot, the ▾ part of a split item, and the gap between items.
const SLOT: f32 = 24.0;
const ARROW_W: f32 = 16.0;
const GAP: f32 = 2.0;
const GLYPH: f32 = 17.0;
/// The room a transient message keeps between the layout tabs and the items.
const READOUT_MIN: f32 = 150.0;
pub const CUSTOMIZATION: &str = "Customization";
pub const MODEL: &str = "MODEL";
pub const PAPER: &str = "PAPER";
const SPACE_TIP: &str = "Model or Paper Space";

/// A status bar item: its key (saved in the workspace), label, group in the Customization list
/// (a separator between groups) and whether AutoCAD shows it by default.
#[derive(Clone, Copy, Debug)]
pub struct Item {
    pub key: &'static str,
    pub label: &'static str,
    pub group: u8,
    pub default: bool,
}

const fn item(key: &'static str, label: &'static str, group: u8, default: bool) -> Item {
    Item { key, label, group, default }
}

/// AutoCAD's Customization list, in order; the status bar shows the checked ones in this order.
pub const ITEMS: &[Item] = &[
    item("coordinates", "Coordinates", 0, false),
    item("modelspace", "Model Space", 1, true),
    item("grid", "Grid", 2, true),
    item("snap", "Snap Mode", 2, true),
    item("infer", "Infer Constraints", 3, false),
    item("dynmode", "Dynamic Input", 4, false),
    item("ortho", "Ortho Mode", 5, true),
    item("polar", "Polar Tracking", 5, true),
    item("isodraft", "Isometric Drafting", 5, true),
    item("otrack", "Object Snap Tracking", 6, true),
    item("osnap", "2D Object Snap", 6, true),
    item("lineweight", "LineWeight", 7, false),
    item("transparency", "Transparency", 8, false),
    item("selcycling", "Selection Cycling", 9, false),
    item("osnap3d", "3D Object Snap", 10, false),
    item("ducs", "Dynamic UCS", 11, false),
    item("selfilter", "Selection Filtering", 11, false),
    item("gizmo", "Gizmo", 11, false),
    item("annovisibility", "Annotation Visibility", 12, true),
    item("autoscale", "AutoScale", 12, true),
    item("annoscale", "Annotation Scale", 12, true),
    item("workspace", "Workspace Switching", 13, true),
    item("annomonitor", "Annotation Monitor", 14, true),
    item("units", "Units", 15, false),
    item("quickprops", "Quick Properties", 16, false),
    item("lockui", "Lock UI", 17, false),
    item("isolate", "Isolate Objects", 18, true),
    item("graphics", "Graphics Performance", 19, false),
    item("cleanscreen", "Clean Screen", 20, true),
];

/// The items AutoCAD shows by default.
pub fn default_shown() -> Vec<String> {
    ITEMS.iter().filter(|i| i.default).map(|i| i.key.to_string()).collect()
}

/// Every label the status bar row shows, for the translation catalogs.
pub fn labels() -> impl Iterator<Item = &'static str> {
    ITEMS
        .iter()
        .map(|i| i.label)
        .chain([CUSTOMIZATION, MODEL, PAPER, SPACE_TIP, super::layouts::LIST, super::layouts::NEW, super::layouts::MORE])
        .chain(statusmenus::labels())
}

fn is_shown(app: &CadApp, key: &str) -> bool {
    app.ui.status_shown.iter().any(|k| k == key)
}

/// `ui.statusbar {item, on?}`: show or hide one item (`on` absent: toggle it); without `item`,
/// report the items shown.
pub fn command(app: &mut CadApp, params: &Value) -> Result<Value, String> {
    let Some(key) = params.get("item").and_then(Value::as_str) else {
        return Ok(json!({ "shown": app.ui.status_shown }));
    };
    let Some(it) = ITEMS.iter().find(|i| i.key == key) else {
        let keys: Vec<&str> = ITEMS.iter().map(|i| i.key).collect();
        return Err(format!("unknown status bar item \"{}\" (one of {})", key.chars().take(40).collect::<String>(), keys.join(", ")));
    };
    let on = params.get("on").and_then(Value::as_bool).unwrap_or(!is_shown(app, it.key));
    let shown: Vec<String> = ITEMS.iter().filter(|i| if i.key == it.key { on } else { is_shown(app, i.key) }).map(|i| i.key.to_string()).collect();
    app.ui.status_shown = shown;
    Ok(json!({ "item": it.key, "on": on }))
}

/// The status bar row.
pub fn show(app: &mut CadApp, ui: &mut Ui) {
    let t = Tokens::get();
    egui::Panel::bottom("cc_status").exact_size(HEIGHT).show_separator_line(false).frame(egui::Frame::NONE.fill(t.status_bg)).show(ui, |ui| {
        let r = ui.max_rect();
        // Clear the status message on time even when no input arrives (the app only repaints on input).
        if let Some((_, at)) = &app.status
            && crate::now_ms() - at < 5000.0
        {
            let left = std::time::Duration::try_from_secs_f64((5000.0 - (crate::now_ms() - at)) / 1000.0).unwrap_or_default();
            ui.ctx().request_repaint_after(left);
        }
        let band = Rect::from_min_max(r.min, pos2(r.right(), r.top() + BAND_H));
        ui.painter().rect_filled(band, 0.0, t.status_band);
        let row = Rect::from_min_max(pos2(r.left(), band.bottom()), r.max);
        let cy = row.center().y;
        // Right to left: Customization, then the items shown, last to first.
        let mut right = customization(app, ui, row.right() - 4.0, cy) - GAP;
        for it in ITEMS.iter().rev() {
            if is_shown(app, it.key) {
                right = item_ui(app, ui, it, right, cy) - GAP;
            }
        }
        let message = app.status.as_ref().filter(|(_, at)| crate::now_ms() - at < 5000.0).map(|(m, _)| m.clone());
        // The layout tabs start right of a palette docked on the left, measured from the
        // drawing area's last position.
        let left_dock = app.canvas.rect.map_or(0.0, |c| c.left() - r.left());
        let tabs_row = Rect::from_min_max(pos2(r.left() + if left_dock > 60.0 { left_dock + 8.0 } else { 0.0 }, row.top()), row.max);
        let reserve = if message.is_some() { READOUT_MIN } else { 8.0 };
        let tabs_end = super::layouts::show(app, ui, tabs_row, band.top(), right - reserve);
        if let Some(text) = message {
            readout(ui, &text, Rect::from_min_max(pos2(tabs_end + 8.0, row.top()), pos2(right - 8.0, row.bottom())));
        }
    });
}

/// A transient message between the layout tabs and the items, cut to fit, in full on hover.
fn readout(ui: &mut Ui, text: &str, rect: Rect) {
    if rect.width() <= 4.0 {
        return;
    }
    let t = Tokens::get();
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), crate::theme::body(), t.warn);
    job.wrap = egui::text::TextWrapping { max_width: rect.width(), max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    let galley = ui.painter().layout_job(job);
    ui.painter().with_clip_rect(rect).galley(pos2(rect.left(), rect.center().y - galley.size().y / 2.0), galley, t.warn);
    ui.interact(rect, ui.id().with("status_readout"), Sense::hover()).on_hover_text(text);
}

/// An interactive rect that is disabled unless `enabled`.
fn interact(ui: &mut Ui, rect: Rect, id: egui::Id, enabled: bool) -> Response {
    super::interact_enabled(ui, rect, id, enabled)
}

/// `label` with its command's key, as AutoCAD's tooltips read "Grid (F7)".
fn tip(label: &str, cmd: Option<&str>) -> String {
    let label = crate::i18n::t(label);
    match cmd.and_then(cadcraft_engine::find_command).and_then(|c| c.shortcut) {
        Some(key) => format!("{label} ({key})"),
        None => label.to_string(),
    }
}

/// What an icon item shows and does.
struct Toggle {
    icon: Icon,
    label: &'static str,
    /// The command it needs (`None`: AsenaCAD has none yet).
    cmd: Option<&'static str>,
    /// What clicking runs (the command and its parameters).
    run: Option<(&'static str, Value)>,
    /// On (drawn filled); `None` for buttons that only open a menu or run a command.
    on: Option<bool>,
    menu: Option<Menu>,
    /// The whole item opens `menu` (no face of its own).
    menu_only: bool,
}

impl Toggle {
    fn new(icon: Icon, label: &'static str, cmd: Option<&'static str>, on: Option<bool>) -> Self {
        let run = cmd.map(|c| (c, json!({})));
        Toggle { icon, label, cmd, run, on, menu: None, menu_only: false }
    }

    fn menu(self, menu: Menu) -> Self {
        Toggle { menu: Some(menu), ..self }
    }

    fn menu_only(self, menu: Menu) -> Self {
        Toggle { menu: Some(menu), menu_only: true, run: None, ..self }
    }
}

/// The icon items, by key.
fn toggle_of(app: &CadApp, key: &str) -> Option<Toggle> {
    use cadcraft_engine::snap::mode;
    let s = &app.session.settings;
    let doc = app.session.doc().ok();
    let anno_all = doc.is_some_and(|d| d.anno_all_visible());
    let infer = doc.is_some_and(|d| d.parametric.settings.infer);
    let t = match key {
        "grid" => Toggle::new(Icon::Grid, "Grid", Some("grid"), Some(s.gridmode)),
        "snap" => Toggle::new(Icon::Snap, "Snap Mode", Some("snap"), Some(s.snapmode)).menu(Menu::Snap),
        "infer" => Toggle {
            run: Some(("constraintinfer", json!({ "value": u8::from(!infer) }))),
            ..Toggle::new(Icon::Constraint, "Infer Constraints", Some("constraintinfer"), Some(infer))
        },
        "dynmode" => Toggle::new(Icon::DynInput, "Dynamic Input", Some("dynmode"), Some(s.dynmode)),
        "ortho" => Toggle::new(Icon::Ortho, "Ortho Mode", Some("ortho"), Some(s.orthomode)),
        "polar" => Toggle::new(Icon::Polar, "Polar Tracking", Some("polar"), Some(s.polarmode)).menu(Menu::Polar),
        "isodraft" => Toggle::new(Icon::Isodraft, "Isometric Drafting", Some("isodraft"), Some(s.isodraft)).menu(Menu::Isodraft),
        "otrack" => Toggle::new(Icon::OTrack, "Object Snap Tracking", Some("otrack"), Some(s.otrack)),
        "osnap" => Toggle::new(Icon::Osnap, "2D Object Snap", Some("osnap"), Some(s.osmode & mode::OFF == 0 && s.osmode != 0)).menu(Menu::Osnap),
        "lineweight" => Toggle::new(Icon::Lineweight, "LineWeight", Some("lwdisplay"), Some(s.lwdisplay)),
        "transparency" => Toggle::new(Icon::Transparency, "Transparency", Some("transparencydisplay"), Some(s.transparency_display)),
        "selcycling" => Toggle::new(Icon::SelCycling, "Selection Cycling", Some("selectioncycling"), Some(s.selection_cycling)),
        "osnap3d" => Toggle::new(Icon::Osnap3d, "3D Object Snap", Some("3dosnap"), Some(false)),
        "ducs" => Toggle::new(Icon::DynUcs, "Dynamic UCS", None, Some(false)),
        "selfilter" => Toggle::new(Icon::SelFilter, "Selection Filtering", None, Some(false)),
        "gizmo" => Toggle::new(Icon::Gizmo, "Gizmo", None, Some(false)),
        "annovisibility" => Toggle::new(Icon::Annotation, "Annotation Visibility", Some("annovisible.toggle"), Some(anno_all)),
        "autoscale" => Toggle {
            run: Some(("setvar", json!({ "name": "ANNOAUTOSCALE", "value": if s.annoautoscale { -4 } else { 4 } }))),
            ..Toggle::new(Icon::AutoScale, "AutoScale", Some("setvar"), Some(s.annoautoscale))
        },
        "workspace" => Toggle::new(Icon::Gear, "Workspace Switching", Some("wscurrent"), None).menu_only(Menu::Workspace),
        "annomonitor" => Toggle::new(Icon::AnnoMonitor, "Annotation Monitor", Some("annomonitor"), Some(false)),
        "quickprops" => Toggle::new(Icon::QuickProps, "Quick Properties", Some("qpmode"), Some(s.qpmode)),
        "lockui" => Toggle::new(Icon::Lock, "Lock UI", Some("lockui"), Some(false)),
        "isolate" => Toggle::new(Icon::Isolate, "Isolate Objects", Some("isolateobjects"), None).menu_only(Menu::Isolate),
        "graphics" => Toggle::new(Icon::GraphicsPerf, "Graphics Performance", Some("graphicsconfig"), None),
        "cleanscreen" => Toggle::new(Icon::CleanScreen, "Clean Screen", Some("cleanscreenon"), None),
        _ => return None,
    };
    Some(t)
}

/// Draw item `it` ending at `right`; returns its left edge.
fn item_ui(app: &mut CadApp, ui: &mut Ui, it: &Item, right: f32, cy: f32) -> f32 {
    match it.key {
        "coordinates" => coordinates(app, ui, right, cy),
        "modelspace" => space_button(app, ui, right, cy),
        "units" => units_button(app, ui, right, cy),
        "annoscale" => match app.session.layout_space() {
            cadcraft_doc::Space::Model => super::annoscale::status(app, ui, right, cy) + 4.0,
            cadcraft_doc::Space::Paper(_) => crate::vpscale::status(app, ui, right, cy),
        },
        key => match toggle_of(app, key) {
            Some(tg) => toggle_ui(app, ui, &tg, right, cy),
            None => right,
        },
    }
}

fn paint_fill(ui: &Ui, rect: Rect, on: bool, resp: &Response) {
    let t = Tokens::get();
    if on {
        ui.painter().rect_filled(rect, 2.0, t.toggle_on.gamma_multiply(0.55));
    }
    if resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_pressed);
    } else if resp.hovered() && resp.enabled() {
        ui.painter().rect_filled(rect, 2.0, if on { t.toggle_on.gamma_multiply(0.75) } else { t.ribbon_hover });
    }
}

/// An icon item: a toggle (filled while on), a button, or a menu; with its ▾ menu when it has
/// one.
fn toggle_ui(app: &mut CadApp, ui: &mut Ui, tg: &Toggle, right: f32, cy: f32) -> f32 {
    let why = tg.cmd.map_or(Availability::Missing, |c| avail::availability(ui.ctx(), &app.session, c));
    let enabled = why.is_ready();
    let arrow_w = if tg.menu.is_some_and(Menu::has_arrow) { ARROW_W } else { 0.0 };
    let face = Rect::from_min_size(pos2(right - arrow_w - SLOT, cy - SLOT / 2.0), vec2(SLOT, SLOT));
    let arrow = Rect::from_min_max(pos2(face.right(), face.top()), pos2(right, face.bottom()));
    let id = ui.id().with(("status", tg.label));
    let on = tg.on == Some(true);
    if tg.menu_only {
        let whole = face.union(arrow);
        let resp = interact(ui, whole, id, enabled);
        paint_fill(ui, whole, false, &resp);
        icons::paint(ui.painter(), Rect::from_center_size(face.center(), vec2(GLYPH, GLYPH)), tg.icon, !enabled);
        if tg.menu.is_some_and(|m| m.has_arrow()) {
            icons::paint(ui.painter(), Rect::from_center_size(arrow.center(), vec2(9.0, 9.0)), Icon::ChevronDown, !enabled);
        }
        icons::describe_control(ui, &resp, tg.label, None, false);
        let resp = resp.on_hover_text(tip(tg.label, tg.cmd)).on_disabled_hover_text(why.disabled_text());
        if let Some(menu) = tg.menu {
            statusmenus::popup(app, &resp, menu);
        }
        return whole.left();
    }
    let resp = interact(ui, face, id, enabled);
    paint_fill(ui, face, on, &resp);
    icons::paint(ui.painter(), Rect::from_center_size(face.center(), vec2(GLYPH, GLYPH)), tg.icon, !enabled);
    icons::describe_control(ui, &resp, tg.label, tg.on, false);
    let resp = resp.on_hover_text(tip(tg.label, tg.cmd)).on_disabled_hover_text(why.disabled_text());
    if resp.clicked()
        && let Some((cmd, params)) = &tg.run
    {
        // Errors are echoed to the command history by `run`.
        let _ = app.run(cmd, params.clone());
    }
    if let Some(menu) = tg.menu {
        let label = crate::tf!("{label} options", label = crate::i18n::t(tg.label));
        let aresp = interact(ui, arrow, id.with("menu"), enabled);
        paint_fill(ui, arrow, false, &aresp);
        icons::paint(ui.painter(), Rect::from_center_size(arrow.center(), vec2(9.0, 9.0)), Icon::ChevronDown, !enabled);
        icons::describe_control(ui, &aresp, &label, None, false);
        let aresp = aresp.on_hover_text(&label).on_disabled_hover_text(why.disabled_text());
        statusmenus::popup(app, &aresp, menu);
    }
    face.left()
}

/// A text item `text` ending at `right`: its rect and response.
fn text_button(ui: &mut Ui, text: &str, label: &str, right: f32, cy: f32) -> (Rect, Response) {
    let t = Tokens::get();
    let font = crate::theme::ribbon_font(ui.ctx());
    let g = ui.painter().layout_no_wrap(text.to_string(), font, t.text);
    let rect = Rect::from_min_max(pos2(right - g.size().x - 12.0, cy - SLOT / 2.0), pos2(right, cy + SLOT / 2.0));
    let resp = ui.interact(rect, ui.id().with(("status", label)), Sense::click());
    paint_fill(ui, rect, false, &resp);
    ui.painter().galley(pos2(rect.left() + 6.0, cy - g.size().y / 2.0), g, t.text);
    icons::describe_control(ui, &resp, label, None, false);
    (rect, resp)
}

/// MODEL / PAPER. In model space MODEL goes back to the last layout; on a layout it switches
/// between paper space and model space in a viewport (PSPACE / MSPACE).
fn space_button(app: &mut CadApp, ui: &mut Ui, right: f32, cy: f32) -> f32 {
    let paper = matches!(app.session.layout_space(), cadcraft_doc::Space::Paper(_));
    let in_viewport = app.session.state().is_ok_and(|st| st.mspace.is_some());
    let label = if !paper || in_viewport { MODEL } else { PAPER };
    let (rect, resp) = text_button(ui, crate::i18n::t(label), label, right, cy);
    if resp.on_hover_text(crate::tl!("Model or Paper Space")).clicked() {
        if paper {
            let _ = app.run(if in_viewport { "pspace" } else { "mspace" }, json!({}));
            app.canvas.list = None;
        } else if let Some(name) = super::layouts::return_layout(app, ui.ctx()) {
            let _ = app.run("layout.set", json!({ "name": name }));
        }
    }
    rect.left()
}

/// The drawing's length units (LUNITS); opens the Drawing Units dialog.
fn units_button(app: &mut CadApp, ui: &mut Ui, right: f32, cy: f32) -> f32 {
    let lunits = app.session.doc().map(|d| d.header.i64("LUNITS", 2)).unwrap_or(2);
    let name = match lunits {
        1 => crate::tl!("Scientific"),
        3 => crate::tl!("Engineering"),
        4 => crate::tl!("Architectural"),
        5 => crate::tl!("Fractional"),
        _ => crate::tl!("Decimal"),
    };
    let (rect, resp) = text_button(ui, name, "Units", right, cy);
    if resp.on_hover_text(crate::tl!("Units")).clicked() {
        app.start("units");
    }
    rect.left()
}

/// The cursor's coordinates in the drawing's length units.
fn coordinates(app: &mut CadApp, ui: &mut Ui, right: f32, cy: f32) -> f32 {
    let t = Tokens::get();
    let (lu, lp) = app.session.doc().map(|d| (d.header.i64("LUNITS", 2), d.header.i64("LUPREC", 4))).unwrap_or((2, 4));
    let f = |v: f64| cadcraft_engine::units::format_distance(v, lu, lp);
    let text = app.canvas.cursor.map_or_else(String::new, |c| format!("{}, {}, {}", f(c.x), f(c.y), f(0.0)));
    let font = crate::theme::ribbon_font(ui.ctx());
    let g = ui.painter().layout_no_wrap(text, font, t.text);
    let w = (g.size().x + 12.0).clamp(150.0, 260.0);
    let rect = Rect::from_min_max(pos2(right - w, cy - SLOT / 2.0), pos2(right, cy + SLOT / 2.0));
    ui.painter().with_clip_rect(rect).galley(pos2(rect.right() - 6.0 - g.size().x, cy - g.size().y / 2.0), g, t.text);
    let resp = ui.interact(rect, ui.id().with("status_coordinates"), Sense::hover());
    icons::describe_control(ui, &resp, "Coordinates", None, false);
    rect.left()
}

/// The Customization "≡": AutoCAD's check list of every item, grouped; checking one shows it.
fn customization(app: &mut CadApp, ui: &mut Ui, right: f32, cy: f32) -> f32 {
    let rect = Rect::from_min_size(pos2(right - SLOT, cy - SLOT / 2.0), vec2(SLOT, SLOT));
    let resp = interact(ui, rect, ui.id().with("status_customization"), true);
    paint_fill(ui, rect, false, &resp);
    icons::paint(ui.painter(), Rect::from_center_size(rect.center(), vec2(GLYPH, GLYPH)), Icon::Menu, false);
    icons::describe_control(ui, &resp, CUSTOMIZATION, None, false);
    let resp = resp.on_hover_text(crate::i18n::t(CUSTOMIZATION));
    let mut toggle = None;
    statusmenus::above(&resp).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        let mut group = None;
        for it in ITEMS {
            if group.is_some_and(|g| g != it.group) {
                ui.add_space(2.0);
                ui.separator();
                ui.add_space(2.0);
            }
            group = Some(it.group);
            if super::check_row(ui, it.label, Some(is_shown(app, it.key)), true, statusmenus::ROW).clicked() {
                toggle = Some(it.key);
            }
        }
    });
    if let Some(key) = toggle {
        let _ = app.run("ui.statusbar", json!({ "item": key }));
    }
    rect.left()
}
