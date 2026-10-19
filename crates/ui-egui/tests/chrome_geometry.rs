//! The real app's painted geometry at native and compact window sizes: AutoCAD's stack of title
//! row, ribbon tab row, panel strip and document tabs, the Home panels that collapse at AutoCAD's
//! window widths (in points, whatever the display scale), and the status bar.

use asenacad_engine::Session;
use asenacad_geom::Vec2;
use asenacad_ui_egui::chrome::{FILE_TABS_H, statusbar, titlerow};
use asenacad_ui_egui::ribbon::{self, home::HOME, layout};
use asenacad_ui_egui::{CadApp, Services};
use egui::accesskit::Role;
use egui::{Rect, Shape, vec2};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

/// The test harness keeps an 8 point margin around the app.
const MARGIN: f32 = 8.0;

/// The interface font a test runs with. The layout must hold in any: the ribbon font is sized
/// to AutoCAD's tab widths whatever the font ([`asenacad_ui_egui::theme::ribbon_font`]).
#[derive(Clone, Copy, Debug)]
enum UiFont {
    /// The system's (DejaVu Sans, Segoe UI, ...), when this machine has one where AsenaCAD looks.
    System,
    /// egui's own, what a system without one shows (the FreeBSD CI VM).
    Egui,
}

const FONTS: [UiFont; 2] = [UiFont::System, UiFont::Egui];

fn harness(font: UiFont, width: f32, height: f32, scale: f32, message: bool) -> Harness<'static, CadApp> {
    harness_with(font, width, height, scale, message, |_| {})
}

fn harness_with(font: UiFont, width: f32, height: f32, scale: f32, message: bool, setup: impl FnOnce(&mut CadApp)) -> Harness<'static, CadApp> {
    let mut app = CadApp::new(Session::default(), Services::default());
    app.run("new", serde_json::json!({})).unwrap();
    app.ui.start_tab = false;
    app.ui.in_window_menu = false;
    app.integrated_titlebar = true;
    setup(&mut app);
    if message {
        app.set_status("Drawing saved successfully with a long status message");
    }
    let mut h = Harness::builder().with_size(vec2(width, height)).with_pixels_per_point(scale).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.canvas.cursor = Some(Vec2::new(1234.5, -9876.25));
            app.ui(ui);
        },
        app,
    );
    if let UiFont::Egui = font {
        // The app installs its fonts once per interface language, in its first pass.
        h.run();
        h.ctx.set_fonts(egui::FontDefinitions::default());
    }
    h.run_steps(5);
    h
}

fn visible_text(shape: &Shape, clip: Rect, result: &mut Vec<(String, Rect)>) {
    match shape {
        Shape::Text(text) => {
            let rect = text.visual_bounding_rect().intersect(clip);
            if rect.is_positive() {
                result.push((text.galley.text().to_string(), rect));
            }
        }
        Shape::Vec(shapes) => {
            for shape in shapes {
                visible_text(shape, clip, result);
            }
        }
        _ => {}
    }
}

#[test]
fn status_labels_never_overlap_at_standard_and_compact_sizes() {
    let mut overlaps = Vec::new();
    let sizes = FONTS.into_iter().flat_map(|font| [(1200.0, 760.0), (900.0, 560.0)].map(|size| (font, size)));
    for (font, (width, height)) in sizes {
        for scale in [1.0, 2.0] {
            for message in [false, true] {
                let mut h = harness(font, width, height, scale, message);
                if let Ok(directory) = std::env::var("CRAFT_UI_VISUAL_DIR") {
                    std::fs::create_dir_all(&directory).unwrap();
                    h.render().unwrap().save(format!("{directory}/cad-{font:?}-{width}-{scale}-{message}.png")).unwrap();
                }
                let mut texts = Vec::new();
                for shape in &h.output().shapes {
                    visible_text(&shape.shape, shape.clip_rect, &mut texts);
                }
                // The status bar's texts: those inside its rectangle (whatever the interface
                // font's ascent puts a text's centre at; "Model", with no descender, sits higher
                // than "Layout1").
                let status_top = height - MARGIN - statusbar::HEIGHT;
                texts.retain(|(_, rect)| rect.top() >= status_top - 0.5 && rect.bottom() <= height - MARGIN + 0.5);
                // The current tab is always drawn; the others move to the layout list when the bar is narrow.
                let at = format!("{font:?} font, {width}px @{scale}");
                assert!(texts.iter().any(|(text, _)| text == "Model"), "{at}: the Model tab's name in {texts:?}");
                if width >= 1200.0 {
                    assert!(texts.iter().any(|(text, _)| text == "Layout2"), "{at}: Layout2's name in {texts:?}");
                }
                for (i, (first, first_rect)) in texts.iter().enumerate() {
                    for (second, second_rect) in texts.iter().skip(i + 1) {
                        if first_rect.intersects(*second_rect) {
                            overlaps.push(format!("{at}: {first:?} {first_rect:?} overlaps {second:?} {second_rect:?}"));
                        }
                    }
                }
            }
        }
    }
    assert!(overlaps.is_empty(), "{}", overlaps.join("\n"));
}

/// AsenaCAD's Tool Bar, which is off by default (AutoCAD has none).
fn with_toolbar(app: &mut CadApp) {
    app.ui.show_toolbar = true;
}

#[test]
fn compact_toolbar_overflow_keeps_help_reachable() {
    let mut h = harness_with(UiFont::System, 900.0, 560.0, 1.0, false, with_toolbar);
    h.get_by_label("More toolbar commands").click();
    h.run_steps(3);
    // The overflow menu's Help, not the title row's Help button above it.
    h.query_all_by_label("Help").max_by(|a, b| a.rect().top().total_cmp(&b.rect().top())).unwrap().click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("about"));
}

#[test]
fn toolbar_plot_preview_opens_dialog() {
    let mut h = harness_with(UiFont::System, 1200.0, 760.0, 1.0, false, with_toolbar);
    h.get_by_label("Plot Preview").click();
    h.run_steps(3);
    assert_eq!(h.state().ui.dialog.as_deref(), Some("preview"));
}

/// The app in a window `width` points wide (the harness's margin added), at display `scale`.
fn window(font: UiFont, width: f32, height: f32, scale: f32) -> Harness<'static, CadApp> {
    harness(font, width + 2.0 * MARGIN, height + 2.0 * MARGIN, scale, false)
}

/// AutoCAD's bands, from the window's top: title row, tab row, panel strip, document tab bar.
fn bands() -> [(&'static str, f32, f32); 4] {
    let tabs_top = titlerow::HEIGHT;
    let strip_top = tabs_top + layout::TAB_ROW_H;
    let doc_top = strip_top + layout::STRIP_H;
    [
        ("title row", 0.0, tabs_top),
        ("tab row", tabs_top, strip_top),
        ("panel strip", strip_top, doc_top),
        ("document tabs", doc_top, doc_top + FILE_TABS_H),
    ]
}

fn within(r: Rect, (name, top, bottom): (&str, f32, f32)) -> Result<(), String> {
    let (top, bottom) = (top + MARGIN, bottom + MARGIN);
    if r.top() >= top - 0.5 && r.bottom() <= bottom + 0.5 { Ok(()) } else { Err(format!("{r:?} is outside the {name} ({top}..{bottom})")) }
}

#[test]
fn the_title_row_tabs_ribbon_and_document_tabs_stack_like_autocads_without_overlapping() {
    // AutoCAD at 1906 px: title row 0..32, tab row 32..56, panels 56..156, document tabs
    // 156..182 and the strip under them to 186, where the drawing starts.
    let [title, tabs, strip, docs] = bands();
    assert_eq!((tabs.1, strip.1, docs.1, docs.2), (32.0, 56.0, 156.0, 186.0));
    for (font, width) in FONTS.into_iter().flat_map(|font| [1906.0, 1280.0].map(|width| (font, width))) {
        for scale in [1.0, 1.34] {
            let h = window(font, width, 1000.0, scale);
            let at = format!("{font:?} font, {width} pt @{scale}");
            let check = |r: Rect, band| within(r, band).unwrap_or_else(|e| panic!("{at}: {e}"));
            check(h.get_by_label("Application Button").rect(), title);
            check(h.get_by_role_and_label(Role::Button, "Save").rect(), title);
            for tab in ribbon::model::tabs() {
                check(h.get_by_role_and_label(Role::Tab, tab.label).rect(), tabs);
            }
            check(h.get_by_label("Line").rect(), strip);
            check(h.query_all_by_label("Draw").map(|n| n.rect()).find(|r| r.top() > strip.1 + MARGIN).expect("Draw's title"), strip);
            check(h.get_by_label("File Tabs").rect(), docs);
            check(h.get_by_role_and_label(Role::Tab, "Start").rect(), docs);
            check(h.get_by_label("New Drawing").rect(), docs);
            let canvas = h.state().canvas.rect.expect("the drawing area is laid out");
            assert!(canvas.top() >= docs.2 + MARGIN - 0.5, "{at}: the drawing starts below the document tabs: {canvas:?}");
            // No two texts of the shell above the drawing touch.
            let mut texts = Vec::new();
            for shape in &h.output().shapes {
                visible_text(&shape.shape, shape.clip_rect, &mut texts);
            }
            texts.retain(|(text, r)| !text.is_empty() && r.bottom() <= canvas.top());
            let mut overlaps = Vec::new();
            for (i, (a, ra)) in texts.iter().enumerate() {
                for (b, rb) in texts.iter().skip(i + 1) {
                    if ra.shrink(0.5).intersects(rb.shrink(0.5)) {
                        overlaps.push(format!("{a:?} {ra:?} / {b:?} {rb:?}"));
                    }
                }
            }
            assert!(overlaps.is_empty(), "{at}: overlapping texts:\n{}", overlaps.join("\n"));
            // Nothing of the old Mac shell: no Tool Bar, no menu bar, no palettes.
            assert!(h.query_by_label("More toolbar commands").is_none() && h.query_by_label("Measure Distance").is_none());
            assert!(h.query_by_role_and_label(Role::Tab, "Tool Sets").is_none() && h.query_by_role_and_label(Role::Tab, "Properties").is_none());
        }
    }
}

/// Whether `panel` of Home is a collapsed panel button (a panel-tall button with its name).
fn collapsed(h: &Harness<'_, CadApp>, panel: &str) -> bool {
    h.query_all_by_role_and_label(Role::Button, panel).any(|n| (n.rect().height() - layout::PANEL_H).abs() < 0.5)
}

/// The rectangles of Home's panel titles and collapsed buttons, left to right.
fn panel_rects(h: &Harness<'_, CadApp>) -> Vec<Rect> {
    let title_top = MARGIN + titlerow::HEIGHT + layout::TAB_ROW_H + layout::PANEL_TOP + layout::PANEL_H - layout::TITLE_H;
    HOME.panels
        .iter()
        .map(|p| {
            let title = h.query_all_by_label(p.label).map(|n| n.rect()).find(|r| (r.top() - title_top).abs() < 1.0);
            let button = h.query_all_by_role_and_label(Role::Button, p.label).map(|n| n.rect()).find(|r| (r.height() - layout::PANEL_H).abs() < 0.5);
            title.or(button).unwrap_or_else(|| panic!("{} is shown", p.label))
        })
        .collect()
}

#[test]
fn home_collapses_at_autocads_window_widths_whatever_the_display_scale() {
    let collapsed_at_1280 = ["Properties", "Groups", "Utilities", "Clipboard", "View"];
    for (font, scale) in FONTS.into_iter().flat_map(|font| [1.0, 1.34].map(|scale| (font, scale))) {
        let at = format!("{font:?} font @{scale}");
        // 1280: Draw, Modify, Annotation, Layers and Block stay (small items without their
        // labels: Modify shows icons only), the five panels on the right collapse.
        let h = window(font, 1280.0, 1000.0, scale);
        let shown: Vec<_> = HOME.panels.iter().map(|p| p.label).filter(|p| collapsed(&h, p)).collect();
        assert_eq!(shown, collapsed_at_1280, "{at}: AutoCAD's collapsed panels at 1280 px");
        let mv = h.get_by_role_and_label(Role::Button, "Move").rect();
        assert!(mv.width() < 30.0, "{at}: Modify shows icons only at 1280 px: {mv:?}");
        assert!(h.get_by_role_and_label(Role::Button, "Line").rect().height() > 60.0, "{at}: Line stays a large button");
        let rects_1280 = panel_rects(&h);
        // 1906: all ten panels with their labels.
        let h = window(font, 1906.0, 1026.0, scale);
        assert!(HOME.panels.iter().all(|p| !collapsed(&h, p.label)), "{at}: every Home panel is expanded at 1906 px");
        assert!(h.get_by_role_and_label(Role::Button, "Move").rect().width() > 40.0, "{at}: Move shows its label at 1906 px");
        let rects_1906 = panel_rects(&h);
        if scale == 1.0 {
            continue;
        }
        // In points, the layout is the same at any display scale, in any interface font: less
        // than a pixel apart (text widths never depend on the scale; see `large_label_width`).
        let reference = |w: f32, rects: &[Rect]| {
            let base = panel_rects(&window(font, w, 1000.0, 1.0));
            for (i, (a, b)) in base.iter().zip(rects).enumerate() {
                assert!(
                    (a.left() - b.left()).abs() <= 0.5 && (a.width() - b.width()).abs() <= 0.5,
                    "{font:?} font, {}: {a:?} at 1.0 vs {b:?} at {scale}",
                    HOME.panels[i].label
                );
            }
        };
        reference(1280.0, &rects_1280);
        reference(1906.0, &rects_1906);
    }
}

/// Every text shape whose whole text is `text`, with its galley.
fn galleys_of(shape: &Shape, text: &str, out: &mut Vec<std::sync::Arc<egui::Galley>>) {
    match shape {
        Shape::Text(t) if t.galley.text() == text => out.push(t.galley.clone()),
        Shape::Vec(shapes) => shapes.iter().for_each(|s| galleys_of(s, text, out)),
        _ => {}
    }
}

#[test]
fn collapsed_panel_labels_keep_whole_words_and_fit_their_buttons() {
    // AutoCAD never breaks a word of a collapsed panel's name: a button whose name is too wide
    // for the standard width gets wider ("Annotation" in a ~900 pt window).
    for (font, scale) in FONTS.into_iter().flat_map(|font| [1.0, 1.34].map(|scale| (font, scale))) {
        for width in [700.0, 900.0] {
            let h = window(font, width, 700.0, scale);
            let at = format!("{font:?} font, {width} pt @{scale}");
            let collapsed_panels: Vec<_> = HOME.panels.iter().map(|p| p.label).filter(|p| collapsed(&h, p)).collect();
            assert!(collapsed_panels.contains(&"Annotation"), "{at}: Annotation collapses: {collapsed_panels:?}");
            for label in collapsed_panels {
                let button = h
                    .query_all_by_role_and_label(Role::Button, label)
                    .map(|n| n.rect())
                    .find(|r| (r.height() - layout::PANEL_H).abs() < 0.5)
                    .unwrap_or_else(|| panic!("{at}: {label}'s button"));
                assert!(button.width() >= layout::PANEL_BUTTON_W - 0.5, "{at}: {label} is at least the standard width: {button:?}");
                let mut galleys = Vec::new();
                for shape in &h.output().shapes {
                    galleys_of(&shape.shape, label, &mut galleys);
                }
                assert!(!galleys.is_empty(), "{at}: {label}'s name is painted");
                for g in galleys {
                    assert_eq!(g.rows.len(), 1, "{at}: {label} is one whole word on one line");
                    assert!(g.size().x <= button.width(), "{at}: {label} ({}) fits its button {button:?}", g.size().x);
                }
            }
        }
    }
}
