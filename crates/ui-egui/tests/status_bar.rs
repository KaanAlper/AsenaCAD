//! AutoCAD's status bar row: the band separating it from the drawing area, the layout bar (≡,
//! Model, the layouts, "+") and the default status bar items in AutoCAD's order, with the
//! Customization menu that shows the hidden ones and the ▾ menus wired to commands.

use asenacad_engine::Session;
use asenacad_engine::snap::mode;
use asenacad_geom::Vec2;
use asenacad_ui_egui::chrome::statusbar;
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::{Role, Toggled};
use egui::{Rect, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

const W: f32 = 1906.0;
const H: f32 = 1026.0;

fn harness_with(setup: impl FnOnce(&mut CadApp)) -> Harness<'static, CadApp> {
    sized(W, setup)
}

fn sized(width: f32, setup: impl FnOnce(&mut CadApp)) -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", json!({})).unwrap();
    app.ui.start_tab = false;
    app.integrated_titlebar = true;
    setup(&mut app);
    let mut h = Harness::builder().with_size(vec2(width, H)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.canvas.cursor = Some(Vec2::new(12.5, -3.25));
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h
}

fn harness() -> Harness<'static, CadApp> {
    harness_with(|_| {})
}

fn canvas(h: &Harness<'_, CadApp>) -> Rect {
    h.state().canvas.rect.expect("the drawing area is laid out")
}

/// The control labelled `label` in the status bar row (below the drawing area).
fn bar_node<'a>(h: &'a Harness<'_, CadApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let bottom = canvas(h).bottom();
    let mut nodes: Vec<_> = h.query_all_by_label(label).filter(|n| n.rect().min.y >= bottom - 0.5).collect();
    assert_eq!(nodes.len(), 1, "one {label} in the status bar row");
    nodes.remove(0)
}

fn in_bar(h: &Harness<'_, CadApp>, label: &str) -> bool {
    let bottom = canvas(h).bottom();
    h.query_all_by_label(label).any(|n| n.rect().min.y >= bottom - 0.5)
}

/// The menu row labelled `label` (a status bar menu's row, above the status bar row).
fn menu_row<'a>(h: &'a Harness<'_, CadApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let bottom = canvas(h).bottom();
    let row = asenacad_ui_egui::chrome::statusmenus::ROW;
    let mut nodes: Vec<_> = h.query_all_by_label(label).filter(|n| n.rect().max.y < bottom && (n.rect().width() - row.x).abs() < 0.5).collect();
    assert_eq!(nodes.len(), 1, "one menu row {label}");
    nodes.remove(0)
}

fn lefts(h: &Harness<'_, CadApp>, labels: &[&str]) -> Vec<f32> {
    labels.iter().map(|l| bar_node(h, l).rect().min.x).collect()
}

const DEFAULT_ITEMS: [&str; 16] = [
    "MODEL",
    "Grid",
    "Snap Mode",
    "Ortho Mode",
    "Polar Tracking",
    "Isometric Drafting",
    "Object Snap Tracking",
    "2D Object Snap",
    "Annotation Visibility",
    "AutoScale",
    "Annotation Scale",
    "Workspace Switching",
    "Annotation Monitor",
    "Isolate Objects",
    "Clean Screen",
    "Customization",
];

#[test]
fn the_layout_bar_reads_menu_model_layouts_then_plus() {
    let h = harness();
    let order = ["Layout list", "Model", "Layout1", "Layout2", "New layout"];
    let xs = lefts(&h, &order);
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "AutoCAD's order: {xs:?}");
    // One "+" only, after the last layout; the tabs are tabs and Model is current.
    assert_eq!(h.query_all_by_label("New layout").count(), 1);
    let model = h.get_by_role_and_label(Role::Tab, "Model");
    assert!(model.accesskit_node().is_selected() == Some(true));
    assert_eq!(h.get_by_role_and_label(Role::Tab, "Layout1").accesskit_node().is_selected(), Some(false));
    // The row is AutoCAD's 33 px tall and starts right under the drawing area.
    let row = bar_node(&h, "Layout list").rect();
    assert!((row.center().y - (canvas(&h).bottom() + statusbar::BAND_H + (statusbar::HEIGHT - statusbar::BAND_H) / 2.0)).abs() < 1.0, "{row:?}");
}

#[test]
fn clicking_a_layout_tab_and_plus_switch_and_add_layouts() {
    let mut h = harness();
    bar_node(&h, "Layout2").click();
    h.run_steps(3);
    assert_eq!(h.state().session.layout_space(), asenacad_doc::Space::Paper("Layout2".into()));
    bar_node(&h, "New layout").click();
    h.run_steps(3);
    assert_eq!(h.state().session.layout_space(), asenacad_doc::Space::Paper("Layout3".into()));
    let xs = lefts(&h, &["Model", "Layout1", "Layout2", "Layout3", "New layout"]);
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "the new layout comes before +: {xs:?}");
}

#[test]
fn the_status_bar_shows_autocads_default_items_in_order() {
    let h = harness();
    let xs = lefts(&h, &DEFAULT_ITEMS);
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "AutoCAD's order: {xs:?}");
    // The items sit at the right end of the row, right of the layout bar.
    assert!(xs[0] > bar_node(&h, "New layout").rect().right());
    assert!(bar_node(&h, "Customization").rect().right() >= W - 16.0);
    for hidden in [
        "Coordinates",
        "Dynamic Input",
        "LineWeight",
        "Transparency",
        "Quick Properties",
        "Infer Constraints",
        "Selection Cycling",
        "Units",
        "Lock UI",
        "Drafting Settings",
        "Show/Hide Lineweight",
    ] {
        assert!(!in_bar(&h, hidden), "{hidden} is hidden by default");
    }
    // Commands AsenaCAD doesn't have yet show disabled.
    for missing in ["Workspace Switching", "Annotation Monitor", "Clean Screen"] {
        assert!(bar_node(&h, missing).accesskit_node().is_disabled(), "{missing} has no command yet");
    }
    for ready in ["Grid", "Snap Mode", "2D Object Snap", "Isolate Objects", "AutoScale"] {
        assert!(!bar_node(&h, ready).accesskit_node().is_disabled(), "{ready} works");
    }
}

#[test]
fn toggles_show_their_state_and_switch_it() {
    let mut h = harness_with(|app| app.session.settings.gridmode = false);
    assert_eq!(bar_node(&h, "Grid").accesskit_node().toggled(), Some(Toggled::False));
    bar_node(&h, "Grid").click();
    h.run_steps(3);
    assert!(h.state().session.settings.gridmode);
    assert_eq!(bar_node(&h, "Grid").accesskit_node().toggled(), Some(Toggled::True));
    let before = h.state().session.settings.annoautoscale;
    bar_node(&h, "AutoScale").click();
    h.run_steps(3);
    assert_ne!(h.state().session.settings.annoautoscale, before, "AutoScale sets ANNOAUTOSCALE");
}

#[test]
fn customization_lists_every_item_in_autocads_order_and_shows_hidden_ones() {
    let mut h = harness();
    bar_node(&h, "Customization").click();
    h.run_steps(3);
    let tops: Vec<f32> = statusbar::ITEMS.iter().map(|it| menu_row(&h, it.label).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "AutoCAD's order: {tops:?}");
    assert_eq!(menu_row(&h, "Grid").accesskit_node().toggled(), Some(Toggled::True));
    assert_eq!(menu_row(&h, "Dynamic Input").accesskit_node().toggled(), Some(Toggled::False));
    // Checking Dynamic Input and Coordinates shows them in place; the menu stays open.
    menu_row(&h, "Dynamic Input").click();
    h.run_steps(3);
    menu_row(&h, "Coordinates").click();
    h.run_steps(3);
    assert!(h.state().ui.status_shown.iter().any(|k| k == "dynmode"));
    let xs = lefts(&h, &["Coordinates", "MODEL", "Snap Mode", "Dynamic Input", "Ortho Mode"]);
    assert!(xs.windows(2).all(|w| w[0] < w[1]), "in the Customization list's order: {xs:?}");
    // Unchecking hides it again.
    menu_row(&h, "Dynamic Input").click();
    h.run_steps(3);
    assert!(!in_bar(&h, "Dynamic Input"));
    // The command validates its item.
    assert!(h.state_mut().run("ui.statusbar", json!({"item": "nope"})).is_err());
    assert_eq!(h.state_mut().run("ui.statusbar", json!({"item": "units", "on": true})), Ok(json!({"item": "units", "on": true})));
}

#[test]
fn object_snap_menu_checks_osmode_bits_in_autocads_order() {
    let mut h = harness_with(|app| app.session.settings.osmode = mode::END | mode::OFF);
    bar_node(&h, "2D Object Snap options").click();
    h.run_steps(3);
    let tops: Vec<f32> =
        std::iter::once("Object Snap Settings...").chain(mode::ALL.iter().map(|(_, l)| *l)).map(|l| menu_row(&h, l).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "{tops:?}");
    assert_eq!(menu_row(&h, "Endpoint").accesskit_node().toggled(), Some(Toggled::True));
    menu_row(&h, "Tangent").click();
    h.run_steps(3);
    let osmode = h.state().session.settings.osmode;
    assert_eq!(osmode, mode::END | mode::TAN, "checking a mode adds it and turns running snaps on");
}

#[test]
fn polar_snap_and_isodraft_menus_set_their_variables() {
    let mut h = harness();
    bar_node(&h, "Polar Tracking options").click();
    h.run_steps(3);
    menu_row(&h, "45, 90, 135, 180...").click();
    h.run_steps(3);
    assert!((h.state().session.settings.polarang.to_degrees() - 45.0).abs() < 1e-9);
    bar_node(&h, "Snap Mode options").click();
    h.run_steps(3);
    menu_row(&h, "Polar Snap").click();
    h.run_steps(3);
    assert!(h.state().session.settings.polarsnap);
    bar_node(&h, "Isometric Drafting options").click();
    h.run_steps(3);
    menu_row(&h, "Isoplane Right").click();
    h.run_steps(3);
    let s = &h.state().session.settings;
    assert!(s.isodraft && s.isoplane.name().eq_ignore_ascii_case("right"));
    bar_node(&h, "Snap Mode options").click();
    h.run_steps(3);
    menu_row(&h, "Snap Settings...").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("dsettings"));
}

#[test]
fn model_returns_to_the_last_layout_and_toggles_paper_space_there() {
    let mut h = harness();
    bar_node(&h, "Layout2").click();
    h.run_steps(3);
    bar_node(&h, "Model").click();
    h.run_steps(3);
    assert_eq!(h.state().session.layout_space(), asenacad_doc::Space::Model);
    bar_node(&h, "MODEL").click();
    h.run_steps(3);
    assert_eq!(h.state().session.layout_space(), asenacad_doc::Space::Paper("Layout2".into()), "back to the last layout");
    assert!(in_bar(&h, "PAPER"));
}

#[test]
fn a_separator_band_spans_the_window_between_the_drawing_and_the_bar() {
    let h = harness();
    let c = canvas(&h);
    let band_color = asenacad_ui_egui::theme::Tokens::get().status_band;
    let band = h.output().shapes.iter().find_map(|s| match &s.shape {
        egui::Shape::Rect(r) if r.fill == band_color && (r.rect.height() - statusbar::BAND_H).abs() < 0.5 => Some(r.rect),
        _ => None,
    });
    let band = band.expect("the band is painted");
    assert!((band.top() - c.bottom()).abs() <= 1.0 && band.width() >= c.width(), "{band:?} under {c:?}");
}

/// The status bar row's controls (buttons and tabs below the drawing area): label and rect.
fn bar_controls(h: &Harness<'_, CadApp>) -> Vec<(String, Rect)> {
    let bottom = canvas(h).bottom();
    let mut out = Vec::new();
    for role in [Role::Button, Role::Tab, Role::TextInput] {
        for n in h.query_all_by_role(role) {
            let r = n.rect();
            if r.min.y >= bottom - 0.5 {
                out.push((n.accesskit_node().label().unwrap_or_default().to_string(), r));
            }
        }
    }
    out
}

#[test]
fn a_narrow_window_drops_items_from_the_left_and_never_overlaps_the_layout_bar() {
    for width in [700.0, 900.0] {
        for paper in [false, true] {
            let mut h = sized(width, |app| {
                app.ui.status_shown.insert(0, "coordinates".into());
                app.ui.status_shown.push("dynmode".into());
                if paper {
                    app.run("layout.set", json!({"name": "Layout2"})).unwrap();
                }
            });
            h.run_steps(2);
            let at = format!("{width} pt, paper {paper}");
            let controls = bar_controls(&h);
            for (i, (a, ra)) in controls.iter().enumerate() {
                assert!(ra.left() >= 0.0 && ra.right() <= width, "{at}: {a} {ra:?} leaves the window");
                for (b, rb) in controls.iter().skip(i + 1) {
                    assert!(!ra.shrink(0.5).intersects(rb.shrink(0.5)), "{at}: {a} {ra:?} overlaps {b} {rb:?}");
                }
            }
            // The layout bar's minimum and Customization always stay.
            for label in ["Layout list", "Model", "New layout", "Customization"] {
                assert!(controls.iter().any(|(l, _)| l == label), "{at}: {label} shows: {controls:?}");
            }
            if paper {
                assert!(controls.iter().any(|(l, _)| l == "Layout2"), "{at}: the current layout shows");
            }
            // What doesn't fit goes from the group's left end: Coordinates first.
            let shown: Vec<&str> = statusbar::ITEMS.iter().filter(|it| controls.iter().any(|(l, _)| l == it.label)).map(|it| it.label).collect();
            assert!(!shown.contains(&"Coordinates"), "{at}: {shown:?}");
            // The controls left work: 2D Object Snap still toggles running snaps.
            if controls.iter().any(|(l, _)| l == "2D Object Snap") {
                let before = h.state().session.settings.osmode;
                bar_node(&h, "2D Object Snap").click();
                h.run_steps(3);
                assert_ne!(h.state().session.settings.osmode, before, "{at}: 2D Object Snap is clickable");
            }
            bar_node(&h, "Model").click();
            h.run_steps(3);
            assert_eq!(h.state().session.layout_space(), asenacad_doc::Space::Model, "{at}: Model is clickable");
        }
    }
}

#[test]
fn object_snap_with_no_modes_set_opens_the_object_snap_settings() {
    let mut h = harness_with(|app| app.session.settings.osmode = 0);
    assert_eq!(bar_node(&h, "2D Object Snap").accesskit_node().toggled(), Some(Toggled::False));
    bar_node(&h, "2D Object Snap").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("dsettings"), "Drafting Settings opens");
    assert_eq!(h.state().session.settings.osmode, 0, "OSMODE is left alone");
    // With modes set the button toggles them again.
    h.state_mut().ui.dialog = None;
    h.state_mut().session.settings.osmode = mode::END;
    h.run_steps(2);
    bar_node(&h, "2D Object Snap").click();
    h.run_steps(3);
    assert_eq!(h.state().session.settings.osmode, mode::END | mode::OFF);
    assert!(h.state().ui.dialog.is_none());
}

/// The x of the painted text `text`, if the status bar row paints it.
fn painted_at(h: &Harness<'_, CadApp>, text: &str) -> Option<f32> {
    fn find(shape: &egui::Shape, text: &str) -> Option<f32> {
        match shape {
            egui::Shape::Text(t) if t.galley.text() == text => Some(t.pos.x),
            egui::Shape::Vec(shapes) => shapes.iter().find_map(|s| find(s, text)),
            _ => None,
        }
    }
    h.output().shapes.iter().find_map(|s| find(&s.shape, text))
}

#[test]
fn layout_tabs_off_hides_the_layout_bar_and_the_readout_starts_at_the_left() {
    let h = harness_with(|app| {
        let r = app.run("ui.toggle.layouttabs", json!({ "on": false }));
        assert_eq!(r, Ok(json!({ "on": false })));
        app.set_status("Layout tabs are hidden");
    });
    for label in ["Layout list", "Model", "Layout1", "Layout2", "New layout"] {
        assert!(!in_bar(&h, label), "{label} is hidden with the layout tabs");
    }
    assert!(in_bar(&h, "MODEL"), "the status bar items stay");
    let x = painted_at(&h, "Layout tabs are hidden").expect("the readout is painted");
    assert!(x < 20.0, "the readout starts at the row's left end: {x}");
    // Without `on` the command flips the tabs back and reports it.
    let mut h = h;
    assert_eq!(h.state_mut().run("ui.toggle.layouttabs", serde_json::Value::Null), Ok(json!({ "on": true })));
    h.run_steps(3);
    assert!(in_bar(&h, "Layout1"));
}
