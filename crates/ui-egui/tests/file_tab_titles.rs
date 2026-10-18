//! AutoCAD's document tab bar: "≡" menu, Start (no ×), the drawings' tabs (name, "*" while
//! modified, × inside) and "+". A long drawing name is cut short with "…" inside its tab instead
//! of running under the next tab and the close button (#513); the unsaved-changes mark stays
//! visible.

use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::{Rect, Shape, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};

/// Painted texts: the full source text, the painted bounds, and whether it was cut short.
fn texts(shape: &Shape, out: &mut Vec<(String, Rect, bool)>) {
    match shape {
        Shape::Text(t) => out.push((t.galley.text().to_string(), t.visual_bounding_rect(), t.galley.elided)),
        Shape::Vec(v) => v.iter().for_each(|s| texts(s, out)),
        _ => {}
    }
}

#[test]
fn long_names_are_cut_short_inside_the_tab() {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", serde_json::json!({})).unwrap();
    app.run("circle", serde_json::json!({"center": [0, 0], "radius": 1})).unwrap();
    app.session.state_mut().unwrap().title = "LICENCIA COFRE DE PEROTE REPOSICION FINAL.dwg".into();
    app.ui.start_tab = false;
    let mut h = Harness::builder().with_size(vec2(1200.0, 760.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    let mut found = Vec::new();
    for s in &h.output().shapes {
        texts(&s.shape, &mut found);
    }
    // The window title shows the whole name too; the tab's is the lower one.
    let (_, rect, cut) =
        found.iter().filter(|(t, ..)| t.starts_with("LICENCIA")).max_by(|a, b| a.1.top().total_cmp(&b.1.top())).cloned().expect("the tab's name");
    assert!(cut, "the name ends in an ellipsis");
    // Tabs are 170 points wide: 8 of padding, then the name, the mark and the close button (24).
    assert!(rect.width() <= 170.0 - 8.0 - 24.0, "{} points wide", rect.width());
    let mark = found.iter().find(|(t, r, _)| t == "*" && (r.left() - rect.right()).abs() < 4.0);
    assert!(mark.is_some(), "the unsaved mark follows the shortened name");
    // A short name is not cut.
    assert!(found.iter().any(|(t, _, cut)| t == "Start" && !cut));
}

fn app_with_two_drawings() -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", serde_json::json!({})).unwrap();
    app.run("new", serde_json::json!({})).unwrap();
    app.run("circle", serde_json::json!({"center": [0, 0], "radius": 1})).unwrap();
    app.ui.start_tab = false;
    let mut h = Harness::builder().with_size(vec2(1280.0, 760.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h
}

#[test]
fn the_tab_bar_has_autocads_order_and_start_has_no_close_button() {
    let mut h = app_with_two_drawings();
    let titles: Vec<String> = h.state().session.docs.iter().map(|d| d.title.clone()).collect();
    assert!(titles.len() >= 2 && h.state().session.active == titles.len() - 1);
    let titles = [titles[titles.len() - 2].clone(), titles[titles.len() - 1].clone()];
    let first = h.state().session.active - 1;
    let left = |h: &Harness<'_, CadApp>, label: &str| h.get_by_label(label).rect().left();
    let order = [left(&h, "File Tabs"), left(&h, "Start"), left(&h, &titles[0]), left(&h, &titles[1]), left(&h, "New Drawing")];
    assert!(order.windows(2).all(|w| w[0] < w[1]), "≡, Start, drawings, +: {order:?}");
    assert_eq!(h.get_by_role_and_label(Role::Tab, &titles[1]).accesskit_node().is_selected(), Some(true), "the current drawing's tab is selected");
    // The current drawing has unsaved changes: "*" follows its name; its × is inside the tab.
    let tab = h.get_by_role_and_label(Role::Tab, &titles[1]).rect();
    let close = h.get_by_label(&format!("Close {}", titles[1])).rect();
    assert!(tab.contains_rect(close), "the × is inside the tab: {close:?} in {tab:?}");
    let mut found = Vec::new();
    for s in &h.output().shapes {
        texts(&s.shape, &mut found);
    }
    assert!(found.iter().any(|(t, r, _)| t == "*" && tab.contains(r.center())), "the unsaved mark");
    assert!(h.query_by_label("Close Start").is_none(), "Start has no ×");
    h.get_by_role_and_label(Role::Tab, "Start").click();
    h.run_steps(3);
    assert!(h.state().ui.start_tab);
    // ≡ lists Start and the drawings; choosing one goes back to it.
    h.get_by_label("File Tabs").click();
    h.run_steps(3);
    let menu_row =
        |h: &Harness<'_, CadApp>, label: &str| h.query_all_by_label(label).max_by(|a, b| a.rect().top().total_cmp(&b.rect().top())).map(|n| n.rect());
    assert!(menu_row(&h, &format!("{}*", titles[1])).is_some(), "the menu marks the modified drawing");
    h.query_all_by_label(&titles[0]).max_by(|a, b| a.rect().top().total_cmp(&b.rect().top())).unwrap().click();
    h.run_steps(3);
    assert!(!h.state().ui.start_tab);
    assert_eq!(h.state().session.active, first);
}
