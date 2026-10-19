//! The controls over the drawing area, after AutoCAD's default workspace: the command line bar
//! (grip, close, Customize, recent commands, input, history toggle), the navigation bar below
//! the ViewCube and the drawing window's buttons at the top-right corner.

use asenacad_engine::Session;
use asenacad_ui_egui::{CadApp, Services, cmdbar, navbar, viewcube};
use egui::accesskit::Role;
use egui::{Event, Modifiers, PointerButton, Pos2, Rect, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn harness() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.start("ui.sample");
    app.integrated_titlebar = true;
    let mut h = Harness::builder().with_size(vec2(1906.0, 1026.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h
}

fn canvas(h: &Harness<'_, CadApp>) -> Rect {
    h.state().canvas.rect.expect("the drawing area is laid out")
}

fn rect_of(h: &Harness<'_, CadApp>, label: &str) -> Rect {
    h.get_by_label(label).rect()
}

/// A real pointer click at `at` (move, press, release in separate frames).
fn click_at(h: &mut Harness<'_, CadApp>, at: Pos2) {
    h.event(Event::PointerMoved(at));
    h.run_steps(1);
    for pressed in [true, false] {
        h.event(Event::PointerButton { pos: at, button: PointerButton::Primary, pressed, modifiers: Modifiers::default() });
        h.run_steps(1);
    }
    h.run_steps(2);
}

const BAR: [&str; 6] = [cmdbar::GRIP, cmdbar::CLOSE, cmdbar::CUSTOMIZE, cmdbar::RECENT_COMMANDS, cmdbar::INPUT, cmdbar::HISTORY_TOGGLE];

#[test]
fn the_command_line_bar_has_autocads_controls_in_order_and_size() {
    let h = harness();
    let c = canvas(&h);
    let rects: Vec<Rect> = BAR.iter().map(|l| rect_of(&h, l)).collect();
    assert!(rects.windows(2).all(|w| w[0].left() < w[1].left()), "AutoCAD's order: {rects:?}");
    assert_eq!(h.get_by_label(cmdbar::INPUT).accesskit_node().role(), Role::TextInput);
    let bar = cmdbar::bar_rect(c);
    // 60% of the drawing area's width, 27 tall, centred, just above its bottom edge.
    assert!((bar.width() - 0.6 * c.width()).abs() < 1.0 && bar.height() == cmdbar::BAR_H, "{bar:?} in {c:?}");
    assert!((bar.center().x - c.center().x).abs() < 0.5);
    assert!(c.bottom() - bar.bottom() > 1.0 && c.bottom() - bar.bottom() < 6.0);
    assert!(rects.iter().all(|r| bar.expand(0.5).contains_rect(*r)), "every control is on the bar");
    // Close and Customize are 20 × 20, the recent-commands button 32 × 22, the history toggle 15 wide.
    assert_eq!((rects[1].size(), rects[2].size(), rects[3].size()), (vec2(20.0, 20.0), vec2(20.0, 20.0), vec2(32.0, 22.0)));
    assert_eq!(rects[5].width(), 15.0);
    // The input field fills the rest.
    assert!(rects[4].width() > bar.width() - 120.0);
}

#[test]
fn close_hides_the_command_line_and_commandline_brings_it_back() {
    let mut h = harness();
    h.get_by_label(cmdbar::CLOSE).click();
    h.run_steps(3);
    assert!(!h.state().ui.show_command_line);
    assert!(h.query_by_label(cmdbar::INPUT).is_none());
    assert_eq!(h.state_mut().run("commandline", serde_json::Value::Null), Ok(json!({"on": true})));
    h.run_steps(3);
    assert!(h.query_by_label(cmdbar::INPUT).is_some());
    assert_eq!(h.state_mut().run("commandlinehide", serde_json::Value::Null), Ok(json!({"on": false})));
    // Ctrl+9 (the Command Line toggle) shows it again.
    let _ = h.state_mut().run("ui.toggle.cmdline", serde_json::Value::Null);
    assert!(h.state().ui.show_command_line);
}

#[test]
fn recent_commands_lists_what_ran_and_runs_it_again() {
    let mut h = harness();
    h.state_mut().cmdline("circle 0,0 5");
    h.run_steps(2);
    h.get_by_label(cmdbar::RECENT_COMMANDS).click();
    h.run_steps(3);
    h.get_by_label("CIRCLE").click();
    h.run_steps(3);
    assert_eq!(h.state().session.running.as_ref().map(|r| r.id.as_str()), Some("circle"));
}

#[test]
fn the_history_toggle_opens_and_closes_the_history() {
    let mut h = harness();
    assert!(!h.state().cmd.expanded);
    h.get_by_label(cmdbar::HISTORY_TOGGLE).click();
    h.run_steps(3);
    assert!(h.state().cmd.expanded);
    h.get_by_label(cmdbar::HISTORY_TOGGLE).click();
    h.run_steps(3);
    assert!(!h.state().cmd.expanded);
}

const NAV: [&str; 10] = [
    "Close Navigation Bar",
    "Full Navigation Wheel",
    "Full Navigation Wheel options",
    "Pan",
    "Zoom Extents",
    "Zoom options",
    "Orbit",
    "Orbit options",
    "ShowMotion",
    "Customize Navigation Bar",
];

/// The navigation bar's node labelled `label` (the ribbon has a Pan, Orbit... of its own).
fn nav_node<'a>(h: &'a Harness<'_, CadApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let strip = navbar::rect(canvas(h));
    let mut nodes: Vec<_> = h.query_all_by_label(label).filter(|n| strip.contains_rect(n.rect())).collect();
    assert_eq!(nodes.len(), 1, "one {label} on the navigation bar");
    nodes.remove(0)
}

#[test]
fn the_navigation_bar_sits_below_the_viewcube_with_its_tools_in_order() {
    let h = harness();
    let c = canvas(&h);
    let strip = navbar::rect(c);
    let cube = viewcube::Layout::new(c);
    assert!(c.contains_rect(strip), "{strip:?} inside {c:?}");
    assert!(strip.top() > cube.pill.bottom() && !strip.intersects(cube.bounds));
    assert!(c.right() - strip.right() < 10.0, "at the right edge");
    let tops: Vec<f32> = NAV.iter().map(|l| nav_node(&h, l).rect().top()).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "top to bottom: {tops:?}");
    for missing in ["Full Navigation Wheel", "ShowMotion", "Customize Navigation Bar"] {
        assert!(nav_node(&h, missing).accesskit_node().is_disabled(), "{missing} has no command yet");
    }
    assert!(!nav_node(&h, "Pan").accesskit_node().is_disabled());
}

#[test]
fn navigation_bar_tools_run_commands_and_keep_clicks_off_the_drawing() {
    let mut h = harness();
    let strip = navbar::rect(canvas(&h));
    // On the strip beside a tool: nothing happens in the drawing.
    let pan = navbar::tools(strip).iter().find(|(t, _)| *t == navbar::Tool::Pan).map(|(_, r)| *r).expect("Pan");
    let beside = egui::pos2(strip.left() + 1.5, pan.center().y);
    assert!(navbar::tools(strip).iter().all(|(_, r)| !r.contains(beside)));
    click_at(&mut h, beside);
    assert!(h.state().session.pending_window.is_none(), "the click stayed on the navigation bar");
    assert!(h.state().session.selection().is_empty());
    nav_node(&h, "Pan").click();
    h.run_steps(3);
    assert_eq!(h.state().session.running.as_ref().map(|r| r.id.as_str()), Some("pan"));
    h.state_mut().session.cancel();
    // Zoom ▾ ▸ Zoom Window runs it, and the face shows it from then on.
    nav_node(&h, "Zoom options").click();
    h.run_steps(3);
    h.query_all_by_label("Zoom Window").find(|n| !strip.contains_rect(n.rect())).expect("the menu row").click();
    h.run_steps(3);
    assert_eq!(h.state().session.running.as_ref().map(|r| r.id.as_str()), Some("zoom.window"));
    h.state_mut().session.cancel();
    h.run_steps(2);
    assert!(nav_node(&h, "Zoom Window").rect().top() > nav_node(&h, "Pan").rect().top());
    // ⊗ hides the bar; NAVBAR brings it back.
    nav_node(&h, "Close Navigation Bar").click();
    h.run_steps(3);
    assert!(!h.state().ui.show_navbar && h.query_by_label("ShowMotion").is_none());
    assert_eq!(h.state_mut().run("navbar", json!({"on": true})), Ok(json!({"on": true})));
}

#[test]
fn the_drawing_window_buttons_sit_at_the_top_right_corner() {
    let h = harness();
    let c = canvas(&h);
    let rects: Vec<Rect> = ["Minimize Drawing", "Restore Drawing", "Close Drawing"].iter().map(|l| rect_of(&h, l)).collect();
    assert!(rects.windows(2).all(|w| w[0].right() <= w[1].left()), "{rects:?}");
    for r in &rects {
        assert!(c.contains_rect(*r) && r.top() - c.top() < 24.0 && c.right() - r.right() < 70.0, "{r:?} at the corner of {c:?}");
    }
    assert!(h.get_by_label("Minimize Drawing").accesskit_node().is_disabled());
    assert!(h.get_by_label("Restore Drawing").accesskit_node().is_disabled());
    assert!(!h.get_by_label("Close Drawing").accesskit_node().is_disabled());
    // Clear of the ViewCube and the navigation bar.
    let strip = navbar::rect(c);
    assert!(rects.iter().all(|r| !r.intersects(strip)));
}
