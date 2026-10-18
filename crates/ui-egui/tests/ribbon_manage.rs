//! The Manage ribbon's controls: the cleanup tools start their commands, the applications that
//! exist are enabled, and the unbuilt Action Recorder, customization and standards tools are
//! disabled.

use cadcraft_engine::Session;
use cadcraft_ui_egui::ribbon::manage::MANAGE;
use cadcraft_ui_egui::ribbon::model::Panel;
use cadcraft_ui_egui::ribbon::render_panel_for_test;
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn panel(label: &str) -> &'static Panel {
    MANAGE.panels.iter().find(|p| p.label == label).expect("a Manage panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &MANAGE, panel), app);
    harness.run_steps(3);
    harness
}

fn disabled(h: &Harness<'_, CadApp>, label: &str) -> bool {
    h.get_by_label(label).accesskit_node().is_disabled()
}

#[test]
fn cleanup_tools_start_their_commands() {
    for (label, cmd) in [("Purge", "purge"), ("Overkill", "overkill"), ("Audit", "audit")] {
        let mut h = harness("Cleanup");
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(h.state().session.running.as_ref().map(|r| r.id.as_str()), Some(cmd), "{label}");
    }
    assert!(disabled(&harness("Cleanup"), "Find Non-Purgeable Items"));
}

#[test]
fn the_action_recorder_is_shown_disabled() {
    let h = harness("Action Recorder");
    for label in ["Record", "Insert Message", "Insert Base Point", "Pause for User Input", "Play", "Preference", "Manage Action Macros"] {
        assert!(disabled(&h, label), "{label}: there is no Action Recorder yet");
    }
    assert!(h.get_by_role_and_label(Role::ComboBox, "Available Action Macro").accesskit_node().is_disabled());
}

#[test]
fn applications_that_exist_are_enabled() {
    let h = harness("Applications");
    assert!(!disabled(&h, "Load Application"));
    assert!(!disabled(&h, "Run Script"));
    for label in ["Visual Basic Editor", "Visual LISP Editor", "Run VBA Macro", "Load Project", "VBA Manager"] {
        assert!(disabled(&h, label), "{label} has no command yet");
    }
}

#[test]
fn edit_aliases_is_a_split_and_the_standards_tools_are_disabled() {
    let h = harness("Customization");
    assert!(h.query_by_label("Edit Aliases options").is_some(), "Edit Aliases has a ▾ part");
    for label in ["User Interface", "Tool Palettes", "Import", "Export"] {
        assert!(disabled(&h, label), "{label} has no command yet");
    }
    let h = harness("CAD Standards");
    for label in ["Layer Translator", "Check", "Configure"] {
        assert!(disabled(&h, label), "{label} has no command yet");
    }
}
