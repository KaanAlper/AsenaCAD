//! The controls over the drawing area, after AutoCAD's default workspace: the command line bar
//! (grip, close, Customize, recent commands, input, history toggle).

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services, cmdbar};
use egui::accesskit::Role;
use egui::{Rect, vec2};
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
