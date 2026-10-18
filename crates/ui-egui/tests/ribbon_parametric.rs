//! The Parametric ribbon's controls: constraint buttons start their commands, the Linear split
//! remembers its last entry, unbuilt items are disabled, the constraint mode toggles show
//! AutoCAD's default and the Parameters Manager toggle follows the manager.

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::parametric::PARAMETRIC;
use cadcraft_ui_egui::ribbon::{render_panel_for_test, widgets::LAUNCHER, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::{Role, Toggled};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn panel(label: &str) -> &'static Panel {
    PARAMETRIC.panels.iter().find(|p| p.label == label).expect("a Parametric panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &PARAMETRIC, panel), app);
    harness.run_steps(3);
    harness
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn geometric_buttons_start_their_constraint_commands() {
    for (label, cmd) in
        [("Coincident", "gccoincident"), ("Symmetric", "gcsymmetric"), ("Auto Constrain", "autoconstrain"), ("Show/Hide", "constraintbar")]
    {
        let mut h = harness("Geometric");
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(running(&h).as_deref(), Some(cmd), "{label}");
    }
}

#[test]
fn choosing_vertical_puts_it_on_the_linear_face() {
    let mut h = harness("Dimensional");
    h.get_by_label("Linear options").click();
    h.run_steps(3);
    let tops: Vec<f32> =
        ["Linear", "Horizontal", "Vertical"].iter().map(|l| h.query_all_by_label(l).map(|n| n.rect().top()).fold(f32::MIN, f32::max)).collect();
    assert!(tops[0] < tops[1] && tops[1] < tops[2], "entries in AutoCAD's order: {tops:?}");
    h.get_by_label("Vertical").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("dcvertical"));
    assert_eq!(RibbonState::split(&h.ctx, "Parametric/Dimensional/0", 0), 2);
    assert!(h.query_by_role_and_label(Role::Button, "Vertical").is_some(), "the face reads Vertical");
}

#[test]
fn convert_and_the_constraint_modes_are_disabled_and_dynamic_mode_is_on() {
    let h = harness("Dimensional");
    assert!(h.get_by_label("Convert").accesskit_node().is_disabled(), "DCCONVERT has no command yet");
    assert!(!h.get_by_label("Show All").accesskit_node().is_disabled());
    let dynamic = h.get_by_label("Dynamic Constraint Mode");
    assert!(dynamic.accesskit_node().is_disabled());
    assert_eq!(dynamic.accesskit_node().toggled(), Some(Toggled::True), "every constraint is dynamic, AutoCAD's default");
    assert_eq!(h.get_by_label("Annotational Constraint Mode").accesskit_node().toggled(), Some(Toggled::False));
}

#[test]
fn the_parameters_manager_toggle_opens_the_manager_and_shows_pressed() {
    let mut h = harness("Manage");
    let toggled = |h: &Harness<'_, CadApp>| h.get_by_label("Parameters Manager").accesskit_node().toggled();
    assert_eq!(toggled(&h), Some(Toggled::False));
    h.get_by_label("Parameters Manager").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("parameters"));
    assert_eq!(toggled(&h), Some(Toggled::True));
}

#[test]
fn the_geometric_launcher_opens_constraint_settings() {
    let mut h = harness("Geometric");
    let launcher = h.query_all_by_role(Role::Button).find(|n| n.rect().size() == LAUNCHER).expect("the panel's dialog launcher");
    assert_eq!(launcher.accesskit_node().label().as_deref(), Some("Constraint Settings, Geometric"));
    launcher.click();
    h.run_steps(3);
    assert!(h.state().ui.dialog.is_some(), "the launcher opens the Constraint Settings dialog");
}
