//! The View ribbon's controls: the viewport tool and interface toggles follow and flip the
//! drawing area's aids, the palette toggles follow the palettes, the Viewport Configuration
//! split lists AutoCAD's arrangements and the Named Views list shows the drawing's views.

use cadcraft_engine::Session;
use cadcraft_ui_egui::docking::Panel as Palette;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::view::VIEW;
use cadcraft_ui_egui::ribbon::{render_panel_for_test, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::{Role, Toggled};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn panel(label: &str) -> &'static Panel {
    VIEW.panels.iter().find(|p| p.label == label).expect("a View panel")
}

fn harness_with(label: &str, app: CadApp) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &VIEW, panel), app);
    harness.run_steps(3);
    harness
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    harness_with(label, CadApp::new(Session::default(), Services::default()))
}

fn toggled(h: &Harness<'_, CadApp>, label: &str) -> Option<Toggled> {
    h.get_by_label(label).accesskit_node().toggled()
}

#[test]
fn viewport_tools_follow_and_flip_the_ucs_icon_view_cube_and_navigation_bar() {
    let mut h = harness("Viewport Tools");
    for label in ["UCS Icon", "View Cube", "Navigation Bar"] {
        assert_eq!(toggled(&h, label), Some(Toggled::True), "{label} is on, as in AutoCAD's default drawing");
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(toggled(&h, label), Some(Toggled::False), "{label} turned off");
    }
    let ui = &h.state().ui;
    assert!(!ui.show_ucs_icon && !ui.show_viewcube && !ui.show_navbar);
    assert!(h.get_by_label("Show Motion").accesskit_node().is_disabled(), "ShowMotion isn't built");
}

#[test]
fn interface_toggles_hide_the_file_and_layout_tabs() {
    let mut h = harness("Interface");
    assert_eq!(toggled(&h, "Layout Tabs"), Some(Toggled::True));
    h.get_by_label("Layout Tabs").click();
    h.run_steps(3);
    assert!(!h.state().ui.show_layout_tabs);
    assert_eq!(toggled(&h, "Layout Tabs"), Some(Toggled::False));
    h.get_by_label("File Tabs").click();
    h.run_steps(3);
    assert!(!h.state().ui.show_file_tabs);
    for label in ["Tile Horizontally", "Tile Vertically", "Cascade"] {
        assert!(h.get_by_label(label).accesskit_node().is_disabled(), "{label} has no command yet");
    }
    assert!(h.get_by_role_and_label(Role::Button, "Switch Windows").accesskit_node().is_disabled());
}

#[test]
fn palette_toggles_follow_the_palettes() {
    let mut h = harness("Palettes");
    assert_eq!(toggled(&h, "Command Line"), Some(Toggled::True));
    assert_eq!(toggled(&h, "Properties"), Some(Toggled::False), "AutoCAD's default workspace has the palettes closed");
    h.get_by_label("Properties").click();
    h.run_steps(3);
    assert!(h.state().ui.docking.layout.contains(&Palette::Properties));
    assert_eq!(toggled(&h, "Properties"), Some(Toggled::True));
    h.get_by_label("External References").click();
    h.run_steps(3);
    assert_eq!(toggled(&h, "External References"), Some(Toggled::True));
    for label in ["Tool Palettes", "Sheet Set Manager", "Design Center", "Materials Browser"] {
        assert!(h.get_by_label(label).accesskit_node().is_disabled(), "{label} isn't built");
    }
    assert!(!h.get_by_label("Text Window").accesskit_node().is_disabled(), "TEXTSCR opens the command history");
}

#[test]
fn viewport_configuration_lists_autocads_arrangements_and_remembers_the_choice() {
    let mut h = harness("Model Viewports");
    h.get_by_label("Viewport Configuration options").click();
    h.run_steps(3);
    let names = ["Single", "Two: Vertical", "Three: Right", "Four: Equal", "Four: Left"];
    let tops: Vec<f32> = names.iter().map(|l| h.get_by_label(l).rect().top()).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "entries in AutoCAD's order: {tops:?}");
    assert!(!h.get_by_label("Single").accesskit_node().is_disabled());
    assert!(!h.get_by_label("Two: Vertical").accesskit_node().is_disabled());
    assert!(h.get_by_label("Four: Right").accesskit_node().is_disabled(), "-VPORTS has no Four: Right arrangement yet");
    h.get_by_label("Four: Equal").click();
    h.run_steps(3);
    assert_eq!(RibbonState::split(&h.ctx, "View/Model Viewports/0", 0), 9);
    let tiles = h.state().session.model_tiles().len();
    assert_eq!(tiles, 4, "Four: Equal tiles the model space into four viewports");
    assert!(h.query_by_role_and_label(Role::Button, "Viewport Configuration").is_some(), "the face keeps its label");
}

#[test]
fn the_named_views_list_shows_unsaved_view_and_lists_the_drawings_views() {
    let h = harness("Named Views");
    let list = h.get_by_role_and_label(Role::ComboBox, "Named Views");
    assert_eq!(list.value().as_deref(), Some("Unsaved View"));
    assert!(list.accesskit_node().is_disabled(), "nothing to restore in a drawing without views");
    let mut app = CadApp::new(Session::default(), Services::default());
    app.session.execute("view", &json!({ "option": "save", "name": "Plan" })).expect("VIEW Save");
    let h = harness_with("Named Views", app);
    assert!(!h.get_by_role_and_label(Role::ComboBox, "Named Views").accesskit_node().is_disabled());
    assert!(!h.get_by_label("View Manager").accesskit_node().is_disabled());
    assert!(h.get_by_label("New View").accesskit_node().is_disabled(), "NEWVIEW has no command yet");
}
