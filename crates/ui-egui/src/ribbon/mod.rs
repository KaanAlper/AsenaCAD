//! AutoCAD's ribbon: a row of tabs, and below it the selected tab's panels of command buttons,
//! built from static tables. Panels shrink and collapse into panel buttons when the window is
//! narrow (`layout`), and drop their slide-outs or show as popups over the drawing (`slideout`).
//!
//! Every item has a stable key, "<tab>/<panel>/<i>" or "<tab>/<panel>/slideout/<i>", the same
//! wherever the panel is shown (strip, collapsed-panel popup, slide-out): split buttons remember
//! their last entry by it, and its egui id scope keeps equally named items apart.

pub mod annotate;
pub(crate) mod avail;
mod bind;
mod build;
pub mod home;
pub mod insert;
pub mod layout;
pub mod model;
pub mod slideout;
pub mod widgets;

pub use bind::{combo, slider};

use egui::{Align, Area, Id, Layout, Order, Pos2, Rect, Sense, Ui, UiBuilder, Vec2, pos2, vec2};
use serde::{Deserialize, Serialize};

use layout::{DIVIDER, Fit, PANEL_BUTTON_W, PANEL_H, PANEL_TOP, TAB_H, TAB_PAD, TAB_ROW_H, TAB_TOP, TITLE_H};
use model::{Item, Panel, Size, Tab};

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// What a tab whose panels are not built yet shows in the strip.
pub const UNBUILT_TAB_NOTE: &str = "This tab is not built yet (M1b)";
/// A collapsed panel's face is inset this much in its panel.
const PANEL_BUTTON_INSET: f32 = 2.0;

/// How far the ribbon is minimized: AutoCAD's minimize button cycles through these in order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RibbonMin {
    #[default]
    Full,
    PanelButtons,
    PanelTitles,
    Tabs,
}

impl RibbonMin {
    pub const ALL: [Self; 4] = [Self::Full, Self::PanelButtons, Self::PanelTitles, Self::Tabs];

    /// The state's name in the minimize menu (and the minimize button's, when it leads there).
    pub const fn label(self) -> &'static str {
        match self {
            Self::Full => "Show Full Ribbon",
            Self::PanelButtons => "Minimize to Panel Buttons",
            Self::PanelTitles => "Minimize to Panel Titles",
            Self::Tabs => "Minimize to Tabs",
        }
    }

    /// The state the minimize button goes to next.
    pub const fn next(self) -> Self {
        match self {
            Self::Full => Self::PanelButtons,
            Self::PanelButtons => Self::PanelTitles,
            Self::PanelTitles => Self::Tabs,
            Self::Tabs => Self::Full,
        }
    }
}

/// The ribbon's height (tab row and panel strip) in `min`.
pub fn height(min: RibbonMin) -> f32 {
    TAB_ROW_H
        + match min {
            RibbonMin::Full => layout::STRIP_H,
            RibbonMin::PanelButtons => layout::BUTTONS_STRIP_H,
            RibbonMin::PanelTitles => layout::TITLES_STRIP_H,
            RibbonMin::Tabs => 0.0,
        }
}

/// The tab named `name`, or the first tab when no tab has that name.
pub fn current_tab(name: &str) -> &'static Tab {
    let tabs = model::tabs();
    tabs.iter().find(|t| t.label == name).or(tabs.first()).unwrap_or(&home::HOME)
}

/// The ribbon: tab row and the selected tab's panels, as a top panel of `ui`.
pub fn show(app: &mut CadApp, ui: &mut Ui) {
    let t = Tokens::get();
    egui::Panel::top("asenacad_ribbon").exact_size(height(app.ui.ribbon_min)).frame(egui::Frame::NONE.fill(t.ribbon_tab_row)).show(ui, |ui| {
        // Combo boxes and other egui widgets in the panels use the ribbon's text size too.
        ui.style_mut().override_font_id = Some(crate::theme::ribbon_font(ui.ctx()));
        let r = ui.max_rect();
        let tab = current_tab(&app.ui.ribbon_tab);
        let row = Rect::from_min_size(r.min, vec2(r.width(), TAB_ROW_H));
        tab_row(app, ui, row, tab);
        let strip = Rect::from_min_max(pos2(r.left(), row.bottom()), pos2(r.right(), row.bottom() + layout::STRIP_H));
        match app.ui.ribbon_min {
            RibbonMin::Full => panel_strip(app, ui, strip, tab),
            RibbonMin::PanelButtons => minimized_strip(app, ui, strip, tab, false),
            RibbonMin::PanelTitles => minimized_strip(app, ui, strip, tab, true),
            RibbonMin::Tabs if slideout::strip_open(ui.ctx()) => {
                slideout::add_inside(ui.ctx(), row);
                let ctx = ui.ctx().clone();
                Area::new(Id::new("asenacad-ribbon-strip")).order(Order::Foreground).fixed_pos(strip.min).show(&ctx, |ui| {
                    ui.style_mut().override_font_id = Some(crate::theme::ribbon_font(ui.ctx()));
                    let (rect, _) = ui.allocate_exact_size(strip.size(), Sense::hover());
                    ui.painter().rect_filled(rect, 0.0, t.ribbon_tab_row);
                    panel_strip(app, ui, rect, tab);
                    slideout::add_inside(ui.ctx(), rect);
                });
            }
            RibbonMin::Tabs => {}
        }
        slideout::track(ui.ctx());
    });
}

fn label_galley(ui: &Ui, text: &str, color: egui::Color32) -> std::sync::Arc<egui::Galley> {
    ui.painter().layout_no_wrap(text.to_string(), crate::theme::ribbon_font(ui.ctx()), color)
}

fn hover_fill(ui: &Ui, rect: Rect, resp: &egui::Response, on: bool) {
    let t = Tokens::get();
    if on || resp.is_pointer_button_down_on() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_pressed);
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
    }
}

/// Tabs in order, the selected one highlighted, then the minimize button and its ▾ menu.
fn tab_row(app: &mut CadApp, ui: &mut Ui, row: Rect, current: &Tab) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    let mut x = row.left() + 4.0;
    for tab in model::tabs() {
        let selected = std::ptr::eq(tab, current);
        let galley = label_galley(ui, crate::i18n::t(tab.label), if selected { t.text } else { t.text_dim });
        let rect = Rect::from_min_size(pos2(x, row.top() + TAB_TOP), vec2(galley.size().x + 2.0 * TAB_PAD, TAB_H));
        let resp = ui.interact(rect, ui.id().with(("ribbon-tab", tab.label)), Sense::click());
        // The selected tab is part of the panel strip below it (unless the ribbon is down to its
        // tabs).
        let joined = app.ui.ribbon_min != RibbonMin::Tabs || slideout::strip_open(&ctx);
        if selected {
            let radius = egui::CornerRadius { nw: 2, ne: 2, sw: if joined { 0 } else { 2 }, se: if joined { 0 } else { 2 } };
            ui.painter().rect_filled(rect, radius, t.ribbon_bg);
        } else if resp.hovered() {
            ui.painter().rect_filled(rect, 2.0, t.ribbon_hover);
        }
        let y = rect.center().y - galley.size().y / 2.0;
        ui.painter().galley(pos2(rect.left() + TAB_PAD, y), galley, t.text);
        icons::describe_control(ui, &resp, tab.label, Some(selected), true);
        if resp.double_clicked() {
            // AutoCAD: double-clicking a tab steps through the minimize states.
            app.ui.ribbon_min = app.ui.ribbon_min.next();
            slideout::close(&ctx);
        } else if resp.clicked() {
            if selected && app.ui.ribbon_min == RibbonMin::Tabs {
                slideout::toggle_strip(&ctx);
            } else {
                app.ui.ribbon_tab = tab.label.to_string();
                slideout::close(&ctx);
                if app.ui.ribbon_min == RibbonMin::Tabs {
                    slideout::toggle_strip(&ctx);
                }
            }
        }
        x = rect.right() + 1.0;
    }
    let button = Rect::from_min_size(pos2(x + 6.0, row.top() + TAB_TOP + 1.0), vec2(20.0, 18.0));
    let resp = ui.interact(button, ui.id().with("ribbon-minimize"), Sense::click());
    hover_fill(ui, button, &resp, false);
    let up = app.ui.ribbon_min == RibbonMin::Full;
    icons::paint(ui.painter(), button.shrink(3.0), if up { Icon::ChevronUp } else { Icon::ChevronDown }, false);
    let next = app.ui.ribbon_min.next();
    icons::describe_control(ui, &resp, next.label(), None, false);
    if resp.on_hover_text(crate::i18n::t(next.label())).clicked() {
        app.ui.ribbon_min = next;
        slideout::close(&ctx);
    }
    let arrow = Rect::from_min_size(pos2(button.right(), button.top()), vec2(10.0, 18.0));
    let resp = ui.interact(arrow, ui.id().with("ribbon-minimize-menu"), Sense::click());
    hover_fill(ui, arrow, &resp, false);
    icons::paint(ui.painter(), Rect::from_center_size(arrow.center(), Vec2::splat(9.0)), Icon::ChevronDown, false);
    icons::describe_control(ui, &resp, "Minimize Options", None, false);
    let resp = resp.on_hover_text(crate::tl!("Minimize Options"));
    egui::Popup::from_toggle_button_response(&resp).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
        ui.set_min_width(190.0);
        for state in RibbonMin::ALL {
            if ui.selectable_label(app.ui.ribbon_min == state, crate::i18n::t(state.label())).clicked() {
                app.ui.ribbon_min = state;
                slideout::close(ui.ctx());
            }
        }
    });
}

/// The panels of `tab` packed left to right in `strip`, shrinking and collapsing from the right
/// when they don't fit.
fn panel_strip(app: &mut CadApp, ui: &mut Ui, strip: Rect, tab: &Tab) {
    let t = Tokens::get();
    let left = strip.left() + 2.0;
    if tab.panels.is_empty() {
        unbuilt_note(ui, Rect::from_min_size(pos2(left, strip.top() + PANEL_TOP), vec2(strip.right() - 2.0 - left, PANEL_H)));
        return;
    }
    let full: Vec<f32> = tab.panels.iter().map(|p| layout::panel_width(ui, tab.label, p, false) + DIVIDER).collect();
    let compact: Vec<f32> = tab.panels.iter().map(|p| layout::panel_width(ui, tab.label, p, true) + DIVIDER).collect();
    let buttons: Vec<f32> = tab.panels.iter().map(|p| layout::panel_button_width(ui, p) + DIVIDER).collect();
    let fits = layout::fit(&full, &compact, &buttons, strip.right() - left);
    ui.scope_builder(UiBuilder::new().max_rect(strip), |ui| {
        ui.shrink_clip_rect(strip);
        let mut x = left;
        for (i, (panel, fit)) in tab.panels.iter().zip(&fits).enumerate() {
            let key = format!("{}/{}", tab.label, panel.label);
            let top = strip.top() + PANEL_TOP;
            let w = match fit {
                Fit::Collapsed => buttons.get(i).copied().unwrap_or(PANEL_BUTTON_W + DIVIDER) - DIVIDER,
                Fit::Compact => compact.get(i).copied().unwrap_or(0.0) - DIVIDER,
                Fit::Full => full.get(i).copied().unwrap_or(0.0) - DIVIDER,
            };
            let rect = Rect::from_min_size(pos2(x, top), vec2(w, PANEL_H));
            ui.painter().rect_filled(rect, 0.0, t.ribbon_bg);
            if *fit == Fit::Collapsed {
                panel_button(app, ui, rect, &key, tab, panel);
            } else {
                show_panel(app, ui, rect, &key, panel, *fit == Fit::Compact, false);
                if slideout::slideout_open(ui.ctx(), &key, false) {
                    slideout::add_inside(ui.ctx(), rect);
                    slideout::slideout_area(app, ui, pos2(rect.left(), rect.bottom()), rect.width(), &key, panel);
                }
            }
            // The frame shows through the gap between panels.
            x = rect.right() + DIVIDER;
        }
    });
}

/// The strip of a tab whose panels are not built yet: one dim, centred note.
fn unbuilt_note(ui: &mut Ui, rect: Rect) {
    let t = Tokens::get();
    ui.painter().rect_filled(rect, 0.0, t.ribbon_bg);
    let text = crate::tl!("This tab is not built yet (M1b)");
    let galley = label_galley(ui, text, t.text_faint);
    let at = rect.center() - galley.size() / 2.0;
    let resp = ui.interact(Rect::from_min_size(at, galley.size()), ui.id().with("ribbon-unbuilt-tab"), Sense::hover());
    ui.painter().galley(at, galley, t.text_faint);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, text));
    crate::control::record_widget(ui, &resp, UNBUILT_TAB_NOTE, "label", None);
}

/// One expanded panel in `rect`: its items and its title bar (slide-out "▾", launcher).
/// `popup`: the panel is shown in its collapsed button's popup.
fn show_panel(app: &mut CadApp, ui: &mut Ui, rect: Rect, key: &str, panel: &Panel, compact: bool, popup: bool) {
    let (_, content_w) = layout::place(ui, key, panel.items, compact, Pos2::ZERO);
    render_items(ui, app, key, panel.items, compact, pos2(layout::content_left(rect, content_w), rect.top()));
    title_bar(app, ui, Rect::from_min_max(pos2(rect.left(), rect.bottom() - TITLE_H), rect.max), key, panel, popup);
}

/// The panel's name; with a slide-out the bar is a toggle with "▾" after the name. The dialog
/// launcher sits at its right end.
fn title_bar(app: &mut CadApp, ui: &mut Ui, bar: Rect, key: &str, panel: &Panel, popup: bool) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    ui.painter().rect_filled(bar, 0.0, t.ribbon_panel_title);
    let launcher = panel.launcher.map(|cmd| (cmd, Rect::from_min_size(pos2(bar.right() - widgets::LAUNCHER.x, bar.top()), widgets::LAUNCHER)));
    let title = Rect::from_min_max(bar.min, pos2(launcher.map_or(bar.right(), |(_, r)| r.left()), bar.bottom()));
    let open = slideout::slideout_open(&ctx, key, popup);
    let galley = label_galley(ui, crate::i18n::t(panel.label), t.text);
    let has_slideout = !panel.slideout.is_empty();
    let text_w = galley.size().x + if has_slideout { 14.0 } else { 0.0 };
    let x = title.center().x - text_w / 2.0;
    if has_slideout {
        let resp = ui.interact(title, ui.id().with(("ribbon-title", key, popup)), Sense::click());
        hover_fill(ui, title, &resp, open);
        icons::describe_control(ui, &resp, panel.label, Some(open), false);
        if resp.clicked() {
            slideout::toggle_slideout(&ctx, key, popup);
        }
        icons::paint(
            ui.painter(),
            Rect::from_center_size(pos2(x + galley.size().x + 7.0, bar.center().y), Vec2::splat(10.0)),
            Icon::ChevronDown,
            false,
        );
    } else {
        let resp = ui.interact(title, ui.id().with(("ribbon-title", key, popup)), Sense::hover());
        resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, true, crate::i18n::t(panel.label)));
    }
    ui.painter().galley(pos2(x, bar.center().y - galley.size().y / 2.0), galley, t.text);
    if let Some((cmd, rect)) = launcher {
        let label = panel.launcher_name.or_else(|| widgets::command_label(ui, app, cmd)).unwrap_or(panel.label);
        widgets::launcher(ui, app, rect, key, label, cmd);
    }
}

/// Draw `list` laid out from `origin`, each item in its own id scope named by its stable key.
fn render_items(ui: &mut Ui, app: &mut CadApp, prefix: &str, list: &[Item], compact: bool, origin: Pos2) {
    let (rects, _) = layout::place(ui, prefix, list, compact, origin);
    for (i, rect) in rects {
        let Some(it) = list.get(i) else { continue };
        let key = layout::item_key(prefix, i);
        let builder = UiBuilder::new().max_rect(rect).id_salt(&key).layout(Layout::left_to_right(Align::Center));
        ui.scope_builder(builder, |ui| item(ui, app, &key, it, compact));
    }
}

/// One control of a panel. `key` names it for split-button memory.
fn item(ui: &mut Ui, app: &mut CadApp, key: &str, item: &Item, compact: bool) -> egui::Response {
    let size = |s: Size| widgets::shown_size(s, compact);
    match *item {
        Item::Button { icon, label, cmd, size: s, .. } => match size(s) {
            Size::Large => widgets::large_button(ui, app, icon, label, cmd),
            Size::Small { label: shown } => widgets::small_button(ui, app, icon, label, cmd, shown),
        },
        Item::Toggle { icon, label, cmd, size: s, state, .. } => widgets::toggle(ui, app, icon, label, cmd, size(s), state),
        Item::Split { label, size: s, menu, default, .. } => widgets::split_button(ui, app, key, label, size(s), menu, default),
        Item::Combo { kind, width, .. } => combo(ui, app, kind, width),
        Item::Menu { icon, label, size: s, menu, .. } => widgets::menu_button(ui, app, key, icon, label, size(s), menu),
        Item::Slider { kind, width, .. } => slider(ui, app, kind, width),
    }
}

/// A panel's own icon (its panel button's): the first large item's icon, else the first item's.
pub fn panel_icon(ctx: &egui::Context, key: &str, panel: &Panel) -> Icon {
    let icon_of = |(i, it): (usize, &Item)| match *it {
        Item::Button { icon, .. } | Item::Toggle { icon, .. } | Item::Menu { icon, .. } => Some(icon),
        Item::Split { menu, default, .. } => Some(widgets::split_face(ctx, &layout::item_key(key, i), menu, default).0),
        Item::Combo { .. } | Item::Slider { .. } => None,
    };
    let items = || panel.items.iter().enumerate();
    items().filter(|(_, it)| layout::small_row(it).is_none()).find_map(icon_of).or_else(|| items().find_map(icon_of)).unwrap_or(Icon::Menu)
}

/// A collapsed panel: a full-height button (icon, label, ▾) that shows the panel as a popup.
fn panel_button(app: &mut CadApp, ui: &mut Ui, rect: Rect, key: &str, tab: &Tab, panel: &Panel) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    let resp = ui.interact(rect, ui.id().with(("ribbon-panel-button", key)), Sense::click());
    let open = slideout::popup_open(&ctx, key);
    let face = rect.shrink(PANEL_BUTTON_INSET);
    let fill = if open || resp.is_pointer_button_down_on() {
        t.ribbon_pressed
    } else if resp.hovered() {
        t.ribbon_hover
    } else {
        t.ribbon_collapsed_bg
    };
    ui.painter().rect_filled(face, 2.0, fill);
    let icon_rect = Rect::from_center_size(pos2(rect.center().x, rect.top() + 6.0 + 16.0), Vec2::splat(32.0));
    icons::paint(ui.painter(), icon_rect, panel_icon(&ctx, key, panel), false);
    // The button is wide enough for the label's longest word, so it wraps only at spaces.
    let wrap = rect.width() - 2.0 * layout::PANEL_BUTTON_TEXT_PAD + 1.0;
    let mut job = egui::text::LayoutJob::simple(crate::i18n::t(panel.label).to_string(), crate::theme::ribbon_font(&ctx), t.text, wrap);
    job.wrap.max_rows = 2;
    job.halign = Align::Center;
    let galley = ui.painter().layout_job(job);
    ui.painter().galley(pos2(rect.center().x, icon_rect.bottom() + 4.0), galley, t.text);
    icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.center().x, rect.bottom() - 9.0), Vec2::splat(10.0)), Icon::ChevronDown, false);
    icons::describe_control(ui, &resp, panel.label, None, false);
    if resp.on_hover_text(crate::i18n::t(panel.label)).clicked() {
        slideout::toggle_popup(&ctx, key);
    }
    if slideout::popup_open(&ctx, key) {
        slideout::add_inside(&ctx, rect);
        slideout::panel_popup(app, ui, rect, key, tab, panel);
    }
}

/// The strip of a minimized ribbon: one small button per panel (its icon, or its title with
/// panel titles), each showing its panel as a popup.
fn minimized_strip(app: &mut CadApp, ui: &mut Ui, strip: Rect, tab: &Tab, titles: bool) {
    let t = Tokens::get();
    let ctx = ui.ctx().clone();
    let mut x = strip.left() + 4.0;
    for panel in tab.panels {
        let key = format!("{}/{}", tab.label, panel.label);
        let galley = label_galley(ui, crate::i18n::t(panel.label), t.text);
        let size = if titles { vec2(galley.size().x + 26.0, TAB_H) } else { vec2(46.0, 30.0) };
        let rect = Rect::from_min_size(pos2(x, strip.top() + 2.0), size);
        let resp = ui.interact(rect, ui.id().with(("ribbon-panel-button", &key)), Sense::click());
        let open = slideout::popup_open(&ctx, &key);
        ui.painter().rect_filled(rect, 2.0, t.ribbon_collapsed_bg);
        hover_fill(ui, rect, &resp, open);
        let chevron = Rect::from_center_size(pos2(rect.right() - 9.0, rect.center().y), Vec2::splat(9.0));
        if titles {
            ui.painter().galley(pos2(rect.left() + 8.0, rect.center().y - galley.size().y / 2.0), galley, t.text);
        } else {
            let icon = Rect::from_center_size(pos2(rect.left() + 17.0, rect.center().y), Vec2::splat(24.0));
            icons::paint(ui.painter(), icon, panel_icon(&ctx, &key, panel), false);
        }
        icons::paint(ui.painter(), chevron, Icon::ChevronDown, false);
        icons::describe_control(ui, &resp, panel.label, None, false);
        if resp.on_hover_text(crate::i18n::t(panel.label)).clicked() {
            slideout::toggle_popup(&ctx, &key);
        }
        if slideout::popup_open(&ctx, &key) {
            slideout::add_inside(&ctx, rect);
            slideout::panel_popup(app, ui, rect, &key, tab, panel);
        }
        x = rect.right() + 3.0;
    }
}

/// Render one panel of `tab` at the cursor as the strip shows it (full width), with its
/// slide-out body below it, for the widget tests.
#[doc(hidden)]
pub fn render_panel_for_test(ui: &mut Ui, app: &mut CadApp, tab: &Tab, panel: &Panel) {
    let key = format!("{}/{}", tab.label, panel.label);
    let w = layout::panel_width(ui, tab.label, panel, false);
    let (rect, _) = ui.allocate_exact_size(vec2(w, PANEL_H), Sense::hover());
    show_panel(app, ui, rect, &key, panel, false, false);
    if !panel.slideout.is_empty() {
        let size = layout::slideout_size(ui, &format!("{key}/slideout"), panel.slideout, w);
        let (slide, _) = ui.allocate_exact_size(size, Sense::hover());
        slideout::body(ui, app, slide, &key, panel);
    }
}
