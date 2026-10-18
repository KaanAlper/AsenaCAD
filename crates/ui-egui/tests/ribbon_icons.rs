//! The Home ribbon's icons: every icon a ribbon item uses is drawn, the variants are told apart,
//! and (with `CRAFT_UI_VISUAL_DIR` set) an icon sheet at 16 and 32 px for a visual check.

use cadcraft_ui_egui::icons::{self, Icon};
use cadcraft_ui_egui::ribbon::model::{Entry, Item, tabs};
use egui::{Color32, FontId, Rect, pos2, vec2};

/// Every icon the ribbon tabs use (faces, split menu entries, slide-outs), in ribbon order.
fn ribbon_icons() -> Vec<Icon> {
    let mut out = Vec::new();
    let mut push = |icon: Icon| {
        if !out.contains(&icon) {
            out.push(icon);
        }
    };
    for tab in tabs() {
        for panel in tab.panels {
            for item in panel.items.iter().chain(panel.slideout) {
                match item {
                    Item::Button { icon, .. } | Item::Toggle { icon, .. } => push(*icon),
                    Item::Split { menu, .. } => menu.iter().for_each(|e| {
                        if let Entry::Command { icon, .. } = e {
                            push(*icon)
                        }
                    }),
                    Item::Combo { .. } | Item::Slider { .. } => {}
                }
            }
        }
    }
    push(Icon::Launcher);
    push(Icon::Pin);
    push(Icon::Unpin);
    out
}

/// The shapes `icon` paints into a 32 px square, as text (positions, colours, glyphs).
fn shapes_of(icon: Icon) -> String {
    let ctx = egui::Context::default();
    let mut shapes = String::new();
    // A second pass so the fonts are loaded when text is laid out.
    for _ in 0..2 {
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            icons::paint(ui.painter(), Rect::from_min_size(pos2(0.0, 0.0), vec2(32.0, 32.0)), icon, false);
        });
        out.textures_delta.clear();
        shapes = out.shapes.iter().map(|s| format!("{:?}", s.shape)).collect::<Vec<_>>().join("\n");
    }
    shapes
}

#[test]
fn every_ribbon_icon_is_drawn() {
    let icons = ribbon_icons();
    assert!(icons.len() > 150, "the Home tab uses {} icons", icons.len());
    for icon in icons {
        let shapes = shapes_of(icon);
        assert!(!shapes.is_empty(), "{icon:?} paints nothing");
        // The old placeholder was a lone rounded square.
        assert!(shapes.lines().count() > 1 || !shapes.starts_with("Rect"), "{icon:?} is a placeholder square:\n{shapes}");
    }
}

#[test]
fn ribbon_icons_are_distinct() {
    // Icons that share a drawing on purpose (they predate the ribbon and mean the same thing).
    let same = [(Icon::Measure, Icon::Distance), (Icon::MLeader, Icon::AlignLeaders), (Icon::Layers, Icon::LayerProps)];
    let icons = ribbon_icons();
    let drawn: Vec<(Icon, String)> = icons.iter().map(|i| (*i, shapes_of(*i))).collect();
    for (i, (a, sa)) in drawn.iter().enumerate() {
        for (b, sb) in &drawn[i + 1..] {
            let allowed = same.iter().any(|&(x, y)| (x, y) == (*a, *b) || (y, x) == (*a, *b));
            assert!(allowed || sa != sb, "{a:?} and {b:?} look the same");
        }
    }
}

/// `CRAFT_UI_VISUAL_DIR=/dir cargo test -p cadcraft-ui-egui --test ribbon_icons icon_sheet`
/// renders every ribbon icon at 16 and 32 px with its name, in both themes.
#[test]
fn icon_sheet() {
    use egui_kittest::Harness;
    let Some(directory) = std::env::var_os("CRAFT_UI_VISUAL_DIR").map(std::path::PathBuf::from) else { return };
    std::fs::create_dir_all(&directory).unwrap();
    let icons = ribbon_icons();
    const COLS: usize = 4;
    const CELL: egui::Vec2 = egui::Vec2 { x: 190.0, y: 40.0 };
    for (theme, name) in [(egui::Theme::Dark, "dark"), (egui::Theme::Light, "light")] {
        for (page, chunk) in icons.chunks(COLS * 12).enumerate() {
            let chunk = chunk.to_vec();
            let rows = chunk.len().div_ceil(COLS);
            let size = vec2(CELL.x * COLS as f32, CELL.y * rows as f32);
            let mut h = Harness::builder().with_size(size).wgpu().build_ui(move |ui| {
                cadcraft_ui_egui::theme::set_active(theme);
                let t = cadcraft_ui_egui::theme::Tokens::get();
                ui.painter().rect_filled(Rect::from_min_size(pos2(0.0, 0.0), size), 0.0, t.panel);
                for (k, icon) in chunk.iter().enumerate() {
                    let o = pos2((k % COLS) as f32 * CELL.x, (k / COLS) as f32 * CELL.y);
                    icons::paint(ui.painter(), Rect::from_min_size(o + vec2(4.0, 12.0), vec2(16.0, 16.0)), *icon, false);
                    icons::paint(ui.painter(), Rect::from_min_size(o + vec2(24.0, 4.0), vec2(32.0, 32.0)), *icon, false);
                    let text = if t.panel.r() > 128 { Color32::BLACK } else { Color32::WHITE };
                    ui.painter().text(o + vec2(60.0, 20.0), egui::Align2::LEFT_CENTER, format!("{icon:?}"), FontId::proportional(11.0), text);
                }
            });
            h.run_steps(2);
            h.render().unwrap().save(directory.join(format!("icons-{name}-{page}.png"))).unwrap();
        }
    }
}
