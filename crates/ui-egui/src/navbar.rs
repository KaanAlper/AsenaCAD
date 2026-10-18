//! The controls along the drawing area's right edge, after AutoCAD's default workspace:
//!
//! - The **navigation bar** (NAVBAR): a translucent strip below the ViewCube with, top to
//!   bottom, its close ⊗, the Full Navigation Wheel (and its ▾), Pan, Zoom (the last zoom used,
//!   with the zoom variants under its ▾), Orbit (and its ▾), ShowMotion and the bar's Customize ▾.
//! - The **drawing window's buttons** at the top-right corner: minimize, restore and close.
//!
//! Like the ViewCube, the strip registers one widget over its whole rectangle so clicks between
//! its tools never reach the drawing underneath, while the drawing keeps every other point.
//! Each tool runs a command; one AsenaCAD doesn't have yet shows disabled and says so.

use egui::{Rect, RectAlign, Sense, Stroke, pos2, vec2};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::ribbon::avail::{self, Availability};
use crate::theme::Tokens;

/// The strip's size (AutoCAD's 32 × 210 px), its gap from the drawing area's right edge and
/// below the ViewCube's WCS menu.
pub const WIDTH: f32 = 32.0;
pub const HEIGHT: f32 = 210.0;
const RIGHT_GAP: f32 = 4.0;
const BELOW_CUBE: f32 = 36.0;
const TOOL: egui::Vec2 = egui::Vec2 { x: 26.0, y: 24.0 };
const ARROW: egui::Vec2 = egui::Vec2 { x: 20.0, y: 9.0 };

const CLOSE: &str = "Close Navigation Bar";
const WHEEL: &str = "Full Navigation Wheel";
const SHOWMOTION: &str = "ShowMotion";
const CUSTOMIZE: &str = "Customize Navigation Bar";
const MINIMIZE: &str = "Minimize Drawing";
const RESTORE: &str = "Restore Drawing";
const CLOSE_DRAWING: &str = "Close Drawing";

/// Zoom's variants, as the ▾ lists them: (label, command, keyword when the variant is an option
/// of ZOOM itself).
const ZOOMS: [(&str, &str, Option<&str>); 11] = [
    ("Zoom Extents", "zoom.extents", None),
    ("Zoom Window", "zoom.window", None),
    ("Zoom Previous", "zoom.previous", None),
    ("Zoom Realtime", "zoom.realtime", None),
    ("Zoom All", "zoom.all", None),
    ("Zoom Dynamic", "zoom.dynamic", None),
    ("Zoom Scale", "zoom", Some("Scale")),
    ("Zoom Center", "zoom", Some("Center")),
    ("Zoom Object", "zoom.object", None),
    ("Zoom In", "zoom.in", None),
    ("Zoom Out", "zoom.out", None),
];
const ORBITS: [(&str, &str); 3] = [("Orbit", "3dorbit"), ("Free Orbit", "3dforbit"), ("Continuous Orbit", "3dcorbit")];

/// The labels the controls show, for the translation catalogs.
pub fn labels() -> impl Iterator<Item = &'static str> {
    [CLOSE, WHEEL, SHOWMOTION, CUSTOMIZE, MINIMIZE, RESTORE, CLOSE_DRAWING, "Zoom", "Pan"]
        .into_iter()
        .chain(ZOOMS.iter().map(|z| z.0))
        .chain(ORBITS.iter().map(|o| o.0))
}

/// The navigation bar's strip for the drawing area `canvas`: at its right edge, below the ViewCube.
/// In a short drawing area it moves up towards the WCS menu to stay inside, as far as that goes.
pub fn rect(canvas: Rect) -> Rect {
    let pill = crate::viewcube::Layout::new(canvas).pill.bottom();
    let fit = canvas.bottom() - 8.0 - HEIGHT;
    let top = (pill + BELOW_CUBE).min(fit).max(pill + 6.0);
    Rect::from_min_size(pos2(canvas.right() - RIGHT_GAP - WIDTH, top), vec2(WIDTH, HEIGHT))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tool {
    Close,
    Wheel,
    WheelMenu,
    Pan,
    Zoom,
    ZoomMenu,
    Orbit,
    OrbitMenu,
    ShowMotion,
    Customize,
}

/// Where each tool is in the strip `r`, top to bottom.
pub fn tools(r: Rect) -> [(Tool, Rect); 10] {
    let cx = r.center().x;
    let tool = |y: f32| Rect::from_center_size(pos2(cx, r.top() + y), TOOL);
    let arrow = |y: f32| Rect::from_center_size(pos2(cx, r.top() + y), ARROW);
    [
        (Tool::Close, Rect::from_min_size(pos2(r.right() - 13.0, r.top() + 3.0), vec2(10.0, 10.0))),
        (Tool::Wheel, tool(28.0)),
        (Tool::WheelMenu, arrow(45.0)),
        (Tool::Pan, tool(70.0)),
        (Tool::Zoom, tool(97.0)),
        (Tool::ZoomMenu, arrow(114.0)),
        (Tool::Orbit, tool(138.0)),
        (Tool::OrbitMenu, arrow(155.0)),
        (Tool::ShowMotion, tool(180.0)),
        (Tool::Customize, Rect::from_center_size(pos2(cx, r.top() + 199.0), vec2(20.0, 10.0))),
    ]
}

fn zoom_id() -> egui::Id {
    egui::Id::new("asenacad-navbar-zoom")
}

/// What a click on a tool, or a choice from its menu, runs.
enum Action {
    Start(&'static str),
    Keyword(&'static str, &'static str),
}

/// Draw the navigation bar over the drawing area `canvas` and handle its tools.
pub fn show(app: &mut CadApp, ui: &mut egui::Ui, canvas: Rect) {
    let t = Tokens::get();
    let strip = rect(canvas);
    let p = ui.painter_at(canvas);
    // One widget under the whole strip keeps the pointer off the drawing.
    let _ = ui.interact(strip, ui.id().with("navbar"), Sense::click_and_drag());
    p.rect_filled(strip, 5.0, t.navbar_bg);
    p.rect_stroke(strip, 5.0, Stroke::new(1.0, t.navbar_border), egui::StrokeKind::Inside);
    let zoom = ui.ctx().data(|d| d.get_temp::<usize>(zoom_id())).filter(|i| *i < ZOOMS.len()).unwrap_or(0);
    let (zoom_label, zoom_cmd, zoom_kw) = ZOOMS.get(zoom).copied().unwrap_or(ZOOMS[0]);
    let mut action = None;
    for (tool, r) in tools(strip) {
        let (label, cmd, icon) = match tool {
            Tool::Close => (CLOSE, Some("navbar"), None),
            Tool::Wheel => (WHEEL, Some("navswheel"), Some(Icon::NavWheel)),
            Tool::WheelMenu => (WHEEL, Some("navswheel"), None),
            Tool::Pan => ("Pan", Some("pan"), Some(Icon::Pan)),
            Tool::Zoom => (zoom_label, Some(zoom_cmd), Some(if zoom_cmd == "zoom.window" { Icon::ZoomWindow } else { Icon::ZoomExtents })),
            Tool::ZoomMenu => ("Zoom", Some("zoom"), None),
            Tool::Orbit => ("Orbit", Some("3dorbit"), Some(Icon::Orbit)),
            Tool::OrbitMenu => ("Orbit", Some("3dorbit"), None),
            Tool::ShowMotion => (SHOWMOTION, Some("navsmotion"), Some(Icon::ShowMotion)),
            Tool::Customize => (CUSTOMIZE, None, None),
        };
        let why = cmd.map_or(Availability::Missing, |c| avail::availability(ui.ctx(), &app.session, c));
        let enabled = why.is_ready();
        let resp = crate::chrome::interact_enabled(ui, r, ui.id().with(("navbar", tool as u8)), enabled);
        if resp.hovered() && enabled {
            p.rect_filled(r, 3.0, t.navbar_hover);
        }
        let ink = if enabled { t.navbar_icon } else { t.navbar_icon.gamma_multiply(0.4) };
        let menu = matches!(tool, Tool::WheelMenu | Tool::ZoomMenu | Tool::OrbitMenu | Tool::Customize);
        match (tool, icon) {
            (Tool::Close, _) => {
                p.circle_stroke(r.center(), 4.5, Stroke::new(1.0, ink));
                icons::paint_tinted(&p, r.shrink(2.0), Icon::Close, ink);
            }
            (_, Some(icon)) => icons::paint_tinted(&p, Rect::from_center_size(r.center(), vec2(18.0, 18.0)), icon, ink),
            _ if menu => {
                let c = r.center();
                p.add(egui::Shape::convex_polygon(vec![c + vec2(-3.5, -1.8), c + vec2(3.5, -1.8), c + vec2(0.0, 2.2)], ink, Stroke::NONE));
                if tool == Tool::Customize {
                    p.line_segment([c + vec2(-3.5, -4.0), c + vec2(3.5, -4.0)], Stroke::new(1.0, ink));
                }
            }
            _ => {}
        }
        let name = if menu && tool != Tool::Customize {
            crate::tf!("{label} options", label = crate::i18n::t(label))
        } else {
            crate::i18n::t(label).to_string()
        };
        icons::describe_control(ui, &resp, &name, None, false);
        let resp = resp.on_hover_text(&name).on_disabled_hover_text(why.disabled_text());
        match tool {
            Tool::Close if resp.clicked() => {
                let _ = app.run("navbar", serde_json::json!({ "on": false }));
            }
            Tool::Pan if resp.clicked() => action = Some(Action::Start("pan")),
            Tool::Zoom if resp.clicked() => action = Some(zoom_kw.map_or(Action::Start(zoom_cmd), |k| Action::Keyword(zoom_cmd, k))),
            Tool::Orbit if resp.clicked() => action = Some(Action::Start("3dorbit")),
            Tool::ZoomMenu => zoom_menu(app, ui, &resp, zoom, &mut action),
            Tool::OrbitMenu => orbit_menu(app, &resp, &mut action),
            _ => {}
        }
    }
    match action {
        Some(Action::Start(cmd)) => {
            app.cmd.buffer.clear();
            app.start(cmd);
        }
        Some(Action::Keyword(cmd, keyword)) => {
            app.cmd.buffer.clear();
            app.start(cmd);
            if let Err(e) = app.session.input(cadcraft_engine::Input::Keyword(keyword.into())) {
                app.session.echo(e.to_string());
            }
        }
        None => {}
    }
}

/// One row of a navigation bar menu.
fn row(ui: &mut egui::Ui, label: &str, checked: bool, enabled: bool) -> bool {
    crate::chrome::check_row(ui, label, Some(checked), enabled, vec2(190.0, 22.0)).clicked()
}

fn zoom_menu(app: &CadApp, ui: &egui::Ui, resp: &egui::Response, current: usize, action: &mut Option<Action>) {
    let mut chosen = None;
    egui::Popup::from_toggle_button_response(resp).align(RectAlign::LEFT_START).show(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for (i, (label, cmd, _)) in ZOOMS.iter().enumerate() {
            let ready = avail::availability(ui.ctx(), &app.session, cmd).is_ready();
            if row(ui, label, i == current, ready) {
                chosen = Some(i);
            }
        }
    });
    if let Some(i) = chosen
        && let Some((_, cmd, kw)) = ZOOMS.get(i)
    {
        // The face shows the zoom chosen last, like AutoCAD's.
        ui.ctx().data_mut(|d| d.insert_temp(zoom_id(), i));
        *action = Some(kw.map_or(Action::Start(cmd), |k| Action::Keyword(cmd, k)));
    }
}

fn orbit_menu(app: &CadApp, resp: &egui::Response, action: &mut Option<Action>) {
    egui::Popup::from_toggle_button_response(resp).align(RectAlign::LEFT_START).show(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        for (i, (label, cmd)) in ORBITS.iter().enumerate() {
            let ready = avail::availability(ui.ctx(), &app.session, cmd).is_ready();
            if row(ui, label, i == 0, ready) {
                *action = Some(Action::Start(cmd));
            }
        }
    });
}

/// The drawing window's minimize, restore and close buttons at the top-right corner of the
/// drawing area. AsenaCAD shows one drawing at a time filling the area, so minimize and restore
/// aren't available; close closes the drawing (asking about unsaved changes).
pub fn window_buttons(app: &mut CadApp, ui: &mut egui::Ui, canvas: Rect) {
    let t = Tokens::get();
    let mut close = false;
    for (i, (label, cmd, icon)) in
        [(MINIMIZE, None, Icon::VpMinimize), (RESTORE, None, Icon::VpRestore), (CLOSE_DRAWING, Some("close"), Icon::Close)].into_iter().enumerate()
    {
        let r = Rect::from_center_size(pos2(canvas.right() - 56.0 + 20.0 * i as f32, canvas.top() + 15.0), vec2(16.0, 16.0));
        let why = cmd.map_or(Availability::Missing, |c| avail::availability(ui.ctx(), &app.session, c));
        let enabled = why.is_ready();
        let resp = crate::chrome::interact_enabled(ui, r, ui.id().with(("drawing_window", i)), enabled);
        if resp.hovered() && enabled {
            ui.painter().rect_filled(r, 2.0, t.ribbon_hover);
        }
        let ink = if enabled { t.canvas_ink } else { t.canvas_ink.gamma_multiply(0.45) };
        icons::paint_tinted(ui.painter(), r.shrink(1.0), icon, ink);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.on_hover_text(crate::i18n::t(label)).on_disabled_hover_text(why.disabled_text()).clicked() {
            close = true;
        }
    }
    if close {
        app.start("close");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_strip_sits_below_the_viewcube_inside_the_drawing_area() {
        let canvas = Rect::from_min_size(pos2(4.0, 186.0), vec2(1898.0, 803.0));
        let r = rect(canvas);
        let cube = crate::viewcube::Layout::new(canvas);
        assert!(canvas.contains_rect(r));
        assert!(r.top() > cube.pill.bottom() && !r.intersects(cube.bounds));
        assert_eq!((r.width(), r.height()), (WIDTH, HEIGHT));
        // AutoCAD's strip ends 4 px from the drawing area's right edge.
        assert_eq!(canvas.right() - r.right(), RIGHT_GAP);
        // Every tool is inside the strip, top to bottom, none overlapping the next.
        let tools = tools(r);
        assert!(tools.iter().all(|(_, t)| r.contains_rect(*t)));
        assert!(tools.windows(2).skip(1).all(|w| w[0].1.bottom() <= w[1].1.top()), "{tools:?}");
    }
}
