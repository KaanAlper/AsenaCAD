//! The Layer Properties Manager palette (LAYER): its title strip, toolbar, filter tree, grid and
//! status line in AutoCAD 2026's order, and every enabled control acting through an engine command.

use asenacad_engine::Session;
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::{Key, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable, by};
use serde_json::json;

/// A drawing with layers 0, Defpoints (the default drawing has it), Doors, Walls and Windows,
/// and a line on Walls, the only layer in use.
fn drawing() -> CadApp {
    let mut app = CadApp::new(Session::default(), Services::default());
    for name in ["Walls", "Doors", "Windows"] {
        app.run("layer.new", json!({ "name": name })).unwrap();
    }
    app.run("layer.current", json!({ "name": "Walls" })).unwrap();
    app.run("line", json!({ "points": [[0, 0], [10, 0]] })).unwrap();
    app.run("layer.current", json!({ "name": "0" })).unwrap();
    app
}

/// The palette alone, as LAYER opens it.
fn palette(mut app: CadApp) -> Harness<'static, CadApp> {
    assert!(app.cmdline("LAYER").is_none());
    assert_eq!(app.ui.dialog.as_deref(), Some("layers"));
    // 60 frames a second: the default step spreads two clicks past the double-click delay.
    let mut h = Harness::builder().with_size(vec2(1000.0, 700.0)).with_step_dt(1.0 / 60.0).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            let mut open = app.ui.dialog.as_deref() == Some("layers");
            if open {
                asenacad_ui_egui::layers::dialog(app, ui.ctx(), &mut open);
                if !open {
                    app.ui.dialog = None;
                }
            }
        },
        app,
    );
    h.run_steps(4);
    h
}

/// Two clicks in a row at `at`, half a second after any earlier click (so they don't count as
/// its second and third).
fn double_click(h: &mut Harness<'_, CadApp>, at: egui::Pos2) {
    h.event(egui::Event::PointerMoved(at));
    h.run_steps(30);
    for _ in 0..2 {
        for pressed in [true, false] {
            h.event(egui::Event::PointerButton { pos: at, button: egui::PointerButton::Primary, pressed, modifiers: egui::Modifiers::NONE });
            h.run_steps(1);
        }
    }
    h.run_steps(2);
}

/// Type into the text box showing `value` (the search box shows "").
fn type_into(h: &mut Harness<'_, CadApp>, value: &str, text: &str) {
    h.get(by().role(Role::TextInput).value(value)).focus();
    h.run_steps(2);
    h.get(by().role(Role::TextInput).value(value)).type_text(text);
    h.run_steps(2);
}

fn layer(app: &CadApp, name: &str) -> Option<asenacad_engine::doc::Layer> {
    app.session.doc().unwrap().layers.iter().find(|l| l.name == name).cloned()
}

/// The grid's layer names, top to bottom.
fn rows(h: &Harness<'_, CadApp>) -> Vec<String> {
    let mut cells: Vec<(f32, String)> =
        h.get_all(by().role(Role::Cell)).map(|n| (n.rect().top(), n.accesskit_node().label().unwrap_or_default())).collect();
    cells.sort_by(|a, b| a.0.total_cmp(&b.0));
    cells.into_iter().map(|(_, l)| l).collect()
}

fn status_line(h: &Harness<'_, CadApp>) -> String {
    h.get(by().role(Role::Label).predicate(|n| n.value().is_some_and(|v| v.contains("layers displayed")))).value().unwrap_or_default()
}

#[test]
fn toolbar_has_autocads_buttons_in_order_and_disables_missing_commands() {
    let h = palette(drawing());
    let names = [
        "New Property Filter",
        "New Group Filter",
        "Layer States Manager",
        "New Layer",
        "New Layer VP Frozen in All Viewports",
        "Delete Layer",
        "Set Current",
        "Refresh",
        "Settings",
    ];
    let nodes: Vec<_> = names.iter().map(|n| h.get(by().role(Role::Button).label(n))).collect();
    let xs: Vec<f32> = nodes.iter().map(|n| n.rect().left()).collect();
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "toolbar order: {xs:?}");
    let y = nodes[0].rect().center().y;
    assert!(nodes.iter().all(|n| (n.rect().center().y - y).abs() < 1.0), "one toolbar row");
    for (name, node) in names.iter().zip(&nodes) {
        let missing = ["New Property Filter", "New Group Filter", "New Layer VP Frozen in All Viewports", "Settings"].contains(name);
        assert_eq!(node.accesskit_node().is_disabled(), missing, "{name}");
    }
    // The top row: the current layer at the left, the search box at the right.
    let current = h.get(by().role(Role::Label).value("Current layer: 0"));
    let search = h.get(by().role(Role::TextInput));
    assert!(current.rect().right() < search.rect().left() && (current.rect().center().y - search.rect().center().y).abs() < 4.0);
    assert!(search.rect().bottom() < nodes[0].rect().top(), "search above the toolbar");
}

#[test]
fn grid_columns_follow_autocad_and_plot_style_shows_only_in_a_layout() {
    let model = ["Status", "Name", "On", "Freeze", "Lock", "Plot", "Color", "Linetype", "Lineweight", "Transparency", "New VP Freeze", "Description"];
    let headers = |h: &Harness<'_, CadApp>| {
        let mut v: Vec<(f32, String)> =
            h.get_all(by().role(Role::ColumnHeader)).map(|n| (n.rect().left(), n.accesskit_node().label().unwrap_or_default())).collect();
        v.sort_by(|a, b| a.0.total_cmp(&b.0));
        v.into_iter().map(|(_, l)| l).collect::<Vec<_>>()
    };
    let mut h = palette(drawing());
    // Wide enough for the extra column.
    h.ctx.data_mut(|d| d.insert_persisted(egui::Id::new("lpm_rect"), [10.0f32, 10.0, 900.0, 400.0]));
    h.run_steps(2);
    assert_eq!(headers(&h), model);
    h.state_mut().run("layout.set", json!({ "name": "Layout1" })).unwrap();
    h.run_steps(3);
    let mut layout: Vec<&str> = model.to_vec();
    layout.insert(10, "Plot Style");
    assert_eq!(headers(&h), layout);
}

#[test]
fn rows_list_the_layers_and_status_marks_the_current_one() {
    let h = palette(drawing());
    assert_eq!(rows(&h), ["0", "Defpoints", "Doors", "Walls", "Windows"]);
    let current = h.get(by().label("Current layer"));
    let zero = h.get(by().role(Role::Cell).label("0"));
    assert!((current.rect().center().y - zero.rect().center().y).abs() < 1.0, "the check is on layer 0's row");
    assert_eq!(status_line(&h), "All: 5 layers displayed of 5 total layers");
}

#[test]
fn search_box_filters_the_grid() {
    let mut h = palette(drawing());
    type_into(&mut h, "", "w*");
    assert_eq!(rows(&h), ["Walls", "Windows"]);
    assert_eq!(status_line(&h), "All: 2 layers displayed of 5 total layers");
}

#[test]
fn used_layers_filter_and_invert() {
    let mut h = palette(drawing());
    h.get(by().label("All Used Layers")).click();
    h.run_steps(3);
    assert_eq!(rows(&h), ["Walls"]);
    assert_eq!(status_line(&h), "All Used Layers: 1 layers displayed of 5 total layers");
    h.get(by().role(Role::CheckBox).label("Invert")).click();
    h.run_steps(3);
    assert_eq!(rows(&h), ["0", "Defpoints", "Doors", "Windows"]);
    assert_eq!(status_line(&h), "All Used Layers: 4 layers displayed of 5 total layers");
}

#[test]
fn new_layer_set_current_and_delete_layer_run_their_commands() {
    let mut h = palette(drawing());
    h.get(by().role(Role::Button).label("New Layer")).click();
    h.run_steps(3);
    assert!(layer(h.state(), "Layer1").is_some());
    // The new layer's name is ready to type over.
    h.get(by().role(Role::TextInput).value("Layer1")).type_text("Stairs");
    h.run_steps(2);
    h.key_press(Key::Enter);
    h.run_steps(3);
    assert!(layer(h.state(), "Layer1").is_none(), "renamed");
    assert!(layer(h.state(), "Stairs").is_some(), "the typed name replaces the proposed one");

    h.get(by().role(Role::Cell).label("Doors")).click();
    h.run_steps(2);
    h.get(by().role(Role::Button).label("Set Current")).click();
    h.run_steps(3);
    assert_eq!(h.state().session.doc().unwrap().header.str("CLAYER", "0"), "Doors");
    h.get(by().role(Role::Label).value("Current layer: Doors"));

    h.get(by().role(Role::Cell).label("Windows")).click();
    h.run_steps(2);
    h.get(by().role(Role::Button).label("Delete Layer")).click();
    h.run_steps(3);
    assert!(layer(h.state(), "Windows").is_none());
}

#[test]
fn row_toggles_and_rename_act_on_the_layer() {
    let mut h = palette(drawing());
    let on: Vec<_> = {
        let mut v: Vec<_> = h.get_all(by().role(Role::Button).label("On")).map(|n| n.rect()).collect();
        v.sort_by(|a, b| a.top().total_cmp(&b.top()));
        v
    };
    // Rows: 0, Defpoints, Doors, Walls, Windows.
    let walls_on = on[3];
    h.get_all(by().role(Role::Button).label("On")).find(|n| n.rect() == walls_on).unwrap().click();
    h.run_steps(3);
    assert!(!layer(h.state(), "Walls").unwrap().on);
    h.get_all(by().role(Role::Button).label("Freeze")).find(|n| (n.rect().top() - walls_on.top()).abs() < 1.0).unwrap().click();
    h.run_steps(3);
    assert!(layer(h.state(), "Walls").unwrap().frozen);
    // Double-click renames.
    let doors = h.get(by().role(Role::Cell).label("Doors")).rect().center();
    double_click(&mut h, doors);
    h.get(by().role(Role::TextInput).value("Doors")).type_text("Gates");
    h.run_steps(2);
    h.key_press(Key::Enter);
    h.run_steps(3);
    assert!(layer(h.state(), "Gates").is_some() && layer(h.state(), "Doors").is_none());
}

#[test]
fn title_strip_close_button_closes_the_palette() {
    let mut h = palette(drawing());
    h.get(by().role(Role::Button).label("Close")).click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog, None);
}

#[test]
fn opens_at_the_top_left_of_the_drawing_area_650_by_350() {
    let mut app = drawing();
    app.ui.in_window_menu = false;
    let mut h = Harness::builder().with_size(vec2(1600.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(4);
    h.state_mut().cmdline("LAYER");
    h.run_steps(4);
    let canvas = h.state().canvas.rect.unwrap();
    let area = h.ctx.memory(|m| m.area_rect(egui::Id::new("lpm_window"))).unwrap();
    assert!((area.width() - 650.0).abs() < 1.0 && (area.height() - 350.0).abs() < 1.0, "{area:?}");
    assert!(canvas.contains(area.min) && area.min.x - canvas.min.x < 20.0 && area.min.y - canvas.min.y < 20.0, "{area:?} in {canvas:?}");
}
