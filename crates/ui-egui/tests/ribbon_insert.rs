//! The Insert ribbon's controls: buttons start their commands, split buttons remember the last
//! entry, unbuilt items are disabled, lists show the drawing's values and the Reference
//! launcher opens the External References palette.

use cadcraft_engine::Session;
use cadcraft_ui_egui::docking::Panel as Palette;
use cadcraft_ui_egui::ribbon::insert::INSERT;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::{render_panel_for_test, widgets::LAUNCHER, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn panel(label: &str) -> &'static Panel {
    INSERT.panels.iter().find(|p| p.label == label).expect("an Insert panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &INSERT, panel), app);
    harness.run_steps(3);
    harness
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn clicking_define_attributes_starts_attdef() {
    let mut h = harness("Block Definition");
    h.get_by_label("Define Attributes").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("attdef"));
}

#[test]
fn choosing_write_block_puts_it_on_the_create_block_face() {
    let mut h = harness("Block Definition");
    h.get_by_label("Create Block options").click();
    h.run_steps(3);
    let tops: Vec<f32> =
        ["Create Block", "Write Block"].iter().map(|l| h.query_all_by_label(l).map(|n| n.rect().top()).fold(f32::MIN, f32::max)).collect();
    assert!(tops[0] < tops[1], "entries in AutoCAD's order: {tops:?}");
    h.get_by_label("Write Block").click();
    h.run_steps(3);
    // WBLOCK has no interactive form yet: started without a file it only reports what it needs.
    assert_eq!(RibbonState::split(&h.ctx, "Insert/Block Definition/0", 0), 1);
    assert!(h.query_by_role_and_label(Role::Button, "Write Block").is_some(), "the face reads Write Block");
    assert!(h.query_by_label("Create Block").is_none());
}

#[test]
fn items_without_a_command_are_disabled() {
    let h = harness("Data");
    for label in ["Field", "Update Fields", "OLE Object", "Hyperlink"] {
        assert!(h.get_by_label(label).accesskit_node().is_disabled(), "{label} has no command yet");
    }
    let h = harness("Reference");
    // ATTACH is a placeholder command: shown, but disabled.
    assert!(h.get_by_label("Attach").accesskit_node().is_disabled());
    assert!(h.get_by_label("Clip").accesskit_node().is_disabled());
    assert!(!h.get_by_label("Edit Reference").accesskit_node().is_disabled(), "slide-out items with a command are enabled");
    let h = harness("Linking & Extraction");
    assert!(!h.get_by_label("Data Link").accesskit_node().is_disabled());
    assert!(h.get_by_label("Upload to Source").accesskit_node().is_disabled());
}

#[test]
fn the_lists_show_the_drawings_values_and_unmodelled_ones_are_disabled() {
    let h = harness("Block");
    let display = h.get_by_role_and_label(Role::ComboBox, "Retain Attribute Display");
    assert_eq!(display.value().as_deref(), Some("Retain Attribute Display"));
    assert!(!display.accesskit_node().is_disabled());
    let h = harness("Reference");
    let frames = h.get_by_role_and_label(Role::ComboBox, "*Frames vary*");
    assert_eq!(frames.value().as_deref(), Some("*Frames vary*"));
    assert!(frames.accesskit_node().is_disabled());
    assert!(h.get_by_role_and_label(Role::ComboBox, "Snap to Underlays ON").accesskit_node().is_disabled());
    let fading = h.get_by_role_and_label(Role::Slider, "Xref fading");
    assert!(fading.accesskit_node().is_disabled());
}

#[test]
fn the_reference_launcher_opens_the_external_references_palette() {
    let mut h = harness("Reference");
    assert!(!h.state().ui.docking.layout.contains(&Palette::ExternalRefs));
    let launcher = h.query_all_by_role(Role::Button).find(|n| n.rect().size() == LAUNCHER).expect("the panel's dialog launcher");
    launcher.click();
    h.run_steps(3);
    assert!(h.state().ui.docking.layout.contains(&Palette::ExternalRefs), "the launcher runs XREF");
}

#[test]
fn set_location_is_one_drop_down_button_disabled_until_an_entry_has_a_command() {
    let mut h = harness("Location");
    // One button, no separate face and ▾ part.
    assert!(h.query_by_label("Set Location options").is_none());
    let button = h.get_by_role_and_label(Role::Button, "Set Location");
    assert!(button.accesskit_node().is_disabled(), "none of From Esri Maps, From File, From Bing Maps exists yet");
    button.click();
    h.run_steps(3);
    assert!(h.query_by_label("From File").is_none(), "a disabled drop-down doesn't open");
}

#[test]
fn the_insert_gallery_opens_a_list_leading_to_the_insert_command() {
    let mut h = harness("Block");
    assert!(!h.get_by_role_and_label(Role::Button, "Insert").accesskit_node().is_disabled());
    h.get_by_role_and_label(Role::Button, "Insert").click();
    h.run_steps(3);
    h.get_by_label("More Options...").click();
    h.run_steps(3);
    assert!(h.query_by_label("More Options...").is_none(), "the list closes");
    let started = running(&h).as_deref() == Some("insert") || h.state().ui.dialog.is_some();
    assert!(started, "More Options... starts INSERT");
}
