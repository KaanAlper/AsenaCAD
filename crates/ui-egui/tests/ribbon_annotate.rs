//! The Annotate ribbon's controls: buttons start their commands, the Linear split lists
//! AutoCAD's variants and remembers the last one, unbuilt items are disabled, the style lists
//! show the current styles, Find Text finds text and Annotation Text Height sets TEXTSIZE.

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::annotate::ANNOTATE;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::{render_panel_for_test, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn panel(label: &str) -> &'static Panel {
    ANNOTATE.panels.iter().find(|p| p.label == label).expect("an Annotate panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &ANNOTATE, panel), app);
    harness.run_steps(3);
    harness
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn clicking_center_mark_starts_centermark() {
    let mut h = harness("Centerlines");
    h.get_by_label("Center Mark").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("centermark"));
}

#[test]
fn the_linear_menu_ends_jogged_ordinate_and_remembers_the_choice() {
    let mut h = harness("Dimensions");
    h.get_by_label("Linear options").click();
    h.run_steps(3);
    let entries = ["Aligned", "Angular", "Arc Length", "Radius", "Diameter", "Jogged", "Ordinate"];
    let tops: Vec<f32> = entries.iter().map(|l| h.get_by_label(l).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "entries in AutoCAD's order: {tops:?}");
    h.get_by_label("Ordinate").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("dimordinate"));
    assert_eq!(RibbonState::split(&h.ctx, "Annotate/Dimensions/9", 0), 9);
    assert!(h.query_by_role_and_label(Role::Button, "Ordinate").is_some(), "the face reads Ordinate");
}

#[test]
fn items_without_a_command_are_disabled() {
    let h = harness("Text");
    for label in ["Check Spelling", "Text Align", "Justify", "Scale"] {
        assert!(h.get_by_label(label).accesskit_node().is_disabled(), "{label} has no command yet");
    }
    let h = harness("Dimensions");
    for label in ["Break", "Adjust Space", "Jog Line", "Inspect", "Update", "Reassociate", "Quick", "Override"] {
        assert!(!h.get_by_label(label).accesskit_node().is_disabled(), "{label} runs its command");
    }
}

#[test]
fn the_style_lists_show_the_current_styles_and_layer_lists_are_disabled() {
    let h = harness("Text");
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Text Style").value().as_deref(), Some("Standard"));
    let layer = h.get_by_role_and_label(Role::ComboBox, "Text Layer Gallery");
    assert_eq!(layer.value().as_deref(), Some("Use Current"));
    assert!(layer.accesskit_node().is_disabled());
    let h = harness("Dimensions");
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Dimension Style").value().as_deref(), Some("Standard"));
    assert!(h.get_by_role_and_label(Role::ComboBox, "Dim Layer Gallery").accesskit_node().is_disabled());
    let h = harness("Leaders");
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Multileader Style").value().as_deref(), Some("Standard"));
}

fn text_size(h: &Harness<'_, CadApp>) -> f64 {
    h.state().session.doc().map(|d| d.header.f64("TEXTSIZE", 0.0)).unwrap_or_default()
}

#[test]
fn annotation_text_height_shows_and_sets_textsize() {
    let mut h = harness("Text");
    let field = || "Annotation Text Height";
    assert_eq!(h.get_by_label(field()).value().as_deref(), Some("0.2"));
    h.get_by_label(field()).click();
    h.run_steps(2);
    h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
    h.get_by_label(field()).type_text("3.5");
    h.run_steps(2);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert_eq!(text_size(&h), 3.5, "Enter sets TEXTSIZE");
    assert_eq!(h.get_by_label(field()).value().as_deref(), Some("3.5"));

    // Not a positive number: nothing changes and the box shows the drawing's value again.
    h.get_by_label(field()).click();
    h.run_steps(2);
    h.key_press_modifiers(egui::Modifiers::COMMAND, egui::Key::A);
    h.get_by_label(field()).type_text("-1");
    h.run_steps(2);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert_eq!(text_size(&h), 3.5);
    assert_eq!(h.get_by_label(field()).value().as_deref(), Some("3.5"));
}

#[test]
fn find_text_selects_the_objects_with_the_typed_text() {
    let mut h = harness("Text");
    h.state_mut().run("text", json!({ "at": [0, 0], "text": "Valve A" })).unwrap();
    h.state_mut().run("text", json!({ "at": [0, 5], "text": "Pump" })).unwrap();
    h.run_steps(2);
    h.get_by_label("Find Text").click();
    h.run_steps(2);
    h.get_by_label("Find Text").type_text("Valve");
    h.run_steps(2);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert_eq!(h.state().session.selection().len(), 1, "FIND selects the text it finds");
}
