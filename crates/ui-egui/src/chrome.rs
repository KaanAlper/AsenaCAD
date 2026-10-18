//! Window chrome: the document tab bar, the status bar, the Start page and the (optional) Tool
//! Bar. The title row is `titlerow`, its Quick Access Toolbar `qat`.

use egui::{Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

pub mod annoscale;
pub mod layouts;
pub mod qat;
pub mod statusbar;
pub mod statusmenus;
pub mod titlerow;

/// The Tool Bar's height.
const TOOLBAR_H: f32 = 34.0;
const TOOLBAR: &[&[(Icon, &str, &str, Option<&str>)]] = &[
    &[
        (Icon::New, "new", "New Drawing", Some("Cmd+N")),
        (Icon::Open, "open", "Open", Some("Cmd+O")),
        (Icon::Save, "qsave", "Save", Some("Cmd+S")),
        (Icon::SaveAs, "saveas", "Save As", None),
    ],
    &[(Icon::Undo, "u", "Undo", Some("Cmd+Z")), (Icon::Redo, "redo", "Redo", Some("Cmd+Shift+Z"))],
    &[
        (Icon::Plot, "plot", "Print", None),
        (Icon::Publish, "publish", "Batch Publish", None),
        (Icon::PageSetup, "pagesetup", "Page Setup Manager", None),
        (Icon::Preview, "preview", "Plot Preview", None),
    ],
    &[
        (Icon::Import, "import", "Import", None),
        (Icon::Export, "export", "Export", None),
        (Icon::Attach, "attach", "Attach", None),
        (Icon::Share, "share", "Share", None),
    ],
    &[(Icon::ZoomWindow, "zoom.window", "Zoom Window", None), (Icon::Pan, "pan", "Pan", None), (Icon::Orbit, "3dorbit", "Orbit", None)],
    &[
        (Icon::ZoomExtents, "zoom.extents", "Zoom Extents", None),
        (Icon::Properties, "ui.toggle.palettes", "Properties", None),
        (Icon::Layers, "ui.dialog.layers", "Layer Properties Manager", None),
        (Icon::Blocks, "ui.dialog.blocks", "Blocks", None),
    ],
    &[(Icon::Measure, "dist", "Measure Distance", None), (Icon::List, "list", "List", None), (Icon::Area, "area", "Area", None)],
    &[(Icon::Help, "ui.dialog.about", "Help", None)],
];

/// The macOS title-bar band (#161). AppKit draws the traffic lights itself, centred in the
/// standard 28 pt band at the top of the window; moving them would take AppKit calls (`unsafe`,
/// which the workspace forbids). With the integrated title bar the title row starts its content
/// right of them.
const MAC_TITLE_BAR: f32 = 28.0;
/// The traffic lights as AppKit lays them out: button diameter, centre-to-centre pitch and the
/// first button's left edge (8–10 pt depending on the macOS version; the larger value keeps the
/// gap after them from shrinking).
const MAC_LIGHT: f32 = 12.0;
const MAC_LIGHT_PITCH: f32 = 20.0;
const MAC_LIGHTS_LEFT: f32 = 10.0;
/// Where title-row content may start to the right of the traffic lights: the end of the three
/// lights plus the same gap the lights keep from the top of the window.
const MAC_CONTENT_LEFT: f32 = MAC_LIGHTS_LEFT + 2.0 * MAC_LIGHT_PITCH + MAC_LIGHT + (MAC_TITLE_BAR - MAC_LIGHT) / 2.0;

/// Dragging `rect` moves the window and double-clicking it maximizes or restores it, like the
/// title bar the integrated macOS title row replaces.
fn window_drag(ui: &mut egui::Ui, rect: Rect) {
    let resp = ui.interact(rect, ui.id().with("titledrag"), Sense::CLICK | Sense::DRAG);
    if resp.drag_started() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if resp.double_clicked() {
        let max = ui.ctx().input(|i| i.viewport().maximized.unwrap_or(false));
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Maximized(!max));
    }
}

/// CADCraft's Tool Bar (Window ▸ Tool Bar), under the ribbon. AutoCAD has none, so it is off by
/// default; the title row (`titlerow`) holds the window title and the macOS traffic-light inset.
pub fn toolbar(app: &mut CadApp, ui: &mut egui::Ui) {
    if !app.ui.show_toolbar {
        return;
    }
    let t = Tokens::get();
    egui::Panel::top("cc_toolbar").exact_size(TOOLBAR_H).frame(egui::Frame::NONE.fill(t.chrome)).show(ui, |ui| {
        let row = ui.max_rect();
        let mut x = row.left() + 12.0;
        let size = 26.0;
        let mut clicked = None;
        let mac = ui.ctx().os().is_mac();
        let widths: Vec<_> = TOOLBAR.iter().map(|group| group.len() as f32 * (size + 6.0) - 6.0).collect();
        let available = (row.right() - 12.0 - x).max(0.0);
        let Some(fit) = craft_ui::layout::fit_toolbar(&widths, available, 24.0, 30.0) else {
            // Very small windows still expose the complete command row by scrolling.
            ui.scope_builder(egui::UiBuilder::new().max_rect(row.shrink2(vec2(4.0, 0.0))), |ui| {
                egui::ScrollArea::horizontal().id_salt("toolbar_small").show(ui, |ui| {
                    ui.horizontal(|ui| {
                        for group in TOOLBAR {
                            for (icon, cmd, tip, _) in *group {
                                if icons::button(ui, *icon, size, tip, false).clicked() {
                                    clicked = Some(*cmd);
                                }
                            }
                            ui.add_space(18.0);
                        }
                    });
                });
            });
            if let Some(command) = clicked {
                app.start(command);
            }
            return;
        };
        for (index, _) in &fit.shown {
            let Some(group) = TOOLBAR.get(*index) else { continue };
            for (icon, cmd, label, shortcut) in group.iter() {
                let br = Rect::from_min_size(pos2(x, row.center().y - size / 2.0), vec2(size, size));
                let resp = ui.interact(br, ui.id().with(("tb", *cmd)), Sense::click());
                if resp.hovered() {
                    ui.painter().rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.6));
                }
                icons::paint(ui.painter(), br.shrink(4.0), *icon, false);
                let tip = match shortcut {
                    Some(sc) => format!("{} ({})", crate::i18n::t(label), crate::menus::shortcut_label(sc, mac)),
                    None => crate::i18n::t(label).to_string(),
                };
                icons::describe_control(ui, &resp, &tip, None, false);
                if resp.on_hover_text(tip).clicked() {
                    clicked = Some(*cmd);
                }
                x += size + 6.0;
            }
            x += 18.0;
        }
        if !fit.overflow.is_empty() {
            let br = Rect::from_min_size(pos2(x, row.center().y - size / 2.0), vec2(fit.overflow_width, size));
            let response = ui.interact(br, ui.id().with("toolbar_more"), Sense::click());
            if response.hovered() {
                ui.painter().rect_filled(br, 3.0, t.control_hover.gamma_multiply(0.6));
            }
            for offset in [-5.0, 0.0, 5.0] {
                ui.painter().circle_filled(br.center() + vec2(offset, 0.0), 1.5, t.text);
            }
            icons::describe_control(ui, &response, "More toolbar commands", None, false);
            egui::Popup::menu(&response).show(|ui| {
                for (position, index) in fit.overflow.iter().enumerate() {
                    let Some(group) = TOOLBAR.get(*index) else { continue };
                    if position > 0 {
                        ui.separator();
                    }
                    for (_, command, label, _) in *group {
                        let response = ui.button(crate::i18n::t(label));
                        crate::control::record_widget(ui, &response, label, "button", None);
                        if response.clicked() {
                            clicked = Some(*command);
                            ui.close();
                        }
                    }
                }
            });
            response.on_hover_text(crate::tl!("More toolbar commands"));
        }
        if let Some(c) = clicked {
            app.start(c);
        }
    });
}

/// The document tab bar's height: the bar and the strip under it that the active tab joins.
pub const FILE_TABS_H: f32 = FILE_BAR_H + FILE_BASE_H;
const FILE_BAR_H: f32 = 26.0;
const FILE_BASE_H: f32 = 4.0;
/// Tabs are parallelograms leaning right by this much, standing 3 px below the bar's top.
const FILE_TAB_SLANT: f32 = 8.0;
const FILE_TAB_TOP: f32 = 3.0;
/// Room left of a tab's name (after the slant) and right of it (or of its ×).
const FILE_TAB_PAD: f32 = 10.0;
/// The × inside a document tab, and the gap before it.
const FILE_TAB_CLOSE: f32 = 12.0;
const FILE_TAB_CLOSE_GAP: f32 = 8.0;
/// A document tab is never wider; a longer name ends in "…".
const FILE_TAB_MAX_W: f32 = 170.0;
/// The "≡" file-tabs menu button and the "+" new-tab button.
const FILE_MENU_W: f32 = 32.0;
const FILE_PLUS_W: f32 = 41.0;
const FILE_BUTTON_H: f32 = 24.0;

/// A tab of the document tab bar: its parallelogram's left edge at the bar's bottom, its width.
fn tab_shape(left: f32, top: f32, bottom: f32, w: f32) -> Vec<egui::Pos2> {
    vec![pos2(left, bottom), pos2(left + FILE_TAB_SLANT, top), pos2(left + w, top), pos2(left + w - FILE_TAB_SLANT, bottom)]
}

/// The "/" AutoCAD draws between tabs, at a tab's left edge.
fn tab_separator(p: &egui::Painter, left: f32, top: f32, bottom: f32) {
    let t = Tokens::get();
    p.line_segment([pos2(left + 2.0, bottom - 4.0), pos2(left + FILE_TAB_SLANT - 2.0, top + 4.0)], Stroke::new(1.0, t.text_faint));
}

/// One tab of the document tab bar, laid out.
struct DocTab {
    /// The drawing's index, `None` for Start.
    doc: Option<usize>,
    label: String,
    dirty: bool,
    active: bool,
    left: f32,
    width: f32,
    galley: std::sync::Arc<egui::Galley>,
    cut: bool,
}

/// AutoCAD's document tab bar, in its order: the "≡" menu (open documents, Start, and New, Open,
/// Save All, Close All), the Start tab (no ×), one tab per drawing (its name, "*" while it has
/// unsaved changes, × inside) and "+" for a new drawing.
pub fn file_tabs(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    egui::Panel::top("cc_file_tabs").exact_size(FILE_TABS_H).frame(egui::Frame::NONE.fill(t.ribbon_tab_row)).show(ui, |ui| {
        let r = ui.max_rect();
        let bottom = r.top() + FILE_BAR_H;
        let top = r.top() + FILE_TAB_TOP;
        ui.painter().rect_filled(Rect::from_min_max(pos2(r.left(), bottom), r.max), 0.0, t.ribbon_bg);
        let menu = Rect::from_min_size(pos2(r.left() + 4.0, bottom - FILE_BUTTON_H), vec2(FILE_MENU_W, FILE_BUTTON_H));
        file_tabs_menu(app, ui, menu);
        let tabs = lay_out_doc_tabs(app, ui, menu.right() + 4.0);
        let p = ui.painter();
        // Inactive tabs: a "/" at their left edge, a fill under the pointer. The active tab joins
        // the strip below and covers its neighbours' separators.
        let responses: Vec<egui::Response> = tabs
            .iter()
            .map(|tab| {
                let id = tab.doc.map_or(ui.id().with("tab_start"), |i| ui.id().with(("tab", i)));
                ui.interact(Rect::from_min_max(pos2(tab.left, top), pos2(tab.left + tab.width, bottom)), id, Sense::click())
            })
            .collect();
        for (tab, resp) in tabs.iter().zip(&responses) {
            if !tab.active {
                tab_separator(p, tab.left, top, bottom);
                if resp.hovered() {
                    p.add(egui::Shape::convex_polygon(tab_shape(tab.left, top, bottom, tab.width), t.ribbon_hover, Stroke::NONE));
                }
            }
        }
        for tab in tabs.iter().filter(|tab| tab.active) {
            p.add(egui::Shape::convex_polygon(tab_shape(tab.left, top, bottom + 1.0, tab.width), t.ribbon_bg, Stroke::NONE));
        }
        let font = crate::theme::ribbon_font(ui.ctx());
        let mut switch_to = None;
        let mut close = None;
        for (tab, resp) in tabs.iter().zip(responses) {
            let color = if tab.active { t.text } else { t.text_dim };
            let text_x = tab.left + FILE_TAB_SLANT + FILE_TAB_PAD;
            let cy = (top + bottom) / 2.0;
            let name_w = tab.galley.size().x;
            p.galley(pos2(text_x, cy - tab.galley.size().y / 2.0), tab.galley.clone(), color);
            let mark = if tab.dirty { "*" } else { "" };
            let mark_w =
                if tab.dirty { p.text(pos2(text_x + name_w, cy), egui::Align2::LEFT_CENTER, mark, font.clone(), color).width() } else { 0.0 };
            let resp = if tab.cut { resp.on_hover_text(format!("{}{mark}", tab.label)) } else { resp };
            if let Some(i) = tab.doc
                && (tab.active || resp.hovered())
            {
                let cr = Rect::from_center_size(
                    pos2(text_x + name_w + mark_w + FILE_TAB_CLOSE_GAP + FILE_TAB_CLOSE / 2.0, cy),
                    vec2(FILE_TAB_CLOSE, FILE_TAB_CLOSE),
                );
                let cresp = ui.interact(cr, resp.id.with("x"), Sense::click());
                icons::paint(ui.painter(), cr, Icon::Close, false);
                icons::describe_control(ui, &cresp, &format!("Close {}", tab.label), None, false);
                if cresp.clicked() {
                    close = Some(i);
                }
            }
            icons::describe_control(ui, &resp, &tab.label, Some(tab.active), true);
            match tab.doc {
                Some(i) => {
                    crate::shortcut_menus::attach(app, &resp, |app| crate::shortcut_menus::drawing_tab_entries(app, i));
                    if resp.clicked() {
                        switch_to = Some(i);
                    }
                }
                None if resp.clicked() => app.ui.start_tab = true,
                None => {}
            }
        }
        // "+": a new drawing, then the bar's closing "/".
        let x = tabs.last().map_or(menu.right() + 4.0, |tab| tab.left + tab.width);
        let plus = Rect::from_min_size(pos2(x, bottom - FILE_BUTTON_H), vec2(FILE_PLUS_W, FILE_BUTTON_H));
        let presp = ui.interact(plus, ui.id().with("tab+"), Sense::click());
        if !tabs.last().is_some_and(|tab| tab.active) {
            tab_separator(ui.painter(), x, top, bottom);
        }
        if presp.hovered() {
            ui.painter().rect_filled(plus.shrink2(vec2(FILE_TAB_SLANT, 0.0)), 2.0, t.ribbon_hover);
        }
        icons::paint(ui.painter(), Rect::from_center_size(plus.center(), vec2(16.0, 16.0)), Icon::Plus, false);
        tab_separator(ui.painter(), plus.right(), top, bottom);
        icons::describe_control(ui, &presp, "New Drawing", None, false);
        if presp.on_hover_text(crate::tl!("New Drawing")).clicked() {
            app.start("new");
            app.ui.start_tab = false;
        }
        if let Some(i) = switch_to {
            let _ = app.run("document.switch", json!({ "index": i }));
            app.ui.start_tab = false;
        }
        if let Some(i) = close {
            app.request_close(Some(i));
        }
    });
}

/// The Start tab and the drawings' tabs from `x`, each as wide as its name (and ×) needs, up to
/// [`FILE_TAB_MAX_W`].
fn lay_out_doc_tabs(app: &CadApp, ui: &egui::Ui, mut x: f32) -> Vec<DocTab> {
    let t = Tokens::get();
    let font = crate::theme::ribbon_font(ui.ctx());
    let on_start = app.ui.start_tab || app.session.docs.is_empty();
    let start = (None, crate::tl!("Start").to_string(), false, on_start);
    let docs = app.session.docs.iter().enumerate().map(|(i, d)| (Some(i), d.title.clone(), d.is_dirty(), i == app.session.active && !on_start));
    std::iter::once(start)
        .chain(docs)
        .map(|(doc, label, dirty, active)| {
            let mark_w = if dirty { ui.painter().layout_no_wrap("*".into(), font.clone(), t.text).size().x } else { 0.0 };
            let close_w = if doc.is_some() { FILE_TAB_CLOSE_GAP + FILE_TAB_CLOSE } else { 0.0 };
            let chrome_w = FILE_TAB_SLANT + 2.0 * FILE_TAB_PAD + close_w + mark_w;
            let color = if active { t.text } else { t.text_dim };
            let (galley, cut) = elided(ui.painter(), &label, font.clone(), color, FILE_TAB_MAX_W - chrome_w);
            let width = chrome_w + galley.size().x;
            let tab = DocTab { doc, label, dirty, active, left: x, width, galley, cut };
            x += width;
            tab
        })
        .collect()
}

/// The "≡" button at `rect` and its menu.
fn file_tabs_menu(app: &mut CadApp, ui: &mut egui::Ui, rect: Rect) {
    let t = Tokens::get();
    let resp = ui.interact(rect, ui.id().with("file-tabs-menu"), Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
    icons::paint(ui.painter(), Rect::from_center_size(rect.center(), vec2(18.0, 18.0)), Icon::Menu, false);
    icons::describe_control(ui, &resp, "File Tabs", None, false);
    let resp = resp.on_hover_text(crate::tl!("File Tabs"));
    let mut chosen = None;
    egui::Popup::from_toggle_button_response(&resp).show(|ui| {
        ui.set_min_width(200.0);
        crate::context_menu::groups_ui(ui, &crate::shortcut_menus::file_tabs_menu_entries(app), &mut chosen);
    });
    if let Some(id) = chosen {
        crate::shortcut_menus::choose(app, ui.ctx(), &id);
    }
}

/// A click widget over `rect`, disabled unless `enabled`. Unlike `add_enabled_ui` it takes no
/// room in `ui`'s layout, so painted controls never grow the panel they sit in.
pub(crate) fn interact_enabled(ui: &mut egui::Ui, rect: Rect, id: egui::Id, enabled: bool) -> egui::Response {
    if enabled {
        return ui.interact(rect, id, Sense::click());
    }
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
    child.disable();
    child.interact(rect, id, Sense::click())
}

/// One menu row of `size`: a check mark column (`checked: Some` for check items) and the label.
pub(crate) fn check_row(ui: &mut egui::Ui, label: &str, checked: Option<bool>, enabled: bool, size: egui::Vec2) -> egui::Response {
    let (resp, color) = menu_row_frame(ui, checked, enabled, size);
    let galley = ui.painter().layout_no_wrap(crate::i18n::t(label).to_string(), crate::theme::ribbon_font(ui.ctx()), color);
    ui.painter().galley(pos2(resp.rect.left() + MENU_TEXT_X, resp.rect.center().y - galley.size().y / 2.0), galley, color);
    icons::describe_control(ui, &resp, label, checked, false);
    resp
}

/// Where a menu row's label starts, right of the check mark column.
const MENU_TEXT_X: f32 = 26.0;

/// A menu row's rectangle, hover fill and check mark; the caller paints the label in the colour
/// returned (dimmed when disabled).
fn menu_row_frame(ui: &mut egui::Ui, checked: Option<bool>, enabled: bool, size: egui::Vec2) -> (egui::Response, egui::Color32) {
    let t = Tokens::get();
    let resp = ui.add_enabled_ui(enabled, |ui| ui.allocate_exact_size(size, Sense::click()).1).inner;
    let rect = resp.rect;
    if resp.hovered() && resp.enabled() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
    let color = if enabled { t.text } else { t.text_faint };
    if checked == Some(true) {
        let c = pos2(rect.left() + 12.0, rect.center().y);
        let points = vec![pos2(c.x - 4.0, c.y), pos2(c.x - 1.0, c.y + 3.0), pos2(c.x + 4.5, c.y - 3.5)];
        ui.painter().add(egui::Shape::line(points, Stroke::new(1.5, color)));
    }
    (resp, color)
}

/// `text` laid out on one line no wider than `max_width`, ending in "…" when it had to be cut;
/// and whether it was.
fn elided(p: &egui::Painter, text: &str, font: egui::FontId, color: egui::Color32, max_width: f32) -> (std::sync::Arc<egui::Galley>, bool) {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), font, color);
    job.wrap = egui::text::TextWrapping { max_width: max_width.max(0.0), max_rows: 1, break_anywhere: true, overflow_character: Some('…') };
    let galley = p.layout_job(job);
    let cut = galley.elided;
    (galley, cut)
}

/// Which of the tabs with these `widths` to show in `room`: all of them when they fit, else a run
/// that holds the `active` one (shown even when it alone is too wide), starting as early as it can.
fn visible_tabs(widths: &[f32], active: Option<usize>, room: f32) -> std::ops::Range<usize> {
    let fits = |range: std::ops::Range<usize>| widths.get(range).is_some_and(|w| w.iter().sum::<f32>() <= room);
    let mut end = 0;
    while end < widths.len() && fits(0..end + 1) {
        end += 1;
    }
    let Some(active) = active.filter(|a| *a >= end && *a < widths.len()) else { return 0..end };
    let mut start = active;
    while start > 0 && fits(start - 1..active + 1) {
        start -= 1;
    }
    start..active + 1
}

/// The status bar row: the layout bar and the status bar items ([`statusbar`]).
pub fn status_bar(app: &mut CadApp, ui: &mut egui::Ui) {
    statusbar::show(app, ui);
}

/// The Start page (no drawing open, or the Start tab).
pub fn start_page(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    let r = ui.max_rect();
    let p = ui.painter().clone();
    let left = Rect::from_min_size(r.min, vec2(300.0, r.height()));
    p.rect_filled(left, 0.0, t.chrome_dark);
    p.rect_filled(Rect::from_min_max(pos2(left.right(), r.top()), r.max), 0.0, t.chrome);
    p.text(pos2(left.left() + 34.0, left.top() + 70.0), egui::Align2::LEFT_CENTER, crate::tl!("AsenaCAD"), egui::FontId::proportional(28.0), t.text);
    p.text(
        pos2(left.left() + 34.0, left.top() + 98.0),
        egui::Align2::LEFT_CENTER,
        crate::tf!("Version {version}", version = env!("CARGO_PKG_VERSION")),
        crate::theme::small(),
        t.text_faint,
    );
    let mut y = left.top() + 140.0;
    let mut action = None;
    for (label, cmd) in [("Open...", "open"), ("New", "new"), ("New (metric)", "new.metric"), ("Open sample drawing", "sample")] {
        let br = Rect::from_min_size(pos2(left.left() + 34.0, y), vec2(220.0, 34.0));
        let resp = ui.interact(br, ui.id().with(("start", cmd)), Sense::click());
        p.rect_stroke(br, 2.0, Stroke::new(1.0, if resp.hovered() { t.text } else { t.text_dim }), egui::StrokeKind::Inside);
        p.text(pos2(br.left() + 16.0, br.center().y), egui::Align2::LEFT_CENTER, crate::i18n::t(label), crate::theme::body(), t.text);
        if resp.clicked() {
            action = Some(cmd);
        }
        y += 44.0;
    }
    let main = Rect::from_min_max(pos2(left.right() + 36.0, r.top() + 40.0), r.max);
    p.text(main.min, egui::Align2::LEFT_TOP, crate::tl!("Recent"), egui::FontId::proportional(22.0), t.text);
    let mut recent = None;
    let mut y = main.top() + 44.0;
    for (i, path) in app.ui.recent_files.iter().enumerate() {
        let row = Rect::from_min_size(pos2(main.left(), y), vec2((r.right() - main.left() - 36.0).clamp(0.0, 640.0), 42.0));
        let resp = ui.interact(row, ui.id().with(("recent", i)), Sense::click());
        if resp.hovered() {
            p.rect_filled(row, 2.0, t.control_hover.gamma_multiply(0.5));
        }
        let dir = std::path::Path::new(path).parent().map(|d| d.display().to_string()).unwrap_or_default();
        let clip = p.with_clip_rect(row.shrink(2.0));
        clip.text(pos2(row.left() + 10.0, row.top() + 13.0), egui::Align2::LEFT_CENTER, crate::menus::file_name(path), crate::theme::body(), t.text);
        clip.text(pos2(row.left() + 10.0, row.top() + 30.0), egui::Align2::LEFT_CENTER, dir, crate::theme::small(), t.text_faint);
        if resp.on_hover_text(path).clicked() {
            recent = Some(path.clone());
        }
        y += 46.0;
    }
    if let Some(path) = recent {
        app.open_path(&path);
    } else if app.ui.recent_files.is_empty() {
        p.text(
            pos2(main.center().x, main.center().y),
            egui::Align2::CENTER_CENTER,
            crate::tl!("There is nothing here yet."),
            egui::FontId::proportional(18.0),
            t.text,
        );
        p.text(
            pos2(main.center().x, main.center().y + 26.0),
            egui::Align2::CENTER_CENTER,
            crate::tl!("To get started, create or open a drawing."),
            crate::theme::body(),
            t.text_dim,
        );
    }
    match action {
        Some("open") => app.start("ui.open"),
        Some("new") => {
            app.start("new");
            app.ui.start_tab = false;
        }
        Some("new.metric") => {
            let _ = app.run("new", json!({ "metric": true }));
            app.ui.start_tab = false;
        }
        Some("sample") => {
            app.start("ui.sample");
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_title_row_content_clears_the_traffic_lights() {
        // The lights are centred in the 28 pt band, and content keeps the same gap after them.
        let margin = (MAC_TITLE_BAR - MAC_LIGHT) / 2.0;
        let lights_end = MAC_LIGHTS_LEFT + 2.0 * MAC_LIGHT_PITCH + MAC_LIGHT;
        assert_eq!(MAC_CONTENT_LEFT - lights_end, margin);
    }

    #[test]
    fn document_tabs_are_parallelograms_leaning_right() {
        let shape = tab_shape(10.0, 3.0, 26.0, 100.0);
        assert_eq!(shape, [pos2(10.0, 26.0), pos2(18.0, 3.0), pos2(110.0, 3.0), pos2(102.0, 26.0)]);
    }

    #[test]
    fn layout_tabs_that_do_not_fit_keep_the_current_one() {
        let w = [50.0; 6];
        // All fit, with or without a current tab.
        assert_eq!(visible_tabs(&w, None, 300.0), 0..6);
        assert_eq!(visible_tabs(&w, Some(5), 300.0), 0..6);
        // Too many: the first ones, unless the current one is further on.
        assert_eq!(visible_tabs(&w, None, 170.0), 0..3);
        assert_eq!(visible_tabs(&w, Some(1), 170.0), 0..3);
        assert_eq!(visible_tabs(&w, Some(4), 170.0), 2..5);
        // No room at all: nothing but the current tab; hostile numbers don't panic.
        assert_eq!(visible_tabs(&w, None, -10.0), 0..0);
        assert_eq!(visible_tabs(&w, Some(3), 0.0), 3..4);
        assert_eq!(visible_tabs(&w, Some(9), f32::NAN), 0..0);
        assert_eq!(visible_tabs(&[], Some(0), 100.0), 0..0);
    }
}
