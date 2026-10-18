//! The Home ribbon's controls: buttons start their commands, split buttons list AutoCAD's
//! variants and remember the last one, unbuilt items are disabled, combo boxes show the drawing's
//! current values and toggles show their state.

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::home::HOME;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::{render_panel_for_test, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::{Role, Toggled};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn panel(label: &str) -> &'static Panel {
    HOME.panels.iter().find(|p| p.label == label).expect("a Home panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness =
        Harness::builder().with_size(vec2(900.0, 420.0)).build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, panel), app);
    harness.run_steps(3);
    harness
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn clicking_line_starts_the_line_command() {
    let mut h = harness("Draw");
    h.get_by_label("Line").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("line"));
}

#[test]
fn the_circle_menu_lists_autocads_variants_in_order_with_separators() {
    let mut h = harness("Draw");
    h.get_by_label("Circle options").click();
    h.run_steps(3);
    let entries = ["Center, Radius", "Center, Diameter", "2-Point", "3-Point", "Tan, Tan, Radius", "Tan, Tan, Tan"];
    let tops: Vec<f32> = entries.iter().map(|l| h.get_by_label(l).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "entries in AutoCAD's order: {tops:?}");
    let step = tops[1] - tops[0];
    assert!(tops[2] - tops[1] > step + 2.0, "a separator before 2-Point");
    assert!((tops[3] - tops[2] - step).abs() < 1.0, "no separator between 2-Point and 3-Point");
    assert!(tops[4] - tops[3] > step + 2.0, "a separator before Tan, Tan, Radius");
}

#[test]
fn choosing_2_point_keeps_the_circle_label_and_starts_circle_2p() {
    let mut h = harness("Draw");
    h.get_by_label("Circle options").click();
    h.run_steps(3);
    h.get_by_label("2-Point").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("circle.2p"));
    assert!(h.query_by_label("2-Point").is_none(), "the menu closes");
    assert!(h.query_by_role_and_label(Role::Button, "Circle").is_some(), "Circle keeps its label");
    let remembered = RibbonState::split(&h.ctx, "Draw/2", 0);
    assert_eq!(remembered, 3, "the face (its icon) now follows the 2-Point entry");

    // The face runs the remembered variant.
    h.state_mut().session.cancel();
    h.get_by_label("Circle").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("circle.2p"));
}

#[test]
fn choosing_extend_in_the_trim_menu_puts_extend_on_the_face() {
    let mut h = harness("Modify");
    h.get_by_label("Trim options").click();
    h.run_steps(3);
    h.get_by_label("Extend").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("extend"));
    assert!(h.query_by_label("Trim").is_none(), "the face no longer reads Trim");
    assert!(h.query_by_role_and_label(Role::Button, "Extend").is_some(), "the face reads Extend");
    assert!(h.query_by_label("Extend options").is_some());
}

#[test]
fn an_item_without_a_command_is_disabled() {
    let mut h = harness("Modify");
    h.get_by_label("Set to ByLayer").click();
    h.run_steps(3);
    assert!(h.get_by_label("Set to ByLayer").accesskit_node().is_disabled());
    assert_eq!(running(&h), None, "a disabled item runs nothing");
    assert!(!h.get_by_label("Change Space").accesskit_node().is_disabled(), "slide-out items with a command are enabled");
}

#[test]
fn the_layer_combo_shows_the_current_layer() {
    let h = harness("Layers");
    let combo = h.get_by_role_and_label(Role::ComboBox, "Layer");
    assert_eq!(combo.value().as_deref(), Some("0"));
    assert!(!combo.accesskit_node().is_disabled());
}

#[test]
fn the_properties_combos_show_the_current_properties() {
    let h = harness("Properties");
    let value = |label: &str| h.get_by_role_and_label(Role::ComboBox, label).value();
    assert_eq!(value("Object Color").as_deref(), Some("ByLayer"));
    assert_eq!(value("Linetype").as_deref(), Some("ByLayer"));
    assert_eq!(value("Lineweight").as_deref(), Some("ByLayer"));
    // No binding yet: shown disabled with its value.
    assert!(h.get_by_role_and_label(Role::ComboBox, "Transparency Mode").accesskit_node().is_disabled());
    assert!(h.get_by_role_and_label(Role::Slider, "Transparency").accesskit_node().is_disabled());
    assert!(h.query_by_label("Properties").is_some(), "the panel's dialog launcher");
}

#[test]
fn group_selection_shows_and_flips_pickstyle() {
    let mut h = harness("Groups");
    let toggled = |h: &Harness<'_, CadApp>| h.get_by_label("Group Selection On/Off").accesskit_node().toggled();
    assert_eq!(h.state().session.settings.pickstyle & 1, 1, "group selection starts on");
    assert_eq!(toggled(&h), Some(Toggled::True));
    h.get_by_label("Group Selection On/Off").click();
    h.run_steps(3);
    assert_eq!(h.state().session.settings.pickstyle & 1, 0);
    assert_eq!(toggled(&h), Some(Toggled::False));
}

#[test]
fn quick_calculator_shows_pressed_once_its_palette_is_open() {
    let mut h = harness("Utilities");
    let toggled = |h: &Harness<'_, CadApp>| h.get_by_label("Quick Calculator").accesskit_node().toggled();
    assert_eq!(toggled(&h), Some(Toggled::False));
    h.get_by_label("Quick Calculator").click();
    h.run_steps(3);
    assert_eq!(toggled(&h), Some(Toggled::True));
}
