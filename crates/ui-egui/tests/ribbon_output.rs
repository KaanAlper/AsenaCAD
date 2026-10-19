//! The Output and Collaborate ribbons' controls: Plot and its launcher open the Plot dialog, the
//! plot tools that exist are enabled, the Export split lists AutoCAD's formats and remembers the
//! choice, the export lists show AutoCAD's defaults disabled, and Collaborate's online tools are
//! disabled while DWG Compare runs COMPARE.

use asenacad_engine::Session;
use asenacad_ui_egui::ribbon::model::{Panel, Tab};
use asenacad_ui_egui::ribbon::output::{COLLABORATE, OUTPUT};
use asenacad_ui_egui::ribbon::{render_panel_for_test, widgets::LAUNCHER, widgets::RibbonState};
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

fn harness(tab: &'static Tab, label: &str) -> Harness<'static, CadApp> {
    let panel: &'static Panel = tab.panels.iter().find(|p| p.label == label).expect("a panel of the tab");
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness =
        Harness::builder().with_size(vec2(900.0, 420.0)).build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, tab, panel), app);
    harness.run_steps(3);
    harness
}

fn disabled(h: &Harness<'_, CadApp>, label: &str) -> bool {
    h.get_by_label(label).accesskit_node().is_disabled()
}

#[test]
fn plot_and_its_launcher_open_the_plot_dialog() {
    let mut h = harness(&OUTPUT, "Plot");
    h.get_by_role_and_label(Role::Button, "Plot").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("plot"));
    for label in ["Batch Plot", "Preview", "Page Setup Manager", "Plotter Manager"] {
        assert!(!disabled(&h, label), "{label} runs its command");
    }
    assert!(disabled(&h, "View Details"), "VIEWPLOTDETAILS has no command yet");

    let mut h = harness(&OUTPUT, "Plot");
    let launcher = h.query_all_by_role(Role::Button).find(|n| n.rect().size() == LAUNCHER).expect("the panel's dialog launcher");
    assert_eq!(launcher.accesskit_node().label().as_deref(), Some("Plot, Plot Options Dialog"));
    launcher.click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("plot"), "the launcher runs PLOT");
}

#[test]
fn export_lists_dwfx_dwf_and_pdf_and_remembers_the_choice() {
    let mut h = harness(&OUTPUT, "Export to DWF/PDF");
    assert!(h.get_by_role_and_label(Role::Button, "Export").accesskit_node().is_disabled(), "the face starts on DWFx, which isn't built");
    h.get_by_label("Export options").click();
    h.run_steps(3);
    let tops: Vec<f32> = ["DWFx", "DWF", "PDF"].iter().map(|l| h.get_by_label(l).rect().top()).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "entries in AutoCAD's order: {tops:?}");
    assert!(disabled(&h, "DWFx") && disabled(&h, "DWF"), "DWF export isn't built");
    assert!(!disabled(&h, "PDF"), "EXPORTPDF exists");
    h.get_by_label("PDF").click();
    h.run_steps(3);
    assert_eq!(RibbonState::split(&h.ctx, "Output/Export to DWF/PDF/0", 0), 2);
    assert!(!h.get_by_role_and_label(Role::Button, "Export").accesskit_node().is_disabled(), "the face runs EXPORTPDF now");
}

#[test]
fn the_export_lists_show_autocads_defaults_disabled() {
    let h = harness(&OUTPUT, "Export to DWF/PDF");
    let values: Vec<_> = h
        .query_all_by_role(Role::ComboBox)
        .filter(|n| n.accesskit_node().label().as_deref() == Some("EplotExport"))
        .map(|n| (n.value(), n.accesskit_node().is_disabled()))
        .collect();
    assert_eq!(values, [(Some("Export: Display".to_string()), true), (Some("Page Setup: Current".to_string()), true)]);
    for label in ["Export Window", "Page Setup", "Preview", "Export to DWF Options", "Export to PDF Options"] {
        assert!(disabled(&h, label), "{label} has no command yet");
    }
}

#[test]
fn collaborates_online_tools_are_disabled_and_dwg_compare_runs_compare() {
    for (panel, labels) in [
        ("Share", &["Share Drawing", "Shared Views"][..]),
        ("Autodesk Docs", &["Push to Autodesk Docs"][..]),
        ("Traces", &["Traces Palette", "Markup Import"][..]),
    ] {
        let h = harness(&COLLABORATE, panel);
        for label in labels {
            assert!(disabled(&h, label), "{label} needs Autodesk's online services");
        }
    }
    let mut h = harness(&COLLABORATE, "Compare");
    h.get_by_label("DWG Compare").click();
    h.run_steps(3);
    assert_eq!(h.state().session.running.as_ref().map(|r| r.id.as_str()), Some("compare"));
}
