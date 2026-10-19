//! The Express Tools ribbon's controls: the Express Tools commands AsenaCAD has (NCOPY, FLATTEN,
//! ALIASEDIT) are enabled and start, the others are disabled, and the Modify Text, Annotation
//! Attachment splits keep their face's label and are disabled, URL Options is a drop-down button
//! disabled until one of its entries has a command.

use asenacad_engine::Session;
use asenacad_ui_egui::ribbon::express::EXPRESS;
use asenacad_ui_egui::ribbon::model::Panel;
use asenacad_ui_egui::ribbon::render_panel_for_test;
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel: &'static Panel = EXPRESS.panels.iter().find(|p| p.label == label).expect("an Express Tools panel");
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &EXPRESS, panel), app);
    harness.run_steps(3);
    harness
}

fn disabled(h: &Harness<'_, CadApp>, label: &str) -> bool {
    h.get_by_label(label).accesskit_node().is_disabled()
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn copy_nested_objects_and_flatten_start_their_commands() {
    for (panel, label, cmd) in [("Blocks", "Copy Nested Objects", "ncopy"), ("Modify", "Flatten objects", "flatten")] {
        let mut h = harness(panel);
        h.get_by_label(label).click();
        h.run_steps(3);
        assert_eq!(running(&h).as_deref(), Some(cmd), "{label}");
    }
    assert!(!disabled(&harness("Tools"), "Command Aliases"), "ALIASEDIT exists");
}

#[test]
fn express_tools_without_a_command_are_disabled() {
    for (panel, labels) in [
        (
            "Blocks",
            &["Explode Attributes", "Replace Block", "List Properties", "Import Attributes", "Export Attributes", "Convert block to xref"][..],
        ),
        ("Text", &["Arc Aligned", "Convert to Mtext", "Auto Number", "Enclose in Object", "Remote Text", "Text Mask", "Unmask Text"][..]),
        ("Modify", &["Move/Copy/Rotate", "Stretch Multiple", "Extended Clip", "Extended Offset", "Convert Shape to Block"][..]),
        ("Layout", &["Align Space", "Synchronize Viewports", "Merge Layout", "List Viewport Scale"][..]),
        ("Draw", &["Break-line Symbol", "Super Hatch"][..]),
        ("Dimension", &["Reset Text", "Import Style", "Export Style"][..]),
        ("Tools", &["Attach Xdata", "List Xdata", "System Variables", "Extended Plan", "Make Linetype", "Make Shape"][..]),
    ] {
        let h = harness(panel);
        for label in labels {
            assert!(disabled(&h, label), "{panel} ▸ {label} has no command yet");
        }
    }
}

#[test]
fn the_splits_keep_a_fixed_face_and_are_disabled_while_no_entry_has_a_command() {
    // Their entries (in AutoCAD's order) are checked by the ribbon_tabs spec test.
    for (panel, face) in [("Text", "Modify Text"), ("Dimension", "Annotation Attachment")] {
        let h = harness(panel);
        assert!(h.get_by_role_and_label(Role::Button, face).accesskit_node().is_disabled(), "{face}: its first entry has no command");
        assert!(disabled(&h, &format!("{face} options")), "{face}: no entry of its list has a command yet");
    }
}

#[test]
fn url_options_is_one_drop_down_button_disabled_until_an_entry_has_a_command() {
    let mut h = harness("Web");
    assert!(h.query_by_label("URL Options options").is_none(), "no separate ▾ part");
    let button = h.get_by_role_and_label(Role::Button, "URL Options");
    assert!(button.accesskit_node().is_disabled(), "none of Show URLs, Change URLs, Find and Replace URLs exists yet");
    button.click();
    h.run_steps(3);
    assert!(h.query_by_label("Show URLs").is_none(), "a disabled drop-down doesn't open");
}

#[test]
fn the_modify_panels_two_rows_are_centred_in_its_height() {
    let h = harness("Modify");
    let (a, b) = (h.get_by_label("Move/Copy/Rotate").rect(), h.get_by_label("Stretch Multiple").rect());
    let title = h.get_by_label("Modify").rect();
    // AutoCAD's large items stand 4 px below the panel's top, 79 px above its title bar, and
    // fill 72 px; the two rows sit in the middle of that, one row step apart (observed: the
    // panel at y 57, the rows at 74 and 98, the title bar at 136).
    let top = title.top() - 79.0 + 4.0;
    assert!((b.top() - a.top() - 24.0).abs() < 0.5, "one row step apart: {a:?} {b:?}");
    let middle = (a.top() + b.bottom()) / 2.0;
    assert!((middle - (top + 36.0)).abs() < 1.0, "centred: rows {a:?} {b:?}, content top {top}");
    assert!((a.top() - (title.top() - 62.0)).abs() < 0.5, "AutoCAD's row 1 is 62 px above the title bar: {a:?}");
}
