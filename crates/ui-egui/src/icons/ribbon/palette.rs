//! Palette tools: the Layer Properties Manager's toolbar (layer filters, layer states, new layers,
//! refresh) and the palettes' title strip buttons. Same motifs as the ribbon's layer tools: the
//! layer stack with a badge at (18, 17).

use super::{GREEN, ICE, YELLOW};
use crate::icons::{Icon, Pen};

/// Paint a palette icon; false for the icons of other families.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    use Icon::*;
    let (b, a) = (pen.base, pen.acc);
    let (x, y) = (18.0, 17.0);
    match icon {
        LayerNew | LayerNewVpFrozen => {
            pen.layers(b);
            if icon == LayerNewVpFrozen {
                // A viewport frame with frost in it.
                pen.knock(x - 6.0, y - 4.5, 11.0, 9.0);
                pen.rect(x - 6.0, y - 4.5, 11.0, 9.0, b);
                let c = pen.tint(ICE);
                pen.poly(&[(x - 3.5, y), (x + 2.5, y)], false, c);
                pen.poly(&[(x - 0.5, y - 3.0), (x - 0.5, y + 3.0)], false, c);
            }
            // The new-item spark at the top right.
            let c = pen.tint(YELLOW);
            let (sx, sy) = if icon == LayerNew { (x, y) } else { (20.5, 6.0) };
            for (dx, dy) in [(1.0, 0.0), (0.0, 1.0), (0.7, 0.7), (0.7, -0.7)] {
                pen.poly(&[(sx - 3.0 * dx, sy - 3.0 * dy), (sx + 3.0 * dx, sy + 3.0 * dy)], false, c);
            }
        }
        PropertyFilter => {
            // A funnel over the layer stack.
            pen.layers(b);
            pen.knock(x - 5.0, y - 5.0, 10.0, 10.0);
            pen.poly(
                &[(x - 5.0, y - 4.0), (x + 5.0, y - 4.0), (x + 1.0, y + 0.5), (x + 1.0, y + 5.0), (x - 1.0, y + 4.0), (x - 1.0, y + 0.5)],
                true,
                a,
            );
        }
        GroupFilter => {
            // A folder.
            pen.poly(&[(3.0, 7.0), (9.0, 7.0), (11.0, 9.0), (21.0, 9.0), (21.0, 19.0), (3.0, 19.0)], true, b);
            pen.la(3.0, 11.0, 21.0, 11.0);
        }
        LayerStates => {
            // The stack with a stored-state sheet beside it.
            pen.layers(b);
            pen.knock(x - 5.0, y - 5.0, 10.0, 11.0);
            pen.rect(x - 4.0, y - 4.0, 8.0, 9.0, a);
            pen.la(x - 2.0, y - 1.0, x + 2.0, y - 1.0);
            pen.la(x - 2.0, y + 2.0, x + 2.0, y + 2.0);
        }
        Refresh => {
            let c = pen.tint(GREEN);
            pen.arc(12.0, 12.0, 7.0, 20.0, 160.0, c);
            pen.arrow_tip(5.4, 9.6, -0.3, 1.0, c);
            pen.arc(12.0, 12.0, 7.0, 200.0, 340.0, c);
            pen.arrow_tip(18.6, 14.4, 0.3, -1.0, c);
        }
        AutoHide => {
            // A bar the palette folds into, and the fold direction.
            pen.l(8.0, 4.0, 8.0, 20.0);
            pen.tri([(18.0, 7.0), (18.0, 17.0), (11.0, 12.0)], b);
        }
        DoubleChevronLeft | DoubleChevronRight => {
            let s = if icon == DoubleChevronLeft { 1.0 } else { -1.0 };
            for x0 in [9.0, 15.0] {
                pen.poly(&[(x0 + 3.0 * s, 6.0), (x0 - 3.0 * s, 12.0), (x0 + 3.0 * s, 18.0)], false, b);
            }
        }
        _ => return false,
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::Color32;

    /// Every palette icon is drawn by this module.
    #[test]
    fn palette_icons_draw() {
        let ctx = egui::Context::default();
        let mut out = ctx.run_ui(Default::default(), |ui| {
            let painter = ui.painter().clone();
            let pen = Pen {
                p: &painter,
                r: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(24.0, 24.0)),
                w: 1.0,
                base: Color32::WHITE,
                acc: Color32::LIGHT_BLUE,
                pt: Color32::ORANGE,
                dim: false,
            };
            for icon in [
                Icon::LayerNew,
                Icon::LayerNewVpFrozen,
                Icon::PropertyFilter,
                Icon::GroupFilter,
                Icon::LayerStates,
                Icon::Refresh,
                Icon::AutoHide,
                Icon::DoubleChevronLeft,
                Icon::DoubleChevronRight,
            ] {
                assert!(paint(&pen, icon), "{icon:?}");
            }
            assert!(!paint(&pen, Icon::Line));
        });
        out.textures_delta.clear();
    }
}
