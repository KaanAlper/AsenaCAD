//! AutoCAD's ribbon: tabs of panels of command buttons, built from static tables.

mod bind;
pub mod home;
pub mod model;
pub mod widgets;

pub use bind::{combo, slider};

use model::{Item, Panel, Size};

use crate::CadApp;

/// One control of a panel. `key` names it (panel and position) for split-button memory.
fn item(ui: &mut egui::Ui, app: &mut CadApp, key: &str, item: &Item) -> egui::Response {
    match *item {
        Item::Button { icon, label, cmd, size: Size::Large, .. } => widgets::large_button(ui, app, icon, label, cmd),
        Item::Button { icon, label, cmd, size: Size::Small { label: shown }, .. } => widgets::small_button(ui, app, icon, label, cmd, shown),
        Item::Toggle { icon, label, cmd, size, state, .. } => widgets::toggle(ui, app, icon, label, cmd, size, state),
        Item::Split { label, size, menu, default, .. } => widgets::split_button(ui, app, key, label, size, menu, default),
        Item::Combo { kind, width, .. } => combo(ui, app, kind, width),
        Item::Slider { kind, width, .. } => slider(ui, app, kind, width),
    }
}

/// Small items' row (1-based), or None for a large item.
fn small_row(item: &Item) -> Option<u8> {
    match *item {
        Item::Button { size: Size::Large, .. } | Item::Toggle { size: Size::Large, .. } | Item::Split { size: Size::Large, .. } => None,
        Item::Button { row, .. } | Item::Toggle { row, .. } | Item::Split { row, .. } | Item::Combo { row, .. } | Item::Slider { row, .. } => {
            Some(row)
        }
    }
}

/// Lay `items` out left to right following the model's rule: a large item is a column of its
/// own; consecutive small items whose row never decreases form a block of rows.
fn items(ui: &mut egui::Ui, app: &mut CadApp, key: &str, list: &[Item]) {
    let mut blocks: Vec<Vec<(usize, Option<u8>)>> = Vec::new();
    for (i, it) in list.iter().enumerate() {
        let row = small_row(it);
        let starts_block = match (row, blocks.last().and_then(|b| b.last())) {
            (None, _) | (_, None) | (_, Some((_, None))) => true,
            (Some(r), Some((_, Some(prev)))) => r < *prev,
        };
        if starts_block {
            blocks.push(Vec::new());
        }
        if let Some(b) = blocks.last_mut() {
            b.push((i, row));
        }
    }
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(2.0, 2.0);
        for block in &blocks {
            if let [(i, None)] = block.as_slice() {
                if let Some(it) = list.get(*i) {
                    item(ui, app, &format!("{key}/{i}"), it);
                }
                continue;
            }
            let rows = block.iter().filter_map(|(_, r)| *r).max().unwrap_or(1);
            ui.vertical(|ui| {
                for r in 1..=rows {
                    ui.horizontal(|ui| {
                        ui.set_min_height(widgets::SMALL_H);
                        for (i, _) in block.iter().filter(|(_, row)| *row == Some(r)) {
                            if let Some(it) = list.get(*i) {
                                item(ui, app, &format!("{key}/{i}"), it);
                            }
                        }
                    });
                }
            });
        }
    });
}

/// Render one panel's items, then its slide-out items and its dialog launcher, with the ribbon
/// widgets (for tests until the panel strip exists).
#[doc(hidden)]
pub fn render_panel_for_test(ui: &mut egui::Ui, app: &mut CadApp, panel: &Panel) {
    ui.vertical(|ui| {
        items(ui, app, panel.label, panel.items);
        if !panel.slideout.is_empty() {
            ui.add_space(6.0);
            items(ui, app, &format!("{}/slideout", panel.label), panel.slideout);
        }
        if let Some(cmd) = panel.launcher {
            let (rect, _) = ui.allocate_exact_size(widgets::LAUNCHER, egui::Sense::hover());
            let label = cadcraft_engine::find_command(cmd).map_or(panel.label, |c| c.label);
            widgets::launcher(ui, app, rect, label, cmd);
        }
    });
}
