//! The Home ribbon's controls: buttons start their commands, split buttons list AutoCAD's
//! variants and remember the last one, unbuilt items are disabled, combo boxes show the drawing's
//! current values and toggles show their state. The whole ribbon (`ribbon::show`): tab row,
//! minimize states, panels packed in order, slide-outs and right-to-left collapse.

use cadcraft_engine::Session;
use cadcraft_ui_egui::docking::Panel as Palette;
use cadcraft_ui_egui::ribbon::home::HOME;
use cadcraft_ui_egui::ribbon::model::{Entry, Item, Panel};
use cadcraft_ui_egui::ribbon::{self, RibbonMin, layout, render_panel_for_test, widgets::RibbonState};
use cadcraft_ui_egui::{CadApp, Services};
use egui::accesskit::{Role, Toggled};
use egui::{pos2, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn panel(label: &str) -> &'static Panel {
    HOME.panels.iter().find(|p| p.label == label).expect("a Home panel")
}

fn harness(label: &str) -> Harness<'static, CadApp> {
    let panel = panel(label);
    let app = CadApp::new(Session::default(), Services::default());
    let mut harness = Harness::builder()
        .with_size(vec2(900.0, 420.0))
        .build_ui_state(move |ui, app: &mut CadApp| render_panel_for_test(ui, app, &HOME, panel), app);
    harness.run_steps(3);
    harness
}

fn running(h: &Harness<'_, CadApp>) -> Option<String> {
    h.state().session.running.as_ref().map(|r| r.id.clone())
}

#[test]
fn clicking_line_starts_the_line_command() {
    let mut h = harness("Draw");
    h.get_by_label("Line").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("line"));
}

#[test]
fn the_circle_menu_lists_autocads_variants_in_order_with_separators() {
    let mut h = harness("Draw");
    h.get_by_label("Circle options").click();
    h.run_steps(3);
    let entries = ["Center, Radius", "Center, Diameter", "2-Point", "3-Point", "Tan, Tan, Radius", "Tan, Tan, Tan"];
    let tops: Vec<f32> = entries.iter().map(|l| h.get_by_label(l).rect().min.y).collect();
    assert!(tops.windows(2).all(|w| w[0] < w[1]), "entries in AutoCAD's order: {tops:?}");
    let step = tops[1] - tops[0];
    // A separator adds at least half an item spacing to the plain step between entries.
    let gap = step + h.ctx.global_style().spacing.item_spacing.y / 2.0;
    assert!(tops[2] - tops[1] > gap, "a separator before 2-Point");
    assert!((tops[3] - tops[2] - step).abs() < 1.0, "no separator between 2-Point and 3-Point");
    assert!(tops[4] - tops[3] > gap, "a separator before Tan, Tan, Radius");
}

#[test]
fn choosing_2_point_keeps_the_circle_label_and_starts_circle_2p() {
    let mut h = harness("Draw");
    h.get_by_label("Circle options").click();
    h.run_steps(3);
    h.get_by_label("2-Point").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("circle.2p"));
    assert!(h.query_by_label("2-Point").is_none(), "the menu closes");
    assert!(h.query_by_role_and_label(Role::Button, "Circle").is_some(), "Circle keeps its label");
    let remembered = RibbonState::split(&h.ctx, "Home/Draw/2", 0);
    assert_eq!(remembered, 3, "the face (its icon) now follows the 2-Point entry");

    // The face runs the remembered variant.
    h.state_mut().session.cancel();
    h.get_by_label("Circle").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("circle.2p"));
}

#[test]
fn choosing_extend_in_the_trim_menu_puts_extend_on_the_face() {
    let mut h = harness("Modify");
    h.get_by_label("Trim options").click();
    h.run_steps(3);
    h.get_by_label("Extend").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("extend"));
    assert!(h.query_by_label("Trim").is_none(), "the face no longer reads Trim");
    assert!(h.query_by_role_and_label(Role::Button, "Extend").is_some(), "the face reads Extend");
    assert!(h.query_by_label("Extend options").is_some());
}

#[test]
fn split_buttons_add_no_unlabelled_accessibility_nodes() {
    for panel in ["Draw", "Modify", "Block", "Clipboard"] {
        let h = harness(panel);
        let unlabelled: Vec<_> =
            h.query_all_by_role(Role::Button).filter(|n| n.accesskit_node().label().is_none_or(|l| l.is_empty())).map(|n| n.rect()).collect();
        assert!(unlabelled.is_empty(), "{panel}: unlabelled buttons at {unlabelled:?}");
        assert!(h.query_all_by_role(Role::Unknown).next().is_none(), "{panel}: no anonymous nodes");
    }
}

#[test]
fn an_item_without_a_command_is_disabled() {
    let mut h = harness("Modify");
    h.get_by_label("Set to ByLayer").click();
    h.run_steps(3);
    assert!(h.get_by_label("Set to ByLayer").accesskit_node().is_disabled());
    assert_eq!(running(&h), None, "a disabled item runs nothing");
    assert!(!h.get_by_label("Change Space").accesskit_node().is_disabled(), "slide-out items with a command are enabled");
}

#[test]
fn the_layer_combo_shows_the_current_layer() {
    let h = harness("Layers");
    let combo = h.get_by_role_and_label(Role::ComboBox, "Layer");
    assert_eq!(combo.value().as_deref(), Some("0"));
    assert!(!combo.accesskit_node().is_disabled());
}

#[test]
fn the_properties_combos_show_the_current_properties() {
    let h = harness("Properties");
    let value = |label: &str| h.get_by_role_and_label(Role::ComboBox, label).value();
    assert_eq!(value("Object Color").as_deref(), Some("ByLayer"));
    assert_eq!(value("Linetype").as_deref(), Some("ByLayer"));
    assert_eq!(value("Lineweight").as_deref(), Some("ByLayer"));
    // No binding yet: shown disabled with its value.
    assert!(h.get_by_role_and_label(Role::ComboBox, "Transparency Mode").accesskit_node().is_disabled());
    assert!(h.get_by_role_and_label(Role::Slider, "Transparency").accesskit_node().is_disabled());
    let launcher = h
        .query_all_by_role_and_label(Role::Button, "Properties")
        .map(|n| n.rect().size())
        .find(|s| *s == cadcraft_ui_egui::ribbon::widgets::LAUNCHER);
    assert!(launcher.is_some(), "the panel's dialog launcher");
}

fn current_layer(h: &Harness<'_, CadApp>) -> String {
    h.state().session.doc().map(|d| d.header.str("CLAYER", "0")).unwrap_or_default()
}

#[test]
fn picking_a_layer_in_the_combo_makes_it_current_and_undo_restores_0() {
    let mut h = harness("Layers");
    h.state_mut().run("layer.new", json!({ "name": "Walls" })).unwrap();
    h.run_steps(2);
    assert_eq!(current_layer(&h), "0");
    h.get_by_role_and_label(Role::ComboBox, "Layer").click();
    h.run_steps(3);
    h.get_by_label("Walls").click();
    h.run_steps(3);
    assert_eq!(current_layer(&h), "Walls", "the entry runs layer.current");
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Layer").value().as_deref(), Some("Walls"));
    h.state_mut().run("u", json!({})).unwrap();
    h.run_steps(2);
    assert_eq!(current_layer(&h), "0", "undo reverts the combo's change");
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Layer").value().as_deref(), Some("0"));
}

#[test]
fn the_lineweight_list_selects_the_current_lineweight() {
    let mut h = harness("Properties");
    h.state_mut().run("lweight", json!({ "lineweight": 0.25 })).unwrap();
    h.run_steps(2);
    assert_eq!(h.get_by_role_and_label(Role::ComboBox, "Lineweight").value().as_deref(), Some("0.25 mm"));
    h.get_by_role_and_label(Role::ComboBox, "Lineweight").click();
    h.run_steps(3);
    assert_eq!(h.get_by_label("0.25 mm").accesskit_node().toggled(), Some(Toggled::True), "the current entry is the selected one");
    let selected: Vec<_> = ["ByLayer", "ByBlock", "Default", "0.00 mm", "0.30 mm", "2.11 mm"]
        .into_iter()
        .filter(|l| h.query_all_by_label(l).any(|n| n.accesskit_node().toggled() == Some(Toggled::True)))
        .collect();
    assert!(selected.is_empty(), "no other entry is selected: {selected:?}");
}

#[test]
fn the_properties_launcher_opens_the_properties_palette() {
    let mut h = harness("Properties");
    h.state_mut().run("ui.dock", json!({ "operation": "close", "panel": "properties" })).unwrap();
    h.run_steps(2);
    assert!(!h.state().ui.docking.layout.contains(&Palette::Properties));
    let launcher = h
        .query_all_by_role_and_label(Role::Button, "Properties")
        .find(|n| n.rect().size() == cadcraft_ui_egui::ribbon::widgets::LAUNCHER)
        .expect("the panel's dialog launcher");
    launcher.click();
    h.run_steps(3);
    assert!(h.state().ui.docking.layout.contains(&Palette::Properties), "the launcher runs PROPERTIES");
}

#[test]
fn the_paste_face_defaults_to_paste_and_cut_and_copy_clip_show_icons_only() {
    let h = harness("Clipboard");
    assert!(h.query_by_role_and_label(Role::Button, "Paste").is_some(), "the face shows Paste");
    assert!(h.query_by_label("Paste options").is_some());
    for label in ["Cut", "Copy Clip"] {
        let node = h.get_by_role_and_label(Role::Button, label);
        assert!(node.rect().width() <= 24.5, "{label} is icon-only, named for accessibility: {:?}", node.rect());
    }
}

/// The texts painted in the last frame, as laid out (a forced line break is a "\n").
fn painted(h: &Harness<'_, CadApp>) -> Vec<String> {
    fn collect(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(t) => out.push(t.galley.text().to_string()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| collect(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    h.output().shapes.iter().for_each(|s| collect(&s.shape, &mut out));
    out
}

#[test]
fn large_labels_break_at_words_like_autocad() {
    let h = harness("Block");
    assert!(painted(&h).iter().any(|t| t == "Detect and\nConvert"), "Detect and / Convert: {:?}", painted(&h));
    assert!(painted(&h).iter().any(|t| t == "Insert"), "a short label stays on one line");
    let h = harness("Layers");
    assert!(painted(&h).iter().any(|t| t == "Layer\nProperties"), "Layer / Properties");
    let h = harness("Properties");
    assert!(painted(&h).iter().any(|t| t == "Match\nProperties"), "Match / Properties");
}

#[test]
fn group_selection_shows_and_flips_pickstyle() {
    let mut h = harness("Groups");
    let toggled = |h: &Harness<'_, CadApp>| h.get_by_label("Group Selection On/Off").accesskit_node().toggled();
    assert_eq!(h.state().session.settings.pickstyle & 1, 1, "group selection starts on");
    assert_eq!(toggled(&h), Some(Toggled::True));
    h.get_by_label("Group Selection On/Off").click();
    h.run_steps(3);
    assert_eq!(h.state().session.settings.pickstyle & 1, 0);
    assert_eq!(toggled(&h), Some(Toggled::False));
}

#[test]
fn quick_calculator_shows_pressed_once_its_palette_is_open() {
    let mut h = harness("Utilities");
    let toggled = |h: &Harness<'_, CadApp>| h.get_by_label("Quick Calculator").accesskit_node().toggled();
    assert_eq!(toggled(&h), Some(Toggled::False));
    h.get_by_label("Quick Calculator").click();
    h.run_steps(3);
    assert_eq!(toggled(&h), Some(Toggled::True));
}

// ---- The whole ribbon ----

fn ribbon(width: f32, height: f32) -> Harness<'static, CadApp> {
    let app = CadApp::new(Session::default(), Services::default());
    let mut h = Harness::builder().with_size(vec2(width, height)).build_ui_state(|ui, app: &mut CadApp| ribbon::show(app, ui), app);
    h.run_steps(3);
    h
}

/// The ribbon's top edge (the harness's panel has a margin): the tab row's top.
fn ribbon_top(h: &Harness<'_, CadApp>) -> f32 {
    h.get_by_role_and_label(Role::Tab, "Home").rect().top() - layout::TAB_TOP
}

/// The panel title bars' band: panels start 1 px below the strip, which starts below the tab row.
fn in_title_band(h: &Harness<'_, CadApp>, r: egui::Rect) -> bool {
    let top = ribbon_top(h) + layout::TAB_ROW_H + layout::PANEL_TOP + layout::PANEL_H - layout::TITLE_H;
    r.top() >= top - 0.5 && r.bottom() <= top + layout::TITLE_H + 0.5
}

/// Where `panel`'s title bar is, when the panel is expanded in the strip.
fn title_rect(h: &Harness<'_, CadApp>, panel: &str) -> Option<egui::Rect> {
    h.query_all_by_label(panel).map(|n| n.rect()).find(|r| in_title_band(h, *r))
}

/// A collapsed panel's button: a panel-tall node labelled with the panel's name.
fn panel_button(h: &Harness<'_, CadApp>, panel: &str) -> Option<egui::Rect> {
    h.query_all_by_role_and_label(Role::Button, panel).map(|n| n.rect()).find(|r| (r.height() - layout::PANEL_H).abs() < 0.5)
}

fn click_panel_button(h: &Harness<'_, CadApp>, panel: &str) {
    let node = h.query_all_by_role_and_label(Role::Button, panel).find(|n| (n.rect().height() - layout::PANEL_H).abs() < 0.5);
    node.unwrap_or_else(|| panic!("{panel} is collapsed")).click();
}

/// The accessible labels of a panel's items (a split face shows its default entry's label).
fn item_labels(items: &[Item]) -> Vec<&'static str> {
    items
        .iter()
        .map(|it| match *it {
            Item::Button { label, .. } | Item::Toggle { label, .. } => label,
            Item::Split { label: Some(l), .. } => l,
            Item::Split { label: None, menu, default, .. } => match menu.get(default) {
                Some(Entry::Command { label, .. }) => label,
                _ => "",
            },
            Item::Combo { kind, .. } => kind.label(),
            Item::Slider { kind, .. } => kind.label(),
        })
        .collect()
}

#[test]
fn the_tab_row_shows_home_selected() {
    let h = ribbon(1906.0, 1026.0);
    let home = h.get_by_role_and_label(Role::Tab, "Home");
    assert_eq!(home.accesskit_node().is_selected(), Some(true));
    assert!(home.rect().height() <= layout::TAB_H + 0.5 && home.rect().top() < layout::TAB_ROW_H);
    assert_eq!(h.state().ui.ribbon_tab, "Home");
    let minimize = h.get_by_label("Minimize to Panel Buttons").rect();
    assert!(minimize.left() > home.rect().right(), "the minimize button follows the last tab");
}

#[test]
fn at_1906_px_all_ten_home_panels_are_expanded_in_order() {
    let h = ribbon(1906.0, 1026.0);
    let lefts: Vec<f32> = HOME.panels.iter().map(|p| title_rect(&h, p.label).unwrap_or_else(|| panic!("{} expanded", p.label)).left()).collect();
    assert!(lefts.windows(2).all(|w| w[0] < w[1]), "panels in AutoCAD's order: {lefts:?}");
    for p in HOME.panels {
        assert!(panel_button(&h, p.label).is_none(), "{} is not collapsed", p.label);
    }
    assert!(h.query_by_label("Match Properties").is_some() && h.query_by_label("Base").is_some());
    // Items of a narrow ribbon hide their labels; here Modify shows them.
    let mv = h.get_by_label("Move").rect();
    assert!(mv.width() > 40.0, "Move shows its label: {mv:?}");
}

#[test]
fn at_1280_px_the_rightmost_panels_collapse_into_panel_buttons() {
    let mut h = ribbon(1280.0, 1024.0);
    let collapsed: Vec<&Panel> = HOME.panels.iter().filter(|p| panel_button(&h, p.label).is_some()).collect();
    for p in HOME.panels {
        assert!(title_rect(&h, p.label).is_some() != panel_button(&h, p.label).is_some(), "{}: expanded or collapsed", p.label);
    }
    let names: Vec<_> = collapsed.iter().map(|p| p.label).collect();
    assert!(!names.is_empty(), "something collapses at 1280 px");
    let first = HOME.panels.len() - names.len();
    assert_eq!(names, HOME.panels[first..].iter().map(|p| p.label).collect::<Vec<_>>(), "collapse goes from the right");
    assert_eq!(names, ["Properties", "Groups", "Utilities", "Clipboard", "View"], "AutoCAD's collapsed panels at 1280 px");
    // Each panel button opens the whole panel (items, title, launcher) as a popup.
    for p in collapsed {
        click_panel_button(&h, p.label);
        h.run_steps(3);
        let button = panel_button(&h, p.label).unwrap();
        for label in item_labels(p.items) {
            let found = h.query_all_by_label(label).any(|n| n.rect().top() >= button.bottom() - 0.5);
            assert!(found, "{}'s popup shows {label}", p.label);
        }
        if let Some(cmd) = p.launcher {
            let label = cadcraft_engine::find_command(cmd).map_or(p.label, |c| c.label);
            assert!(h.query_all_by_label(label).any(|n| n.rect().width() <= 14.5), "{}: launcher", p.label);
        }
        assert!(h.query_all_by_label(p.label).count() >= 2, "{}: the popup has its title bar", p.label);
        click_panel_button(&h, p.label);
        h.run_steps(3);
        assert!(h.query_by_label(item_labels(p.items)[0]).is_none(), "{}: the button closes its popup", p.label);
    }
}

#[test]
fn a_split_remembers_its_entry_in_a_collapsed_panel_and_in_the_strip() {
    let mut h = ribbon(1280.0, 1024.0);
    click_panel_button(&h, "Utilities");
    h.run_steps(3);
    h.get_by_label("Measure options").click();
    h.run_steps(3);
    h.get_by_label("Distance").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("dist"));
    assert_eq!(RibbonState::split(&h.ctx, "Home/Utilities/0", 0), 1, "remembered under the panel's stable key");
    h.state_mut().session.cancel();
    h.set_size(vec2(1906.0, 1026.0));
    h.run_steps(3);
    assert!(title_rect(&h, "Utilities").is_some(), "expanded now");
    h.get_by_role_and_label(Role::Button, "Measure").click();
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("dist"), "the expanded panel's face runs the same entry");
}

fn open_draw_slideout(h: &mut Harness<'_, CadApp>) {
    let title = title_rect(h, "Draw").expect("Draw's title bar");
    h.hover_at(title.center());
    h.run_steps(1);
    h.query_all_by_label("Draw").find(|n| in_title_band(h, n.rect())).unwrap().click();
    h.run_steps(3);
}

fn slideout_is_open(h: &Harness<'_, CadApp>) -> bool {
    h.query_by_label("Spline Fit").is_some()
}

#[test]
fn the_draw_title_opens_its_slide_out_with_autocads_items_in_order() {
    let mut h = ribbon(1906.0, 1026.0);
    assert!(!slideout_is_open(&h));
    open_draw_slideout(&mut h);
    let draw = HOME.panels.iter().find(|p| p.label == "Draw").unwrap();
    let labels = item_labels(draw.slideout);
    assert_eq!(labels.first(), Some(&"Spline Fit"));
    assert_eq!(labels.last(), Some(&"Revision Cloud"));
    let strip_bottom = ribbon_top(&h) + layout::TAB_ROW_H + layout::STRIP_H;
    // "Measure" is also Utilities' split: take the one in the slide-out, below the strip.
    let below = |l: &str| h.query_all_by_label(l).map(|n| n.rect()).find(|r| r.top() >= strip_bottom - 1.0);
    let mut rects: Vec<(&str, egui::Rect)> = labels.iter().map(|l| (*l, below(l).unwrap_or_else(|| panic!("{l} in the slide-out")))).collect();
    assert!(rects.iter().all(|(_, r)| r.top() >= strip_bottom - 1.0), "the slide-out drops below the strip");
    rects.sort_by(|a, b| (a.1.top(), a.1.left()).partial_cmp(&(b.1.top(), b.1.left())).unwrap());
    assert_eq!(rects.iter().map(|(l, _)| *l).collect::<Vec<_>>(), labels, "row by row, left to right");
    let title = title_rect(&h, "Draw").unwrap();
    let left = rects.iter().map(|(_, r)| r.left()).fold(f32::MAX, f32::min);
    assert!(left >= title.left() && left < title.left() + 10.0, "under the Draw panel");
    // The title toggles it closed again.
    h.query_all_by_label("Draw").find(|n| in_title_band(&h, n.rect())).unwrap().click();
    h.run_steps(3);
    assert!(!slideout_is_open(&h));
}

#[test]
fn an_unpinned_slide_out_closes_after_the_pointer_leaves_and_a_pinned_one_stays() {
    let mut h = ribbon(1906.0, 1026.0);
    open_draw_slideout(&mut h);
    let away = pos2(1500.0, 800.0);
    h.hover_at(away);
    h.run_steps(1);
    assert!(slideout_is_open(&h), "not at once");
    h.run_steps(3);
    assert!(!slideout_is_open(&h), "closed half a second after the pointer left");

    open_draw_slideout(&mut h);
    h.get_by_label("Pin").click();
    h.run_steps(2);
    assert_eq!(h.get_by_label("Pin").accesskit_node().toggled(), Some(Toggled::True), "the pin shows pressed");
    h.hover_at(away);
    h.run_steps(8);
    assert!(slideout_is_open(&h), "a pinned slide-out stays open");

    h.get_by_label("Pin").click();
    h.run_steps(2);
    h.hover_at(away);
    h.run_steps(4);
    assert!(!slideout_is_open(&h), "unpinned again, it closes");

    open_draw_slideout(&mut h);
    h.key_press(egui::Key::Escape);
    h.run_steps(2);
    assert!(!slideout_is_open(&h), "Escape closes it");
}

#[test]
fn choosing_from_a_split_menu_inside_the_slide_out_keeps_the_slide_out_open() {
    let mut h = ribbon(1906.0, 1026.0);
    open_draw_slideout(&mut h);
    h.get_by_label("Revision Cloud options").click();
    h.run_steps(2);
    let slide_bottom = h.get_by_label("Pin").rect().bottom();
    let polygonal = h.get_by_label("Polygonal").rect();
    h.hover_at(polygonal.center());
    h.run_steps(6);
    assert!(slideout_is_open(&h), "the menu counts as inside the slide-out");
    assert!(h.query_by_label("Polygonal").is_some(), "the menu is still open");
    let reaches_past = h.get_by_label("Freehand").rect().bottom() > slide_bottom;
    h.get_by_label("Polygonal").click();
    h.run_steps(2);
    assert_eq!(running(&h).as_deref(), Some("revcloud.polygonal"));
    assert_eq!(RibbonState::split(&h.ctx, "Home/Draw/slideout/12", 0), 1);
    assert!(slideout_is_open(&h), "still open right after the choice");
    assert!(reaches_past, "the menu reaches below the slide-out (what this test is about)");
}

#[test]
fn the_minimize_button_cycles_the_four_states() {
    let mut h = ribbon(1906.0, 1026.0);
    assert_eq!(h.state().ui.ribbon_min, RibbonMin::Full);
    for (button, state) in [
        ("Minimize to Panel Buttons", RibbonMin::PanelButtons),
        ("Minimize to Panel Titles", RibbonMin::PanelTitles),
        ("Minimize to Tabs", RibbonMin::Tabs),
        ("Show Full Ribbon", RibbonMin::Full),
    ] {
        h.get_by_label(button).click();
        h.run_steps(3);
        assert_eq!(h.state().ui.ribbon_min, state);
        let expanded = h.query_by_label("Line").is_some();
        assert_eq!(expanded, state == RibbonMin::Full, "{state:?}: panels expanded only in the full ribbon");
        let panels = h.query_all_by_role_and_label(Role::Button, "Modify").count() + usize::from(title_rect(&h, "Modify").is_some());
        assert_eq!(panels > 0, state != RibbonMin::Tabs, "{state:?}: panel strip hidden only when minimized to tabs");
    }
    // The ▾ menu picks a state directly.
    h.get_by_label("Minimize Options").click();
    h.run_steps(2);
    h.get_by_label("Minimize to Tabs").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.ribbon_min, RibbonMin::Tabs);
    // Minimized to tabs, clicking the tab drops its panels down.
    h.get_by_role_and_label(Role::Tab, "Home").click();
    h.run_steps(3);
    assert!(h.query_by_label("Line").is_some(), "the tab shows its panels");
}

#[test]
fn panel_buttons_open_their_panel_as_a_popup() {
    let mut h = ribbon(1906.0, 1026.0);
    h.state_mut().ui.ribbon_min = RibbonMin::PanelButtons;
    h.run_steps(3);
    h.get_by_role_and_label(Role::Button, "Draw").click();
    h.run_steps(3);
    assert!(h.query_by_label("Line").is_some() && h.query_by_label("Circle").is_some());
}

/// `CRAFT_UI_VISUAL_DIR=/dir cargo test -p cadcraft-ui-egui --test ribbon_home ribbon_visuals`
/// renders the ribbon at AutoCAD's two observed window widths for a visual check.
#[test]
fn ribbon_visuals() {
    let Some(directory) = std::env::var_os("CRAFT_UI_VISUAL_DIR").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&directory).unwrap();
    for width in [1906.0, 1280.0] {
        let app = CadApp::new(Session::default(), Services::default());
        let mut h = Harness::builder().with_size(vec2(width, 300.0)).wgpu().build_ui_state(|ui, app: &mut CadApp| ribbon::show(app, ui), app);
        h.run_steps(3);
        h.render().unwrap().save(directory.join(format!("ribbon-{width}.png"))).unwrap();
        if width > 1500.0 {
            open_draw_slideout(&mut h);
        } else {
            click_panel_button(&h, "Properties");
            h.run_steps(3);
        }
        h.render().unwrap().save(directory.join(format!("ribbon-{width}-open.png"))).unwrap();
    }
    let app = CadApp::new(Session::default(), Services::default());
    let mut h = Harness::builder().with_size(vec2(1280.0, 300.0)).wgpu().build_ui_state(|ui, app: &mut CadApp| ribbon::show(app, ui), app);
    for state in [RibbonMin::PanelButtons, RibbonMin::PanelTitles, RibbonMin::Tabs] {
        h.key_press(egui::Key::Escape);
        h.state_mut().ui.ribbon_min = state;
        h.run_steps(3);
        if state == RibbonMin::Tabs {
            h.get_by_role_and_label(Role::Tab, "Home").click();
        } else {
            h.get_by_role_and_label(Role::Button, "Modify").click();
        }
        h.run_steps(3);
        h.render().unwrap().save(directory.join(format!("ribbon-{state:?}.png"))).unwrap();
    }
}
