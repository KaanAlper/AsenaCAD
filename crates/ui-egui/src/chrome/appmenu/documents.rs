//! The Application menu's right pane: Recent Documents (the recent-files list, by recency or by
//! file type) or Open Documents (the open drawings), as a list or with larger icons. Clicking a
//! document opens it, or switches to it when it is open already.

use egui::{Pos2, Rect, Stroke, Ui, Vec2, pos2, vec2};

use super::{Act, Docs, Dropdown, Order, State, View, hit, text};
use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// The pane's title, its buttons and its list, relative to the menu's top-left.
const TITLE: Pos2 = Pos2::new(152.0, 49.0);
const ORDER_BUTTON: Rect = Rect { min: Pos2::new(152.0, 73.0), max: Pos2::new(250.0, 95.0) };
const VIEW_BUTTON_W: f32 = 34.0;
const VIEW_BUTTON_GAP: f32 = 5.0;
const LIST: Rect = Rect { min: Pos2::new(144.0, 108.0), max: Pos2::new(423.0, 523.0) };
const LIST_ROW_H: f32 = 22.0;
const ICON_ROW_H: f32 = 44.0;
const GROUP_H: f32 = 20.0;
const DROPDOWN_ROW_H: f32 = 22.0;
const DROPDOWN_W: f32 = 150.0;

impl Order {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Order::Ordered => crate::tl!("By Ordered List"),
            Order::Type => crate::tl!("By Type"),
        }
    }
}

impl View {
    pub(crate) fn label(self) -> &'static str {
        match self {
            View::List => crate::tl!("List"),
            View::Icons => crate::tl!("Icons"),
        }
    }
}

/// The "ordered by" button: AutoCAD's 98 px, or wider when our font needs it for the longest
/// ordering's name and the ▾.
fn order_button(ui: &Ui, o: Pos2) -> Rect {
    let font = super::font(ui);
    let widest = [Order::Ordered, Order::Type]
        .iter()
        .map(|v| ui.fonts_mut(|f| f.layout_no_wrap(v.label().to_string(), font.clone(), egui::Color32::WHITE).size().x))
        .fold(0.0, f32::max);
    let w = ORDER_BUTTON.width().max(widest + 24.0);
    Rect::from_min_size(o + ORDER_BUTTON.min.to_vec2(), vec2(w, ORDER_BUTTON.height()))
}

fn view_button(ui: &Ui, o: Pos2, docs: Docs) -> Rect {
    let left = if docs == Docs::Recent { order_button(ui, o).right() + VIEW_BUTTON_GAP } else { o.x + ORDER_BUTTON.left() };
    Rect::from_min_size(pos2(left, o.y + ORDER_BUTTON.top()), vec2(VIEW_BUTTON_W, ORDER_BUTTON.height()))
}

/// One document row.
struct Doc {
    name: String,
    /// The folder (recent files) or the full path (open drawings), shown under the name with
    /// larger icons and as the tooltip.
    detail: String,
    act: Act,
    current: bool,
}

/// The file name's extension in capitals ("DWG"), for grouping by type.
fn kind(path: &str) -> String {
    std::path::Path::new(path).extension().map(|e| e.to_string_lossy().to_uppercase()).unwrap_or_default()
}

fn folder(path: &str) -> String {
    std::path::Path::new(path).parent().map(|p| p.to_string_lossy().to_string()).unwrap_or_default()
}

/// The recent files as rows, grouped under their file type when ordered by type.
fn recent(app: &CadApp, order: Order) -> Vec<(Option<String>, Doc)> {
    let mut files: Vec<&String> = app.ui.recent_files.iter().collect();
    if order == Order::Type {
        // A stable sort keeps the most recent first within each type.
        files.sort_by_key(|p| kind(p));
    }
    let mut last: Option<String> = None;
    files
        .into_iter()
        .map(|path| {
            let group = (order == Order::Type).then(|| kind(path)).filter(|k| last.as_ref() != Some(k));
            if order == Order::Type {
                last = Some(kind(path));
            }
            let doc = Doc { name: crate::menus::file_name(path), detail: folder(path), act: Act::OpenFile(path.clone()), current: false };
            (group, doc)
        })
        .collect()
}

fn open(app: &CadApp) -> Vec<(Option<String>, Doc)> {
    app.session
        .docs
        .iter()
        .enumerate()
        .map(|(i, d)| {
            let doc = Doc {
                name: d.title.clone(),
                detail: d.path.clone().unwrap_or_default(),
                act: Act::Switch(i),
                current: i == app.session.active && !app.ui.start_tab,
            };
            (None, doc)
        })
        .collect()
}

/// The pane in its current mode.
pub(super) fn show(app: &CadApp, ui: &mut Ui, o: Pos2, st: &mut State, act: &mut Option<Act>) {
    let t = Tokens::get();
    let (icon, title) = match st.docs {
        Docs::Recent => (Icon::RecentDocs, crate::tl!("Recent Documents")),
        Docs::Open => (Icon::OpenDocs, crate::tl!("Open Documents")),
    };
    icons::paint(ui.painter(), Rect::from_min_size(o + TITLE.to_vec2(), Vec2::splat(16.0)), icon, false);
    text(ui, o + TITLE.to_vec2() + vec2(20.0, 1.0), title, super::font(ui), t.text, 240.0, false);
    if st.docs == Docs::Recent {
        let rect = order_button(ui, o);
        let resp = hit(ui, rect, ui.id().with("appmenu-order"), true, st.dropdown == Some(Dropdown::Order));
        let label = st.order.label();
        text(ui, pos2(rect.left() + 4.0, rect.center().y - 7.0), label, super::font(ui), t.text, rect.width() - 20.0, false);
        icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 8.0, rect.center().y), Vec2::splat(9.0)), Icon::ChevronDown, false);
        icons::describe_control(ui, &resp, label, None, false);
        if resp.clicked() {
            st.dropdown = if st.dropdown == Some(Dropdown::Order) { None } else { Some(Dropdown::Order) };
        }
    }
    let rect = view_button(ui, o, st.docs);
    let resp = hit(ui, rect, ui.id().with("appmenu-view"), true, st.dropdown == Some(Dropdown::View));
    let icon = if st.view == View::List { Icon::ViewList } else { Icon::ViewIcons };
    icons::paint(ui.painter(), Rect::from_min_size(pos2(rect.left() + 3.0, rect.center().y - 8.0), Vec2::splat(16.0)), icon, false);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 8.0, rect.center().y), Vec2::splat(9.0)), Icon::ChevronDown, false);
    icons::describe_control(ui, &resp, crate::tl!("Document View"), None, false);
    let resp = resp.on_hover_text(crate::tl!("Document View"));
    if resp.clicked() {
        st.dropdown = if st.dropdown == Some(Dropdown::View) { None } else { Some(Dropdown::View) };
    }

    let rows = match st.docs {
        Docs::Recent => recent(app, st.order),
        Docs::Open => open(app),
    };
    let list = LIST.translate(o.to_vec2());
    let row_h = if st.view == View::List { LIST_ROW_H } else { ICON_ROW_H };
    let mut y = list.top();
    for (k, (group, doc)) in rows.into_iter().enumerate() {
        if let Some(group) = group {
            if y + GROUP_H > list.bottom() {
                break;
            }
            text(ui, pos2(list.left() + 8.0, y + 4.0), &group, super::font(ui), t.text_dim, list.width() - 16.0, true);
            let rule = y + GROUP_H - 2.0;
            ui.painter().line_segment([pos2(list.left() + 8.0, rule), pos2(list.right() - 8.0, rule)], Stroke::new(1.0, t.border));
            y += GROUP_H;
        }
        if y + row_h > list.bottom() {
            break;
        }
        let rect = Rect::from_min_size(pos2(list.left() + 4.0, y), vec2(list.width() - 8.0, row_h));
        y += row_h;
        if let Some(a) = document(ui, rect, k, &doc, st.view) {
            *act = Some(a);
        }
    }
}

fn document(ui: &mut Ui, rect: Rect, k: usize, doc: &Doc, view: View) -> Option<Act> {
    let t = Tokens::get();
    let resp = hit(ui, rect, ui.id().with(("appmenu-doc", k, &doc.name)), true, doc.current);
    match view {
        View::List => {
            icons::paint(ui.painter(), Rect::from_min_size(pos2(rect.left() + 4.0, rect.center().y - 8.0), Vec2::splat(16.0)), Icon::Document, false);
            let (galley, _) = crate::chrome::elided(ui.painter(), &doc.name, super::font(ui), t.text, rect.width() - 30.0);
            ui.painter().galley(pos2(rect.left() + 26.0, rect.center().y - galley.size().y / 2.0), galley, t.text);
        }
        View::Icons => {
            icons::paint(
                ui.painter(),
                Rect::from_min_size(pos2(rect.left() + 6.0, rect.center().y - 16.0), Vec2::splat(32.0)),
                Icon::Document,
                false,
            );
            let x = rect.left() + 46.0;
            let w = rect.width() - 52.0;
            let (name, _) = crate::chrome::elided(ui.painter(), &doc.name, super::font(ui), t.text, w);
            ui.painter().galley(pos2(x, rect.top() + 6.0), name, t.text);
            let (detail, _) = crate::chrome::elided(ui.painter(), &doc.detail, egui::FontId::proportional(11.0), t.text_dim, w);
            ui.painter().galley(pos2(x, rect.top() + 24.0), detail, t.text_dim);
        }
    }
    icons::describe_control(ui, &resp, &doc.name, None, false);
    let resp = if doc.detail.is_empty() { resp } else { resp.on_hover_text(&doc.detail) };
    resp.clicked().then(|| doc.act.clone())
}

/// The open drop-down of the pane's Order or View button, drawn over everything else.
pub(super) fn dropdown(ui: &mut Ui, o: Pos2, st: &mut State) {
    let Some(which) = st.dropdown else { return };
    if !st.query.trim().is_empty() || st.pane != super::Pane::Documents {
        st.dropdown = None;
        return;
    }
    let t = Tokens::get();
    let (button, labels): (Rect, Vec<(&str, bool)>) = match which {
        Dropdown::Order => (order_button(ui, o), [Order::Ordered, Order::Type].iter().map(|v| (v.label(), *v == st.order)).collect()),
        Dropdown::View => (view_button(ui, o, st.docs), [View::List, View::Icons].iter().map(|v| (v.label(), *v == st.view)).collect()),
    };
    let rect = Rect::from_min_size(pos2(button.left(), button.bottom() + 1.0), vec2(DROPDOWN_W, DROPDOWN_ROW_H * labels.len() as f32 + 4.0));
    // Under the rows, so a click between them doesn't reach the pane.
    let _ = ui.interact(rect, ui.id().with("appmenu-dropdown-back"), egui::Sense::click());
    ui.painter().rect_filled(rect, 2.0, t.panel);
    ui.painter().rect_stroke(rect, 2.0, Stroke::new(1.0, t.border), egui::StrokeKind::Inside);
    for (k, (label, on)) in labels.into_iter().enumerate() {
        let row = Rect::from_min_size(rect.min + vec2(2.0, 2.0 + DROPDOWN_ROW_H * k as f32), vec2(rect.width() - 4.0, DROPDOWN_ROW_H));
        let resp = hit(ui, row, ui.id().with(("appmenu-dropdown", k)), true, false);
        if on {
            let c = pos2(row.left() + 10.0, row.center().y);
            let tick = Stroke::new(1.5, t.text);
            ui.painter().line_segment([c + vec2(-4.0, 0.0), c + vec2(-1.0, 3.0)], tick);
            ui.painter().line_segment([c + vec2(-1.0, 3.0), c + vec2(4.0, -3.5)], tick);
        }
        text(ui, pos2(row.left() + 22.0, row.center().y - 7.0), label, super::font(ui), t.text, row.width() - 26.0, false);
        icons::describe_control(ui, &resp, label, Some(on), false);
        if resp.clicked() {
            match (which, k) {
                (Dropdown::Order, 0) => st.order = Order::Ordered,
                (Dropdown::Order, _) => st.order = Order::Type,
                (Dropdown::View, 0) => st.view = View::List,
                (Dropdown::View, _) => st.view = View::Icons,
            }
            st.dropdown = None;
        }
    }
    let pressed_elsewhere =
        ui.input(|i| i.pointer.any_pressed() && i.pointer.press_origin().is_some_and(|p| !rect.contains(p) && !button.contains(p)));
    if pressed_elsewhere {
        st.dropdown = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_files_group_by_type_keeping_recency() {
        let mut app = CadApp::new(asenacad_engine::Session::default(), crate::Services::default());
        app.ui.recent_files = ["/a/new.dxf", "/b/plan.dwg", "/a/old.dxf", "/c/sheet.dwt"].map(String::from).to_vec();
        let names = |rows: Vec<(Option<String>, Doc)>| rows.into_iter().map(|(g, d)| (g, d.name)).collect::<Vec<_>>();
        let by_list = names(recent(&app, Order::Ordered));
        assert!(by_list.iter().all(|(g, _)| g.is_none()));
        assert_eq!(by_list.iter().map(|(_, n)| n.as_str()).collect::<Vec<_>>(), ["new.dxf", "plan.dwg", "old.dxf", "sheet.dwt"]);
        let by_type = names(recent(&app, Order::Type));
        let expected = [(Some("DWG"), "plan.dwg"), (Some("DWT"), "sheet.dwt"), (Some("DXF"), "new.dxf"), (None, "old.dxf")];
        assert_eq!(by_type.iter().map(|(g, n)| (g.as_deref(), n.as_str())).collect::<Vec<_>>(), expected);
    }
}
