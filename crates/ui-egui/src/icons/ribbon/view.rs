//! View tab tools: the drawing area's navigation aids, named views, model viewport
//! configurations, palettes and the window arrangement. Shared motifs: a viewport frame whose
//! tiles are split by blue lines for the viewport configurations, and a small window with a title
//! bar for the palettes and window tools.

use egui::Color32;

use super::{GOLD, GREEN, ICE, P, YELLOW};
use crate::icons::{Icon, Pen};

/// The viewport frame of the configuration icons: left, top, right, bottom.
const VP: (f32, f32, f32, f32) = (3.0, 4.0, 21.0, 20.0);

impl Pen<'_> {
    /// A window with a title bar, top-left at (x, y).
    fn window(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.knock(x, y, w, h);
        self.rect(x, y, w, h, c);
        self.fill_rect(x, y, w, 2.5, c);
    }
    /// The viewport frame with tile dividers: `v` vertical lines (x, y0, y1) and `h` horizontal
    /// ones (y, x0, x1), in the frame's 0–1 coordinates.
    fn tiles(&self, v: &[(f32, f32, f32)], h: &[(f32, f32, f32)]) {
        let (l, t, r, b) = VP;
        let (w, ht) = (r - l, b - t);
        self.rect(l, t, w, ht, self.base);
        for (x, y0, y1) in v {
            self.la(l + w * x, t + ht * y0, l + w * x, t + ht * y1);
        }
        for (y, x0, x1) in h {
            self.la(l + w * x0, t + ht * y, l + w * x1, t + ht * y);
        }
    }
    /// A shaded sphere centred at (x, y): the materials motif.
    fn sphere(&self, x: f32, y: f32, r: f32) {
        self.fill_c(x, y, r, self.tint(ICE).gamma_multiply(0.35));
        self.c(x, y, r, self.base);
        self.fill_c(x - r * 0.35, y - r * 0.35, r * 0.25, self.base);
    }
}

/// View tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        UcsIcon => {
            pen.arrow(4.0, 20.0, 21.0, 20.0, a);
            pen.arrow(4.0, 20.0, 4.0, 3.0, pen.tint(GREEN));
            pen.rect(4.0, 16.0, 4.0, 4.0, b);
            pen.text(16.0, 15.5, "X", 6.0, b);
            pen.text(9.5, 6.0, "Y", 6.0, b);
        }
        ViewCube => {
            let top: [P; 4] = [(12.0, 3.0), (19.0, 6.5), (12.0, 10.0), (5.0, 6.5)];
            pen.fill_poly(&top, a.gamma_multiply(0.35));
            pen.poly(&top, true, b);
            pen.poly(&[(5.0, 6.5), (5.0, 13.5), (12.0, 17.0), (19.0, 13.5), (19.0, 6.5)], false, b);
            pen.l(12.0, 10.0, 12.0, 17.0);
            pen.ell(12.0, 18.0, 10.0, 3.5, 0.0, 200.0, 340.0, a);
        }
        NavBar => {
            pen.rect(8.0, 2.0, 8.0, 20.0, b);
            pen.c(12.0, 6.0, 2.0, a);
            pen.la(10.0, 11.0, 14.0, 11.0);
            pen.la(12.0, 9.0, 12.0, 13.0);
            pen.c(11.5, 17.0, 1.8, a);
            pen.la(12.8, 18.3, 14.0, 19.5);
        }
        NewView => {
            view_frame(pen);
            pen.thick(18.0, 13.0, 18.0, 22.0, 2.0, pen.tint(GREEN));
            pen.thick(13.5, 17.5, 22.5, 17.5, 2.0, pen.tint(GREEN));
        }
        ViewManager => {
            view_frame(pen);
            pen.knock(12.0, 12.0, 12.0, 12.0);
            pen.rect(13.0, 13.0, 10.0, 9.0, b);
            for y in [16.0, 19.0] {
                pen.la(15.0, y, 21.0, y);
            }
        }
        VpSingle => pen.tiles(&[], &[]),
        VpTwoV => pen.tiles(&[(0.5, 0.0, 1.0)], &[]),
        VpTwoH => pen.tiles(&[], &[(0.5, 0.0, 1.0)]),
        VpThreeR => pen.tiles(&[(0.5, 0.0, 1.0)], &[(0.5, 0.0, 0.5)]),
        VpThreeL => pen.tiles(&[(0.5, 0.0, 1.0)], &[(0.5, 0.5, 1.0)]),
        VpThreeA => pen.tiles(&[(0.5, 0.5, 1.0)], &[(0.5, 0.0, 1.0)]),
        VpThreeB => pen.tiles(&[(0.5, 0.0, 0.5)], &[(0.5, 0.0, 1.0)]),
        VpThreeV => pen.tiles(&[(1.0 / 3.0, 0.0, 1.0), (2.0 / 3.0, 0.0, 1.0)], &[]),
        VpThreeH => pen.tiles(&[], &[(1.0 / 3.0, 0.0, 1.0), (2.0 / 3.0, 0.0, 1.0)]),
        VpFourE => pen.tiles(&[(0.5, 0.0, 1.0)], &[(0.5, 0.0, 1.0)]),
        VpFourR => pen.tiles(&[(0.4, 0.0, 1.0)], &[(1.0 / 3.0, 0.0, 0.4), (2.0 / 3.0, 0.0, 0.4)]),
        VpFourL => pen.tiles(&[(0.6, 0.0, 1.0)], &[(1.0 / 3.0, 0.6, 1.0), (2.0 / 3.0, 0.6, 1.0)]),
        VpNamed => {
            pen.rect(2.0, 3.0, 13.0, 11.0, b);
            pen.la(8.5, 3.0, 8.5, 14.0);
            pen.knock(9.0, 12.0, 15.0, 12.0);
            pen.poly(&[(10.0, 15.0), (19.0, 15.0), (22.5, 18.5), (19.0, 22.0), (10.0, 22.0)], true, a);
            pen.fill_c(13.0, 18.5, 1.2, a);
        }
        VpJoin => {
            pen.rect(2.0, 5.0, 20.0, 14.0, b);
            pen.dash(12.0, 5.0, 12.0, 19.0, b);
            pen.arrow(4.0, 12.0, 10.5, 12.0, a);
            pen.arrow(20.0, 12.0, 13.5, 12.0, a);
        }
        VpToggle => {
            pen.rect(2.0, 2.0, 11.0, 9.0, b);
            pen.rect(11.0, 13.0, 11.0, 9.0, b);
            pen.la(16.5, 13.0, 16.5, 22.0);
            pen.la(11.0, 17.5, 22.0, 17.5);
            pen.arrow(5.0, 14.0, 5.0, 20.0, pen.tint(GREEN));
            pen.arrow(19.0, 10.0, 19.0, 4.0, pen.tint(GREEN));
        }
        DwgCompare => {
            pen.page(2.0, 2.0, 12.0, 16.0, b);
            pen.page(10.0, 6.0, 12.0, 16.0, b);
            pen.knock(10.5, 6.5, 6.5, 2.5);
            pen.rect(4.0, 7.0, 4.0, 4.0, pen.pt);
            pen.rect(13.0, 13.0, 5.0, 4.0, pen.tint(GREEN));
            pen.dash(12.0, 1.0, 12.0, 23.0, a);
        }
        ActivityInsights => {
            pen.clipboard();
            for y in [9.0, 12.5, 16.0] {
                pen.l(6.0, y, 11.0, y);
            }
            pen.fill_poly(&[(20.0, 9.0), (15.5, 16.0), (19.0, 16.0)], pen.tint(YELLOW));
            pen.fill_poly(&[(19.0, 15.0), (22.5, 15.0), (17.5, 22.5)], pen.tint(YELLOW));
        }
        ToolPalettes => {
            pen.rect(4.0, 2.0, 14.0, 20.0, b);
            for (x, y) in [(6.0, 5.0), (11.5, 5.0), (6.0, 10.5), (11.5, 10.5), (6.0, 16.0), (11.5, 16.0)] {
                pen.fill_rect(x, y, 4.0, 4.0, b);
            }
            pen.fill_rect(18.0, 4.0, 3.0, 7.0, a);
        }
        CountPalette => {
            pen.block(2.0, 3.0, 0.65, b);
            for (i, y) in [6.0, 12.0, 18.0].into_iter().enumerate() {
                pen.text(16.0, y, ["1", "2", "3"][i], 6.0, a);
                pen.l(18.5, y, 22.0, y);
            }
        }
        CommandMacros => {
            pen.page(3.0, 3.0, 14.0, 18.0, b);
            pen.fill_poly(&[(7.5, 8.0), (7.5, 16.0), (13.5, 12.0)], a);
            // A four-pointed spark: two thin diamonds.
            pen.fill_poly(&[(19.0, 1.0), (20.3, 5.5), (19.0, 10.0), (17.7, 5.5)], pen.tint(GOLD));
            pen.fill_poly(&[(14.5, 5.5), (19.0, 4.2), (23.5, 5.5), (19.0, 6.8)], pen.tint(GOLD));
        }
        SheetSet => {
            for (x, y) in [(8.0, 2.0), (5.0, 5.0), (2.0, 8.0)] {
                pen.knock(x, y, 13.0, 14.0);
                pen.rect(x, y, 13.0, 14.0, b);
            }
            pen.rect(4.0, 15.5, 9.0, 4.5, a);
        }
        CommandLine => {
            pen.rect(2.0, 5.0, 20.0, 14.0, b);
            pen.poly(&[(5.0, 9.0), (8.5, 12.0), (5.0, 15.0)], false, a);
            pen.la(10.0, 15.0, 15.0, 15.0);
        }
        MarkupSet => {
            pen.fill_rect(3.0, 3.0, 13.0, 13.0, pen.tint(YELLOW).gamma_multiply(0.6));
            pen.rect(3.0, 3.0, 13.0, 13.0, b);
            pen.cloud(&[(12.0, 12.0), (22.0, 12.0), (22.0, 22.0), (12.0, 22.0)], true, 3.3, pen.pt);
        }
        DesignCenter => {
            pen.c(12.0, 12.0, 3.0, a);
            for (x, y) in [(2.0, 2.0), (17.0, 2.0), (2.0, 17.0), (17.0, 17.0)] {
                pen.rect(x, y, 5.0, 5.0, b);
            }
            for (x, y) in [(7.0, 7.0), (17.0, 7.0), (7.0, 17.0), (17.0, 17.0)] {
                let (cx, cy) = (12.0 + (x - 12.0) * 0.45, 12.0 + (y - 12.0) * 0.45);
                pen.poly(&[(cx, cy), (x, y)], false, a);
            }
        }
        XrefPalette => {
            pen.page(2.0, 2.0, 13.0, 17.0, b);
            pen.dash_rect(10.0, 10.0, 12.0, 12.0, a);
            pen.arrow(6.0, 14.0, 13.5, 17.0, a);
        }
        TextWindow => {
            pen.window(2.0, 3.0, 20.0, 18.0, b);
            for (y, w) in [(9.0, 14.0), (12.5, 10.0), (16.0, 12.0)] {
                pen.l(5.0, y, 5.0 + w, y);
            }
        }
        MaterialsBrowser => {
            pen.sphere(9.0, 10.0, 7.0);
            for y in [15.0, 18.5, 22.0] {
                pen.la(15.0, y, 22.0, y);
            }
        }
        MaterialsEditor => {
            pen.sphere(10.0, 10.0, 7.5);
            pen.pencil();
        }
        VisualStyles => {
            let top: [P; 4] = [(12.0, 3.0), (20.0, 7.0), (12.0, 11.0), (4.0, 7.0)];
            pen.fill_poly(&top, b.gamma_multiply(0.6));
            pen.fill_poly(&[(4.0, 7.0), (12.0, 11.0), (12.0, 21.0), (4.0, 17.0)], a.gamma_multiply(0.5));
            pen.poly(&[(4.0, 7.0), (4.0, 17.0), (12.0, 21.0), (20.0, 17.0), (20.0, 7.0)], false, b);
            pen.poly(&top, true, b);
            pen.l(12.0, 11.0, 12.0, 21.0);
        }
        RenderSettings => {
            pen.sphere(8.0, 9.0, 6.0);
            for (y, k) in [(16.0, 17.0), (20.0, 13.0)] {
                pen.l(3.0, y, 21.0, y);
                pen.fill_c(k, y, 1.8, pen.pt);
            }
        }
        LightsInModel => {
            let c = pen.tint(YELLOW);
            pen.c(12.0, 9.0, 6.0, c);
            pen.poly(&[(9.0, 14.0), (9.5, 17.0), (14.5, 17.0), (15.0, 14.0)], false, c);
            pen.l(9.5, 19.0, 14.5, 19.0);
            pen.l(10.5, 21.0, 13.5, 21.0);
        }
        SunProperties => {
            let c = pen.tint(GOLD);
            pen.fill_c(10.0, 10.0, 4.0, c);
            for k in 0..8 {
                let t = (k as f32 * 45.0).to_radians();
                pen.poly(&[(10.0 + 6.0 * t.cos(), 10.0 + 6.0 * t.sin()), (10.0 + 8.5 * t.cos(), 10.0 + 8.5 * t.sin())], false, c);
            }
            pen.knock(14.0, 14.0, 10.0, 10.0);
            for y in [16.0, 19.5, 23.0] {
                pen.la(16.0, y - 0.5, 23.0, y - 0.5);
            }
        }
        SwitchWindows => {
            pen.window(7.0, 2.0, 15.0, 11.0, b);
            pen.window(2.0, 11.0, 15.0, 11.0, b);
            pen.arrow(19.0, 15.0, 19.0, 21.0, a);
        }
        FileTabs => {
            pen.poly(&[(2.0, 20.0), (2.0, 5.0), (10.0, 5.0), (11.0, 8.0), (22.0, 8.0), (22.0, 20.0)], true, b);
            pen.fill_poly(&[(3.0, 6.0), (9.5, 6.0), (10.3, 8.5), (3.0, 8.5)], a);
            pen.dot(18.0, 13.0);
        }
        LayoutTabs => {
            pen.poly(&[(2.0, 4.0), (22.0, 4.0), (22.0, 16.0), (11.0, 16.0), (10.0, 19.0), (2.0, 19.0)], true, b);
            pen.fill_poly(&[(3.0, 16.5), (9.8, 16.5), (9.2, 18.5), (3.0, 18.5)], a);
            pen.rect(6.0, 7.0, 12.0, 6.0, b);
        }
        TileHorizontal => {
            pen.window(2.0, 2.0, 20.0, 9.0, b);
            pen.window(2.0, 13.0, 20.0, 9.0, b);
        }
        TileVertical => {
            pen.window(2.0, 2.0, 9.0, 20.0, b);
            pen.window(13.0, 2.0, 9.0, 20.0, b);
        }
        Cascade => {
            for k in [0.0, 4.0, 8.0] {
                pen.window(2.0 + k, 2.0 + k, 12.0, 12.0, b);
            }
        }
        _ => return false,
    }
    true
}

/// A view: a frame with corner brackets around a small shape (the named view tools).
fn view_frame(pen: &Pen) {
    let a = pen.acc;
    for (x, y, dx, dy) in [(2.0, 2.0, 1.0, 1.0), (18.0, 2.0, -1.0, 1.0), (2.0, 16.0, 1.0, -1.0), (18.0, 16.0, -1.0, -1.0)] {
        pen.poly(&[(x + 4.0 * dx, y), (x, y), (x, y + 4.0 * dy)], false, a);
    }
    pen.rect(6.5, 6.5, 7.0, 5.0, pen.base);
}
