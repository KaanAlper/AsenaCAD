//! AutoCAD's Application menu under the Application button: its size and place, the first-level
//! entries in AutoCAD's order with their separators, each sub-pane's entries and which are
//! disabled, the default commands, Recent and Open Documents, the command search, Escape,
//! Options and Exit.

use std::cell::RefCell;
use std::rc::Rc;

use asenacad_engine::Session;
use asenacad_ui_egui::chrome::{appmenu, titlerow};
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::{Rect, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT, Queryable};
use serde_json::json;

fn harness_with(app: CadApp) -> Harness<'static, CadApp> {
    let mut h = Harness::builder().with_size(vec2(1906.0, 700.0)).build_ui_state(
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

fn button_rect(h: &Harness<'_, CadApp>) -> Rect {
    h.get_by_role_and_label(Role::Button, "Application Button").rect()
}

fn open_menu(h: &mut Harness<'_, CadApp>) {
    h.get_by_label("Application Button").click();
    h.run_steps(3);
}

fn menu_rect(h: &Harness<'_, CadApp>) -> Option<Rect> {
    h.query_by_label("Application Menu").map(|n| n.rect())
}

/// The one button labelled `label` inside the menu (the Quick Access Toolbar shares some labels).
fn item<'a>(h: &'a Harness<'_, CadApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let menu = menu_rect(h).expect("the menu is open");
    let mut nodes: Vec<_> =
        h.query_all_by_label(label).filter(|n| n.accesskit_node().role() == Role::Button && menu.contains_rect(n.rect())).collect();
    assert_eq!(nodes.len(), 1, "one menu item {label}");
    nodes.remove(0)
}

fn has_item(h: &Harness<'_, CadApp>, label: &str) -> bool {
    let Some(menu) = menu_rect(h) else { return false };
    h.query_all_by_label(label).any(|n| n.accesskit_node().role() == Role::Button && menu.contains_rect(n.rect()))
}

fn click(h: &mut Harness<'_, CadApp>, label: &str) {
    item(h, label).click();
    h.run_steps(3);
}

fn open_sub(h: &mut Harness<'_, CadApp>, top: &str) {
    click(h, &format!("{top} submenu"));
}

/// Click the sub-pane's ▾ until its list ends (it was there: the list overflowed).
fn scroll_to_end(h: &mut Harness<'_, CadApp>) {
    assert!(has_item(h, "Scroll down"), "the list overflows");
    for _ in 0..40 {
        if !has_item(h, "Scroll down") {
            return;
        }
        click(h, "Scroll down");
    }
    panic!("the list never ends");
}

fn selected(h: &Harness<'_, CadApp>, label: &str) -> bool {
    item(h, label).accesskit_node().toggled() == Some(egui::accesskit::Toggled::True)
}

const TOPS: [&str; 10] = ["New", "Open", "Save", "Save As", "Import", "Export", "Publish", "Print", "Drawing Utilities", "Close"];

#[test]
fn the_menu_opens_under_the_button_at_autocads_size() {
    let mut h = harness();
    assert!(menu_rect(&h).is_none());
    open_menu(&mut h);
    let menu = menu_rect(&h).expect("open");
    let button = button_rect(&h);
    assert_eq!((menu.left(), menu.top()), (button.left(), button.bottom()), "directly under the button");
    assert_eq!(menu.size(), vec2(424.0, 568.0));
    assert_eq!(menu.size(), appmenu::SIZE);
    // A second click closes it.
    h.get_by_label("Application Button").click();
    h.run_steps(3);
    assert!(menu_rect(&h).is_none());
}

#[test]
fn first_level_entries_are_in_autocads_order_with_two_separators() {
    let mut h = harness();
    open_menu(&mut h);
    let menu = menu_rect(&h).unwrap();
    let tops: Vec<f32> = TOPS.iter().map(|l| item(&h, l).rect().top() - menu.top()).collect();
    let gaps: Vec<f32> = tops.windows(2).map(|w| (w[1] - w[0]).round()).collect();
    // 44 px rows; a 4 px separator after Export and after Drawing Utilities.
    assert_eq!(gaps, [44.0, 44.0, 44.0, 44.0, 44.0, 48.0, 44.0, 44.0, 48.0], "{tops:?}");
    assert!((tops[0] - 76.0).abs() < 0.5, "the list starts 75 px down: {tops:?}");
    // Every entry but Save has a 20 px ▸ part right of its main part.
    for top in TOPS {
        let arrow = format!("{top} submenu");
        assert_eq!(has_item(&h, &arrow), top != "Save", "{arrow}");
        if top != "Save" {
            let (a, m) = (item(&h, &arrow).rect(), item(&h, top).rect());
            assert_eq!(a.width(), appmenu::ARROW_W);
            assert!((a.left() - m.right()).abs() < 0.5 && a.top() == m.top());
        }
    }
}

/// Resting on an entry's name shows its sub-pane, as resting on its ▸ does; an entry without
/// one (Save) brings the documents back.
#[test]
fn hovering_an_entrys_name_shows_its_sub_pane() {
    let mut h = harness();
    open_menu(&mut h);
    assert!(!has_item(&h, "Drawing Template"));
    item(&h, "Save As").hover();
    h.run_steps(3);
    assert!(has_item(&h, "Drawing Template"), "Save As's sub-pane is shown");
    item(&h, "Import").hover();
    h.run_steps(3);
    assert!(has_item(&h, "Other Formats") && !has_item(&h, "Drawing Template"), "Import's sub-pane replaces it");
    item(&h, "Save").hover();
    h.run_steps(3);
    assert!(!has_item(&h, "Other Formats") && has_item(&h, "By Ordered List"), "Save brings the documents back");
}

/// Each sub-pane's entries in order, with whether each is enabled (on an empty model-space
/// drawing).
#[test]
fn each_sub_pane_lists_autocads_entries_and_disables_what_has_no_command() {
    let panes: &[(&str, &[(&str, bool)])] = &[
        ("New", &[("Drawing", true), ("Sheet Set", false)]),
        ("Open", &[("Drawing", true), ("Drawing from AutoCAD Web & Mobile", false), ("Sheet Set", false), ("DGN", false), ("Sample Files", true)]),
        (
            "Save As",
            &[
                ("Drawing", true),
                ("Drawing to AutoCAD Web & Mobile", false),
                ("Drawing Template", true),
                ("Drawing Standards", false),
                ("Other Formats", true),
                ("Save Layout as a Drawing", false),
                ("DWG Convert", false),
            ],
        ),
        ("Import", &[("PDF", false), ("DGN", false), ("Other Formats", true)]),
        ("Export", &[("DWF", false), ("DWFx", false), ("3D DWF", false), ("PDF", true), ("DGN", false), ("Other Formats", true)]),
        (
            "Publish",
            &[
                ("Send to 3D Print Service", false),
                ("Archive", false),
                ("eTransmit", false),
                ("Email", false),
                ("Share View", false),
                ("Share Drawing", false),
            ],
        ),
        (
            "Print",
            &[
                ("Plot", true),
                ("Batch Plot", true),
                ("Plot Preview", true),
                ("View Plot and Publish Details", false),
                ("Page Setup", true),
                ("3D Print", false),
                ("Manage Plotters", true),
            ],
        ),
        (
            "Drawing Utilities",
            &[
                ("Drawing Properties", false),
                ("DWG Compare", true),
                ("Units", true),
                ("Audit", true),
                ("Status", true),
                ("Purge", true),
                ("Recover", true),
            ],
        ),
        ("Close", &[("Current Drawing", true), ("All Drawings", true)]),
    ];
    let mut h = harness();
    open_menu(&mut h);
    for (top, entries) in panes {
        open_sub(&mut h, top);
        let ys: Vec<f32> = entries.iter().map(|(l, _)| item(&h, l).rect().top()).collect();
        assert!(ys.windows(2).all(|w| w[0] < w[1]), "{top}: AutoCAD's order {ys:?}");
        for (label, enabled) in *entries {
            assert_eq!(!item(&h, label).accesskit_node().is_disabled(), *enabled, "{top} ▸ {label}");
        }
    }
    // Print's list overflows: a ▾ scrolls to the last entries.
    open_sub(&mut h, "Print");
    assert!(!has_item(&h, "Scroll up"));
    scroll_to_end(&mut h);
    assert!(has_item(&h, "Scroll up"));
    assert!(!item(&h, "Manage Plot Styles").accesskit_node().is_disabled());
    assert!(item(&h, "Edit Plot Style Tables").accesskit_node().is_disabled());
    // Nested lists: Recover ▸ and Sample Files ▸.
    open_sub(&mut h, "Drawing Utilities");
    scroll_to_end(&mut h);
    assert!(!item(&h, "Open the Drawing Recovery Manager").accesskit_node().is_disabled());
    click(&mut h, "Recover submenu");
    assert!(!item(&h, "Recover").accesskit_node().is_disabled());
    assert!(item(&h, "Recover with Xrefs").accesskit_node().is_disabled());
    click(&mut h, "Back");
    assert!(has_item(&h, "Units"));
    open_sub(&mut h, "Open");
    click(&mut h, "Sample Files");
    click(&mut h, "Bracket");
    assert_eq!(h.state().session.state().unwrap().title, "Bracket", "the sample drawing opens");
}

#[test]
fn main_parts_run_their_default_commands() {
    let mut h = harness();
    let docs = h.state().session.docs.len();
    open_menu(&mut h);
    click(&mut h, "New");
    assert_eq!(h.state().session.docs.len(), docs + 1, "New starts a drawing");
    assert!(menu_rect(&h).is_none(), "and the menu closes");
    open_menu(&mut h);
    click(&mut h, "Print");
    assert_eq!(h.state().ui.dialog.as_deref(), Some("plot"));
    h.state_mut().ui.dialog = None;
    open_menu(&mut h);
    click(&mut h, "Close");
    assert_eq!(h.state().session.docs.len(), docs, "Close closes the new, unchanged drawing");
    // Publish and Drawing Utilities have no default: their main part shows the sub-pane.
    open_menu(&mut h);
    click(&mut h, "Drawing Utilities");
    assert!(has_item(&h, "Audit"));
    click(&mut h, "Units");
    assert_eq!(h.state().ui.dialog.as_deref(), Some("units"));
}

#[test]
fn save_save_as_and_open_ask_for_a_file() {
    let asked = Rc::new(RefCell::new(Vec::<String>::new()));
    let (save_log, open_log) = (Rc::clone(&asked), Rc::clone(&asked));
    let services = Services {
        pick_open: Some(Box::new(move || {
            open_log.borrow_mut().push("open".into());
            None
        })),
        pick_save: Some(Box::new(move |name: &str| {
            save_log.borrow_mut().push(name.to_string());
            None
        })),
    };
    let mut h = harness_with(CadApp::new(Session::default(), services));
    open_menu(&mut h);
    click(&mut h, "Save");
    open_menu(&mut h);
    click(&mut h, "Open");
    open_menu(&mut h);
    open_sub(&mut h, "Save As");
    click(&mut h, "Drawing Template");
    let asked = asked.borrow();
    assert_eq!(asked.len(), 3, "{asked:?}");
    assert!(asked[0].starts_with("Drawing1."), "Save on an untitled drawing asks where: {asked:?}");
    assert_eq!(asked[1], "open");
    assert!(asked[2].ends_with(".dwt"), "a template is suggested as .dwt: {asked:?}");
}

#[test]
fn recent_and_open_documents_switch_and_open_or_switch_to_a_drawing() {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", json!({})).unwrap();
    app.ui.recent_files = vec!["/nowhere/plan.dxf".into(), "/nowhere/section.dwg".into()];
    let titles: Vec<String> = app.session.docs.iter().map(|d| d.title.clone()).collect();
    assert_eq!(app.session.active, 1);
    let mut h = harness_with(app);
    open_menu(&mut h);
    assert!(selected(&h, "Recent Documents"));
    let (plan, section) = (item(&h, "plan.dxf").rect(), item(&h, "section.dwg").rect());
    assert!(plan.top() < section.top(), "most recent first");
    assert!(has_item(&h, "By Ordered List"));
    // By Type puts DWG before DXF.
    click(&mut h, "By Ordered List");
    click(&mut h, "By Type");
    assert!(item(&h, "section.dwg").rect().top() < item(&h, "plan.dxf").rect().top());
    assert!(has_item(&h, "By Type"));
    // The view button switches to larger icons and back.
    let row = item(&h, "plan.dxf").rect().height();
    click(&mut h, "Document View");
    click(&mut h, "Icons");
    assert!(item(&h, "plan.dxf").rect().height() > row);

    click(&mut h, "Open Documents");
    assert!(selected(&h, "Open Documents"));
    assert!(!has_item(&h, "plan.dxf") && !has_item(&h, "By Type"), "no ordering button for open drawings");
    assert!(has_item(&h, "Document View"));
    click(&mut h, &titles[0]);
    assert_eq!(h.state().session.active, 0, "clicking an open drawing switches to it");
    assert!(menu_rect(&h).is_none());
    // The choice stays for the next time.
    open_menu(&mut h);
    assert!(selected(&h, "Open Documents"));
}

#[test]
fn the_search_finds_a_command_and_starts_it() {
    let mut h = harness();
    open_menu(&mut h);
    assert!(item(&h, "Search").accesskit_node().is_disabled(), "the search button waits for text");
    h.get_by_label("Search Commands").type_text("circ");
    h.run_steps(3);
    assert!(!item(&h, "Search").accesskit_node().is_disabled());
    assert!(!has_item(&h, "plan.dxf"));
    click(&mut h, "Circle");
    assert_eq!(running(&h).as_deref(), Some("circle"));
    assert!(menu_rect(&h).is_none());
}

#[test]
fn enter_in_the_search_box_runs_the_best_match() {
    let mut h = harness();
    open_menu(&mut h);
    h.get_by_label("Search Commands").type_text("line");
    h.run_steps(3);
    h.key_press(egui::Key::Enter);
    h.run_steps(3);
    assert_eq!(running(&h).as_deref(), Some("line"));
}

#[test]
fn escape_and_a_click_outside_close_the_menu() {
    let mut h = harness();
    open_menu(&mut h);
    h.key_press(egui::Key::Escape);
    h.run_steps(3);
    assert!(menu_rect(&h).is_none());
    open_menu(&mut h);
    h.get_by_label("Customize Quick Access Toolbar").click();
    h.run_steps(3);
    assert!(menu_rect(&h).is_none(), "a click elsewhere closes it");
}

#[test]
fn the_keyboard_walks_the_menu() {
    let mut h = harness();
    open_menu(&mut h);
    // Down to Save As, Right into its sub-pane, Down to the template entry: none of it runs yet.
    for key in [egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::ArrowRight] {
        h.key_press(key);
        h.run_steps(2);
    }
    assert!(has_item(&h, "Drawing Template"), "Right opens Save As's sub-pane");
    h.key_press(egui::Key::ArrowLeft);
    h.run_steps(2);
    for key in [egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::ArrowDown, egui::Key::Enter] {
        h.key_press(key);
        h.run_steps(2);
    }
    // Save As → Import → Export → Publish → Print, and Enter runs Print's PLOT.
    assert_eq!(h.state().ui.dialog.as_deref(), Some("plot"));
}

#[test]
fn options_opens_the_options_dialog() {
    let mut h = harness();
    open_menu(&mut h);
    click(&mut h, "Options");
    assert_eq!(h.state().ui.dialog.as_deref(), Some("options"));
}

#[test]
fn exit_goes_through_the_close_path() {
    let mut h = harness();
    h.state_mut().run("line", json!({"points": [[0.0, 0.0], [10.0, 0.0]]})).unwrap();
    open_menu(&mut h);
    click(&mut h, "Exit AsenaCAD");
    assert!(h.state().close_prompt().is_some(), "unsaved changes ask first");
    assert!(!h.state().quit_requested);
    // A clean session quits straight away.
    let mut h = harness();
    open_menu(&mut h);
    click(&mut h, "Exit AsenaCAD");
    assert!(h.state().quit_requested);
}
