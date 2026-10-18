//! AutoCAD's title row: the Application button, the Quick Access Toolbar in AutoCAD's order with
//! its Customize ▾ menu and Undo/Redo history splits, the centred title and the InfoCenter
//! search and Help.

use cadcraft_engine::Session;
use cadcraft_ui_egui::chrome::titlerow;
use cadcraft_ui_egui::{CadApp, Services, ribbon};
use egui::accesskit::{Role, Toggled};
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn harness_with(app: CadApp) -> Harness<'static, CadApp> {
    let mut h = Harness::builder().with_size(vec2(1906.0, 600.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            titlerow::show(app, ui);
            titlerow::qat_strip(app, ui);
        },
        app,
    );
    h.run_steps(3);
    h
}

fn harness() -> Harness<'static, CadApp> {
    harness_with(CadApp::new(Session::default(), Services::default()))
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

fn left(h: &Harness<'_, CadApp>, label: &str) -> f32 {
    h.get_by_role_and_label(Role::Button, label).rect().min.x
}

/// The bottom of the title row (the harness puts its panels inside a margin).
fn row_bottom(h: &Harness<'_, CadApp>) -> f32 {
    h.get_by_role_and_label(Role::Button, "Application Button").rect().center().y + titlerow::HEIGHT / 2.0
}

/// The menu row labelled `label`: the one below the title row (QAT buttons share some labels).
fn menu_row<'a>(h: &'a Harness<'_, CadApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let bottom = row_bottom(h);
    let mut nodes: Vec<_> = h.query_all_by_label(label).filter(|n| n.rect().min.y > bottom).collect();
    assert_eq!(nodes.len(), 1, "one menu row for {label}");
    nodes.remove(0)
}

fn open_customize(h: &mut Harness<'_, CadApp>) {
    h.get_by_label("Customize Quick Access Toolbar").click();
    h.run_steps(3);
}

#[test]
fn the_quick_access_toolbar_has_autocads_items_in_order() {
    let h = harness();
    let order = ["Application Button", "New", "Open", "Save", "Save As...", "Plot", "Undo", "Undo options", "Redo", "Redo options"];
    let xs: Vec<f32> = order.iter().map(|l| left(&h, l)).collect();
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "AutoCAD's order: {xs:?}");
    assert!(left(&h, "Customize Quick Access Toolbar") > left(&h, "Redo options"));
    // AutoCAD's buttons touch: 24 px apart, the splits' ▾ parts 13 px wide.
    assert!((xs[2] - xs[1] - 24.0).abs() < 0.5 && (xs[8] - xs[7] - 13.0).abs() < 0.5, "{xs:?}");
    for absent in ["Plot Preview", "Batch Plot", "Share", "Open from Web & Mobile", "Save to Web & Mobile", "Sign In"] {
        assert!(h.query_by_label(absent).is_none(), "{absent} is not shown");
    }
}

#[test]
fn customize_lists_autocads_items_with_check_marks_and_disables_missing_commands() {
    let mut h = harness();
    open_customize(&mut h);
    let items = [
        "New",
        "Open",
        "Save",
        "Save As...",
        "Plot",
        "Undo",
        "Redo",
        "Batch Plot",
        "Layer",
        "Match Properties",
        "Plot Preview",
        "Properties",
        "Render",
        "Sheet Set Manager",
        "Workspace",
        "More Commands...",
        "Show Menu Bar",
        "Show Below the Ribbon",
    ];
    let tops: Vec<f32> = items.iter().map(|l| menu_row(&h, l).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "AutoCAD's order: {tops:?}");
    assert_eq!(menu_row(&h, "Plot").accesskit_node().toggled(), Some(Toggled::True));
    assert_eq!(menu_row(&h, "Plot Preview").accesskit_node().toggled(), Some(Toggled::False));
    for missing in ["Render", "Sheet Set Manager", "Workspace", "More Commands..."] {
        assert!(menu_row(&h, missing).accesskit_node().is_disabled(), "{missing} has no command yet");
    }
    assert!(!menu_row(&h, "Batch Plot").accesskit_node().is_disabled());
}

#[test]
fn checking_plot_preview_adds_it_after_redo_and_unchecking_removes_it() {
    let mut h = harness();
    open_customize(&mut h);
    h.get_by_label("Plot Preview").click();
    h.run_steps(3);
    assert!(h.state().ui.qat_shown.iter().any(|c| c == "preview"));
    assert!(left(&h, "Plot Preview") > left(&h, "Redo options"), "after Redo, in AutoCAD's order");
    assert!(left(&h, "Customize Quick Access Toolbar") > left(&h, "Plot Preview"));
    assert!(!h.get_by_role_and_label(Role::Button, "Plot Preview").accesskit_node().is_disabled());
    open_customize(&mut h);
    menu_row(&h, "Plot Preview").click();
    h.run_steps(3);
    assert!(h.query_by_label("Plot Preview").is_none());
}

#[test]
fn show_menu_bar_flips_the_in_window_menu_bar() {
    // AutoCAD hides its menu bar by default, on every platform.
    let app = CadApp::new(Session::default(), Services::default());
    assert!(!app.ui.in_window_menu);
    let mut h = harness_with(app);
    open_customize(&mut h);
    h.get_by_label("Show Menu Bar").click();
    h.run_steps(3);
    assert!(h.state().ui.in_window_menu);
    open_customize(&mut h);
    h.get_by_label("Hide Menu Bar").click();
    h.run_steps(3);
    assert!(!h.state().ui.in_window_menu);
}

#[test]
fn show_below_the_ribbon_moves_the_toolbar_into_its_strip() {
    let mut h = harness();
    open_customize(&mut h);
    h.get_by_label("Show Below the Ribbon").click();
    h.run_steps(3);
    assert!(h.state().ui.qat_below_ribbon);
    let new = h.get_by_role_and_label(Role::Button, "New").rect();
    assert!(new.min.y >= row_bottom(&h), "the QAT is under the title row: {new:?}");
    open_customize(&mut h);
    h.get_by_label("Show Above the Ribbon").click();
    h.run_steps(3);
    assert!(!h.state().ui.qat_below_ribbon);
    assert!(h.get_by_role_and_label(Role::Button, "New").rect().max.y <= row_bottom(&h));
}

#[test]
fn redo_is_disabled_until_something_is_undone_and_the_lists_name_the_steps() {
    let mut h = harness();
    assert!(h.get_by_label("Redo").accesskit_node().is_disabled(), "nothing to redo in a fresh drawing");
    assert!(h.get_by_label("Redo options").accesskit_node().is_disabled());
    assert!(!h.get_by_label("Undo").accesskit_node().is_disabled(), "AutoCAD keeps Undo enabled");
    for x in [0.0, 20.0] {
        h.state_mut().run("line", json!({"points": [[x, 0.0], [x + 10.0, 0.0]]})).unwrap();
        h.state_mut().run("circle", json!({"center": [x, 5.0], "radius": 2.0})).unwrap();
    }
    h.run_steps(3);
    h.get_by_label("Undo options").click();
    h.run_steps(3);
    let tops: Vec<f32> = ["Circle", "Line"].iter().map(|l| h.query_all_by_label(l).next().expect(l).rect().min.y).collect();
    assert!(tops[0] < tops[1], "the last step first");
    assert_eq!(h.query_all_by_label("Line").count(), 2);
    // Choosing the second step undoes it and the one listed before it.
    h.query_all_by_label("Line").next().unwrap().click();
    h.run_steps(3);
    let st = h.state().session.state().unwrap();
    assert_eq!((st.undo.len(), st.redo.len()), (2, 2));
    assert!(!h.get_by_label("Redo").accesskit_node().is_disabled());
    h.get_by_label("Redo").click();
    h.run_steps(3);
    assert_eq!(h.state().session.state().unwrap().redo.len(), 1);
}

#[test]
fn the_title_names_asenacad_and_the_drawing() {
    let h = harness();
    let title = h.get_by_role(Role::Label);
    let text = title.accesskit_node().value().unwrap_or_default();
    assert!(text.starts_with("AsenaCAD  ") && text.ends_with(".dwg"), "{text}");
    let name = h.state().session.state().unwrap().title.clone();
    assert!(text.contains(&name), "{text} names {name}");
    // Centred in the window, between the toolbar and the InfoCenter.
    let r = title.rect();
    assert!((r.center().x - 953.0).abs() < 2.0, "{r:?}");
    assert!(r.left() > left(&h, "Customize Quick Access Toolbar") && r.right() < left(&h, "Hide search"));
}

#[test]
fn the_title_is_cut_rather_than_overlapping_in_a_narrow_window() {
    let mut h = harness();
    h.set_size(vec2(760.0, 300.0));
    h.run_steps(3);
    let title = h.get_by_role(Role::Label).rect();
    assert!(title.left() > left(&h, "Customize Quick Access Toolbar") + 16.0);
    assert!(title.right() < left(&h, "Hide search"), "{title:?}");
}

#[test]
fn searching_a_command_name_starts_it() {
    let mut h = harness();
    let search = h.get_by_label("Type a keyword or phrase");
    search.focus();
    h.run_steps(2);
    h.get_by_label("Type a keyword or phrase").type_text("line");
    h.run_steps(2);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("line"));
}

#[test]
fn searching_other_text_opens_the_command_reference_filtered_by_it() {
    let mut app = CadApp::new(Session::default(), Services::default());
    titlerow::search_for(&mut app, &egui::Context::default(), "c");
    assert_eq!(app.session.running.as_ref().map(|r| r.id.as_str()), Some("circle"), "an alias starts its command");
    app.session.cancel();
    let ctx = egui::Context::default();
    titlerow::search_for(&mut app, &ctx, "  polar array ");
    assert_eq!(app.ui.dialog.as_deref(), Some("commands"));
    assert!(app.session.running.is_none());
    let filter = ctx.data(|d| d.get_temp::<String>(egui::Id::new("asenacad-command-reference-filter")));
    assert_eq!(filter.as_deref(), Some("polar array"));
}

#[test]
fn the_search_box_hides_and_shows() {
    let mut h = harness();
    h.get_by_label("Hide search").click();
    h.run_steps(3);
    assert!(h.query_by_label("Type a keyword or phrase").is_none());
    h.get_by_label("Show search").click();
    h.run_steps(3);
    assert!(h.query_by_label("Type a keyword or phrase").is_some());
}

#[test]
fn help_opens_the_command_reference_and_its_menu_lists_the_help_menu() {
    let mut h = harness();
    h.get_by_role_and_label(Role::Button, "Help").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("commands"));
    h.state_mut().ui.dialog = None;
    h.get_by_label("Help options").click();
    h.run_steps(3);
    h.get_by_label("About AsenaCAD").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("about"));
}

#[test]
fn the_application_button_lists_the_file_menu() {
    let mut h = harness();
    h.get_by_label("Application Button").click();
    h.run_steps(3);
    // Menu buttons carry their shortcut in their name ("Save As... Ctrl+Shift+S").
    let menu_item = |h: &Harness<'_, CadApp>, label: &str| {
        let bottom = row_bottom(h);
        h.query_all_by_label_contains(label).filter(|n| n.rect().min.y > bottom).map(|n| n.rect()).next()
    };
    for label in ["New Drawing...", "Open...", "Close All", "Save As...", "Batch Publish..."] {
        assert!(menu_item(&h, label).is_some(), "{label} in the menu");
    }
    let bottom = row_bottom(&h);
    h.query_all_by_label_contains("Save As...").find(|n| n.rect().min.y > bottom).unwrap().click();
    h.run_steps(3);
    assert!(menu_item(&h, "New Drawing...").is_none(), "the menu closes");
}

/// `CRAFT_UI_VISUAL_DIR=/dir cargo test -p cadcraft-ui-egui --test titlerow title_row_visuals`
/// renders the title row over the ribbon at AutoCAD's window width for a visual check.
#[test]
fn title_row_visuals() {
    let Some(directory) = std::env::var_os("CRAFT_UI_VISUAL_DIR").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&directory).unwrap();
    let app = CadApp::new(Session::default(), Services::default());
    let mut h = Harness::builder().with_size(vec2(1906.0, 200.0)).wgpu().build_ui_state(
        |ui, app: &mut CadApp| {
            titlerow::show(app, ui);
            ribbon::show(app, ui);
        },
        app,
    );
    h.run_steps(3);
    h.render().unwrap().save(directory.join("titlerow-1906.png")).unwrap();
    h.get_by_label("Customize Quick Access Toolbar").click();
    h.run_steps(3);
    h.render().unwrap().save(directory.join("titlerow-1906-customize.png")).unwrap();
}
