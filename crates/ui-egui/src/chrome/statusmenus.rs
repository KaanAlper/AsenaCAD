//! The menus of the status bar items' ▾ parts, after AutoCAD's: Snap Mode (Snap Settings...,
//! Polar Snap, Grid Snap), Polar Tracking (Tracking Settings..., the increment angles), Isometric
//! Drafting (the isoplanes), 2D Object Snap (Object Snap Settings..., the running snap modes as
//! check items bound to OSMODE) and Isolate Objects. Every entry runs a command.

use egui::{Popup, PopupCloseBehavior, RectAlign, Response, Ui, pos2, vec2};
use serde_json::{Value, json};

use crate::CadApp;
use crate::theme::Tokens;

/// A menu row's size.
pub const ROW: egui::Vec2 = egui::Vec2 { x: 230.0, y: 22.0 };

const SNAP_SETTINGS: &str = "Snap Settings...";
const TRACKING_SETTINGS: &str = "Tracking Settings...";
const OSNAP_SETTINGS: &str = "Object Snap Settings...";
const POLAR_SNAP: &str = "Polar Snap";
const GRID_SNAP: &str = "Grid Snap";
const ISOPLANES: [(&str, &str); 3] = [("left", "Isoplane Left"), ("top", "Isoplane Top"), ("right", "Isoplane Right")];
const ISOLATE: [(&str, &str); 3] =
    [("isolateobjects", "Isolate Objects"), ("hideobjects", "Hide Objects"), ("unisolateobjects", "End Object Isolation")];
/// Polar Tracking's increment angles (POLARANG), as AutoCAD lists them.
const POLAR_ANGLES: [(f64, &str, &str); 8] = [
    (90.0, "90", ", 180, 270, 360..."),
    (45.0, "45", ", 90, 135, 180..."),
    (30.0, "30", ", 60, 90, 120..."),
    (22.5, "23", ", 45, 68, 90..."),
    (18.0, "18", ", 36, 54, 72..."),
    (15.0, "15", ", 30, 45, 60..."),
    (10.0, "10", ", 20, 30, 40..."),
    (5.0, "5", ", 10, 15, 20..."),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Menu {
    Snap,
    Polar,
    Isodraft,
    Osnap,
    Workspace,
    Isolate,
}

impl Menu {
    /// Whether the item shows a ▾ for this menu (Isolate Objects opens it from its icon).
    pub fn has_arrow(self) -> bool {
        self != Menu::Isolate
    }
}

/// The menus' labels, for the translation catalogs.
pub fn labels() -> impl Iterator<Item = &'static str> {
    [SNAP_SETTINGS, TRACKING_SETTINGS, OSNAP_SETTINGS, POLAR_SNAP, GRID_SNAP]
        .into_iter()
        .chain(ISOPLANES.iter().map(|(_, l)| *l))
        .chain(ISOLATE.iter().map(|(_, l)| *l))
}

/// A popup opened by `resp`, above the status bar.
pub fn above(resp: &Response) -> Popup<'_> {
    const ALTERNATIVES: [RectAlign; 3] = [RectAlign::TOP_START, RectAlign::TOP, RectAlign::BOTTOM_END];
    Popup::from_toggle_button_response(resp).align(RectAlign::TOP_END).align_alternatives(&ALTERNATIVES).gap(2.0)
}

/// Show `menu` when `resp` opens it, and run what is chosen.
pub fn popup(app: &mut CadApp, resp: &Response, menu: Menu) {
    let mut chosen: Option<(&'static str, Value)> = None;
    above(resp).close_behavior(PopupCloseBehavior::CloseOnClick).show(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        match menu {
            Menu::Snap => snap(app, ui, &mut chosen),
            Menu::Polar => polar(app, ui, &mut chosen),
            Menu::Isodraft => isodraft(app, ui, &mut chosen),
            Menu::Osnap => osnap(app, ui, &mut chosen),
            Menu::Isolate => isolate(app, ui, &mut chosen),
            Menu::Workspace => {}
        }
    });
    if let Some((cmd, params)) = chosen {
        app.cmd.buffer.clear();
        if params.is_null() {
            // Typed-like: dialogs open, Isolate Objects asks for the objects.
            app.start(cmd);
        } else {
            // Errors are echoed to the command history by `run`.
            let _ = app.run(cmd, params);
        }
    }
}

fn row(ui: &mut Ui, label: &str, checked: Option<bool>) -> bool {
    super::check_row(ui, label, checked, true, ROW).clicked()
}

fn separator(ui: &mut Ui) {
    ui.add_space(2.0);
    ui.separator();
    ui.add_space(2.0);
}

/// "... Settings...": the Drafting Settings dialog.
fn settings(ui: &mut Ui, label: &str, chosen: &mut Option<(&'static str, Value)>) {
    if row(ui, label, None) {
        *chosen = Some(("ui.dialog.dsettings", Value::Null));
    }
    separator(ui);
}

fn snap(app: &CadApp, ui: &mut Ui, chosen: &mut Option<(&'static str, Value)>) {
    settings(ui, SNAP_SETTINGS, chosen);
    let polar = app.session.settings.polarsnap;
    for (label, on, value) in [(POLAR_SNAP, polar, 1), (GRID_SNAP, !polar, 0)] {
        if row(ui, label, Some(on)) {
            *chosen = Some(("setvar", json!({ "name": "SNAPTYPE", "value": value })));
        }
    }
}

fn polar(app: &CadApp, ui: &mut Ui, chosen: &mut Option<(&'static str, Value)>) {
    settings(ui, TRACKING_SETTINGS, chosen);
    let current = app.session.settings.polarang.to_degrees();
    for (angle, first, rest) in POLAR_ANGLES {
        if angle_row(ui, first, rest, (current - angle).abs() < 1e-6) {
            *chosen = Some(("setvar", json!({ "name": "POLARANG", "value": angle })));
        }
    }
}

/// A Polar Tracking angle: the increment in bold, then the series.
fn angle_row(ui: &mut Ui, first: &str, rest: &str, checked: bool) -> bool {
    let (resp, color) = super::menu_row_frame(ui, Some(checked), true, ROW);
    let font = crate::theme::ribbon_font(ui.ctx());
    let bold = ui.painter().layout_no_wrap(first.to_string(), font.clone(), color);
    let tail = ui.painter().layout_no_wrap(rest.to_string(), font, Tokens::get().text_dim);
    let at = pos2(resp.rect.left() + super::MENU_TEXT_X, resp.rect.center().y - bold.size().y / 2.0);
    let w = bold.size().x;
    ui.painter().galley(at + vec2(0.6, 0.0), bold.clone(), color);
    ui.painter().galley(at, bold, color);
    ui.painter().galley(at + vec2(w + 0.6, 0.0), tail, color);
    crate::icons::describe_control(ui, &resp, &format!("{first}{rest}"), Some(checked), false);
    resp.clicked()
}

fn isodraft(app: &CadApp, ui: &mut Ui, chosen: &mut Option<(&'static str, Value)>) {
    let s = &app.session.settings;
    for (plane, label) in ISOPLANES {
        let on = s.isodraft && s.isoplane.name().eq_ignore_ascii_case(plane);
        if row(ui, label, Some(on)) {
            *chosen = Some(("isodraft", json!({ "plane": plane })));
        }
    }
}

fn osnap(app: &CadApp, ui: &mut Ui, chosen: &mut Option<(&'static str, Value)>) {
    use cadcraft_engine::snap::mode;
    settings(ui, OSNAP_SETTINGS, chosen);
    let osmode = app.session.settings.osmode;
    for (bit, label) in mode::ALL {
        let on = osmode & bit != 0;
        if row(ui, label, Some(on)) {
            // Checking a mode also turns running object snaps on.
            let next = if on { osmode & !bit } else { (osmode | bit) & !mode::OFF };
            *chosen = Some(("osnap", json!({ "osmode": next })));
        }
    }
}

fn isolate(app: &CadApp, ui: &mut Ui, chosen: &mut Option<(&'static str, Value)>) {
    for (cmd, label) in ISOLATE {
        let ready = crate::ribbon::avail::availability(ui.ctx(), &app.session, cmd).is_ready();
        if super::check_row(ui, label, None, ready, ROW).clicked() {
            *chosen = Some((cmd, Value::Null));
        }
    }
}
