//! The Properties palette (PROPERTIES, Ctrl+1): docked left of the drawing with a vertical title
//! strip, the object list and its three buttons, and AutoCAD 2026's groups and rows.

use asenacad_engine::Session;
use asenacad_ui_egui::{CadApp, Services, docking::Panel};
use egui::accesskit::Role;
use egui::vec2;
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable, by};
use serde_json::json;

/// The palette alone, as the docking area shows it.
fn palette(app: CadApp) -> Harness<'static, CadApp> {
    let mut h = Harness::builder().with_size(vec2(270.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            asenacad_ui_egui::palettes::properties::panel(app, ui);
        },
        app,
    );
    h.run_steps(4);
    h
}

fn app() -> CadApp {
    CadApp::new(Session::default(), Services::default())
}

/// Group headers (headings) and row names (row headers), top to bottom.
fn layout(h: &Harness<'_, CadApp>) -> Vec<String> {
    let mut v: Vec<(f32, String)> = h
        .get_all(by().predicate(|n| matches!(n.role(), Role::RowHeader | Role::Heading)))
        .map(|n| (n.rect().top(), n.accesskit_node().label().unwrap_or_default()))
        .collect();
    v.sort_by(|a, b| a.0.total_cmp(&b.0));
    v.into_iter().map(|(_, l)| l).collect()
}

fn row<'h>(h: &'h Harness<'_, CadApp>, name: &'h str) -> egui_kittest::Node<'h> {
    h.get(by().role(Role::RowHeader).label(name))
}

#[test]
fn no_selection_shows_autocads_groups_and_rows_in_order() {
    let h = palette(app());
    let expected = [
        "General",
        "Color",
        "Layer",
        "Linetype",
        "Linetype scale",
        "Lineweight",
        "Transparency",
        "Thickness",
        "3D Visualization",
        "Material",
        "Plot style",
        "Plot style",
        "Plot style table",
        "Plot table attached to",
        "Plot table type",
        "View",
        "Center X",
        "Center Y",
        "Center Z",
        "Height",
        "Width",
        "Misc",
        "Annotation scale",
        "UCS icon On",
        "UCS icon at origin",
    ];
    assert_eq!(layout(&h), expected);
}

#[test]
fn object_list_then_pickadd_select_objects_and_quick_select() {
    let h = palette(app());
    let list = h.get(by().role(Role::ComboBox).label("No selection"));
    let buttons: Vec<_> =
        ["Toggle value of PICKADD Sysvar", "Select Objects", "Quick Select"].iter().map(|n| h.get(by().role(Role::Button).label(n))).collect();
    assert!(list.rect().right() <= buttons[0].rect().left() + 0.5);
    assert!(buttons.windows(2).all(|w| w[0].rect().left() < w[1].rect().left()));
    assert!(buttons.iter().all(|b| (b.rect().center().y - list.rect().center().y).abs() < 3.0), "one row");
    // Above the first group.
    let general = h.get(by().role(Role::Heading).label("General"));
    assert!(list.rect().bottom() <= general.rect().top());
}

#[test]
fn pickadd_button_toggles_the_variable() {
    let mut h = palette(app());
    assert!(h.state().session.settings.pickadd);
    h.get(by().role(Role::Button).label("Toggle value of PICKADD Sysvar")).click();
    h.run_steps(3);
    assert!(!h.state().session.settings.pickadd);
    h.get(by().role(Role::Button).label("Toggle value of PICKADD Sysvar")).click();
    h.run_steps(3);
    assert!(h.state().session.settings.pickadd);
}

#[test]
fn select_objects_and_quick_select_start_their_commands() {
    let mut h = palette(app());
    h.get(by().role(Role::Button).label("Quick Select")).click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("qselect"));
    h.get(by().role(Role::Button).label("Select Objects")).click();
    h.run_steps(3);
    assert!(h.state().session.prompting(), "SELECT is waiting for objects");
}

#[test]
fn read_only_rows_are_greyed() {
    let h = palette(app());
    for name in ["Center X", "Center Y", "Center Z", "Height", "Width", "Plot table attached to", "Plot table type", "UCS icon at origin", "Material"]
    {
        assert!(row(&h, name).accesskit_node().is_disabled(), "{name} is read-only");
    }
    for name in ["Color", "Layer", "Linetype", "Linetype scale", "Lineweight", "Thickness", "Annotation scale", "UCS icon On"] {
        assert!(!row(&h, name).accesskit_node().is_disabled(), "{name} is editable");
    }
    assert_eq!(row(&h, "Plot table attached to").value().as_deref(), Some("Model"));
    assert_eq!(row(&h, "Annotation scale").value().as_deref(), Some("1:1"));
}

#[test]
fn view_rows_show_the_current_view() {
    let mut app = app();
    app.run("view.set", json!({ "center": [40, 25], "height": 18 })).unwrap();
    let h = palette(app);
    assert_eq!(row(&h, "Center X").value().as_deref(), Some("40.0000"));
    assert_eq!(row(&h, "Center Y").value().as_deref(), Some("25.0000"));
    assert_eq!(row(&h, "Center Z").value().as_deref(), Some("0.0000"));
    assert_eq!(row(&h, "Height").value().as_deref(), Some("18.0000"));
}

#[test]
fn groups_collapse_and_expand() {
    let mut h = palette(app());
    h.get(by().role(Role::Heading).label("General")).click();
    h.run_steps(3);
    assert!(h.query(by().role(Role::RowHeader).label("Linetype scale")).is_none());
    assert!(h.query(by().role(Role::RowHeader).label("Material")).is_some());
    h.get(by().role(Role::Heading).label("General")).click();
    h.run_steps(3);
    row(&h, "Linetype scale");
}

#[test]
fn a_selection_lists_its_types_and_counts() {
    let mut app = app();
    let handle = |r: serde_json::Value| if r["handle"].is_string() { r["handle"].clone() } else { r["handles"][0].clone() };
    let a = handle(app.run("line", json!({ "points": [[0, 0], [10, 0]] })).unwrap());
    let b = handle(app.run("line", json!({ "points": [[0, 5], [10, 5]] })).unwrap());
    let c = handle(app.run("circle", json!({ "center": [0, 0], "radius": 3 })).unwrap());
    app.run("select", json!({ "handles": [a, b, c] })).unwrap();
    let mut h = palette(app);
    h.get(by().role(Role::ComboBox).label("All (3)"));
    h.state_mut().run("select", json!({ "handles": [a] })).unwrap();
    h.run_steps(3);
    h.get(by().role(Role::ComboBox).label("Line"));
    let rows = layout(&h);
    let general = rows.iter().position(|r| r == "General").unwrap();
    let geometry = rows.iter().position(|r| r == "Geometry").unwrap();
    assert!(general < geometry);
    let order: Vec<&str> = rows[geometry + 1..].iter().map(String::as_str).collect();
    assert_eq!(order, ["Start X", "Start Y", "End X", "End Y", "Delta X", "Delta Y", "Length", "Angle"]);
    assert!(row(&h, "Delta X").accesskit_node().is_disabled());
}

#[test]
fn properties_opens_docked_left_of_the_drawing_about_270_wide() {
    let mut app = app();
    app.ui.in_window_menu = false;
    app.ui.ribbon_min = asenacad_ui_egui::ribbon::RibbonMin::Tabs;
    let mut h = Harness::builder().with_size(vec2(1600.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    assert!(!h.state().ui.docking.layout.contains(&Panel::Properties), "closed by default, as in AutoCAD");
    h.state_mut().cmdline("PROPERTIES");
    h.run_steps(5);
    assert!(h.state().ui.docking.layout.contains(&Panel::Properties));
    let canvas = h.state().canvas.rect.unwrap();
    let list = h.get(by().role(Role::ComboBox).label("No selection"));
    assert!(list.rect().right() < canvas.left(), "the palette is left of the drawing");
    let close = h.get(by().role(Role::Button).label("Close"));
    let width = canvas.left() - close.rect().left();
    assert!((250.0..300.0).contains(&width), "about 270 wide: {width}");
}
