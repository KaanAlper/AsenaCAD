//! Manage tab tools: the action recorder, interface customization, applications and macros,
//! CAD standards and drawing cleanup. Shared motifs: a page for files, a small window with a
//! title bar for the editors, an open book for the standards and a broom for purging.

use super::{GOLD, GREEN};
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// A play triangle with its left edge at x, centred on y.
    fn play(&self, x: f32, y: f32, h: f32) {
        self.fill_poly(&[(x, y - h / 2.0), (x + h * 0.85, y), (x, y + h / 2.0)], self.acc);
    }
    /// An editor window with a title bar, top-left at (x, y).
    fn editor(&self, x: f32, y: f32, w: f32, h: f32) {
        self.rect(x, y, w, h, self.base);
        self.fill_rect(x, y, w, 3.0, self.base);
    }
    /// An open book (the standards file), 18 wide, top at y.
    fn book(&self, y: f32) {
        self.poly(&[(2.0, y + 1.0), (11.0, y + 3.0), (20.0, y + 1.0), (20.0, y + 13.0), (11.0, y + 15.0), (2.0, y + 13.0)], true, self.base);
        self.poly(&[(11.0, y + 3.0), (11.0, y + 15.0)], false, self.base);
    }
    /// A broom leaning right, its bristles' bottom at (x, y).
    fn broom(&self, x: f32, y: f32) {
        let g = self.tint(GOLD);
        self.thick(x + 5.0, y - 19.0, x + 1.0, y - 7.0, 1.6, self.base);
        self.fill_poly(&[(x - 2.0, y - 7.5), (x + 4.0, y - 6.0), (x + 4.5, y), (x - 5.5, y)], g);
    }
    /// A green check mark from (x, y), `s` = 1 is 8 units wide.
    fn check(&self, x: f32, y: f32, s: f32) {
        let g = self.tint(GREEN);
        self.thick(x, y, x + 3.0 * s, y + 3.0 * s, 2.0, g);
        self.thick(x + 3.0 * s, y + 3.0 * s, x + 8.0 * s, y - 3.0 * s, 2.0, g);
    }
}

/// Manage tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        Record => {
            pen.c(12.0, 12.0, 9.0, b);
            pen.fill_c(12.0, 12.0, 6.0, pen.pt);
        }
        InsertMessage => {
            pen.poly(&[(3.0, 4.0), (21.0, 4.0), (21.0, 15.0), (10.0, 15.0), (6.0, 20.0), (6.5, 15.0), (3.0, 15.0)], true, b);
            pen.la(6.5, 8.0, 17.5, 8.0);
            pen.la(6.5, 11.5, 14.0, 11.5);
        }
        InsertBasePoint => {
            pen.l(2.0, 14.0, 22.0, 14.0);
            pen.l(10.0, 4.0, 10.0, 22.0);
            pen.first(10.0, 14.0);
            pen.thick(19.0, 2.0, 19.0, 10.0, 2.0, pen.tint(GREEN));
            pen.thick(15.0, 6.0, 23.0, 6.0, 2.0, pen.tint(GREEN));
        }
        PauseInput => {
            pen.pointer(3.0, 4.0);
            pen.fill_rect(13.0, 10.0, 3.0, 11.0, a);
            pen.fill_rect(18.5, 10.0, 3.0, 11.0, a);
        }
        PlayMacro => pen.play(6.0, 12.0, 15.0),
        ActPreferences => {
            for (y, on) in [(5.0, true), (12.0, false), (19.0, true)] {
                pen.rect(2.5, y - 2.5, 5.0, 5.0, b);
                if on {
                    pen.fill_rect(3.8, y - 1.2, 2.4, 2.4, a);
                }
                pen.l(10.0, y, 21.0, y);
            }
        }
        ActManager => {
            pen.poly(&[(2.0, 6.0), (2.0, 20.0), (22.0, 20.0), (22.0, 8.0), (11.0, 8.0), (9.0, 6.0)], true, b);
            pen.play(9.5, 14.0, 7.0);
        }
        CuiEditor => {
            pen.editor(2.0, 3.0, 19.0, 15.0);
            for (x, y) in [(5.0, 9.0), (10.0, 9.0), (15.0, 9.0), (5.0, 13.0)] {
                pen.fill_rect(x, y, 3.0, 2.5, a);
            }
            pen.pencil();
        }
        CustomizePalettes => {
            pen.rect(3.0, 2.0, 12.0, 18.0, b);
            for (x, y) in [(5.0, 4.5), (10.0, 4.5), (5.0, 9.5), (10.0, 9.5)] {
                pen.fill_rect(x, y, 3.0, 3.0, b);
            }
            pen.fill_rect(15.0, 4.0, 2.5, 6.0, a);
            pen.pencil();
        }
        CuiImport | CuiExport => {
            pen.page(9.0, 2.0, 13.0, 18.0, b);
            pen.editor(12.0, 9.0, 7.0, 6.0);
            if icon == CuiImport {
                pen.arrow(1.0, 17.0, 10.5, 17.0, a);
            } else {
                pen.arrow(13.0, 21.0, 22.5, 21.0, a);
                pen.knock(1.0, 15.0, 7.0, 7.0);
            }
        }
        EditAliases => {
            pen.text(6.0, 8.0, "L", 9.0, b);
            pen.arrow(9.0, 8.0, 14.0, 8.0, a);
            pen.text(18.5, 8.0, "LINE", 5.5, b);
            pen.pencil();
        }
        LoadApp => {
            pen.page(2.0, 2.0, 13.0, 17.0, b);
            pen.c(16.5, 16.5, 4.0, a);
            for k in 0..6 {
                let t = (k as f32 * 60.0).to_radians();
                pen.thick(16.5 + 4.0 * t.cos(), 16.5 + 4.0 * t.sin(), 16.5 + 6.0 * t.cos(), 16.5 + 6.0 * t.sin(), 1.8, a);
            }
        }
        RunScript => {
            pen.page(2.0, 2.0, 14.0, 19.0, b);
            for y in [7.0, 10.5, 14.0] {
                pen.l(5.0, y, 12.0, y);
            }
            pen.knock(13.0, 11.0, 11.0, 12.0);
            pen.play(15.0, 17.0, 9.0);
        }
        VbaEditor | VlispEditor => {
            pen.editor(2.0, 3.0, 20.0, 17.0);
            pen.text(12.0, 13.0, if icon == VbaEditor { "{ }" } else { "( )" }, 9.0, a);
        }
        VbaRun => {
            pen.editor(2.0, 3.0, 20.0, 17.0);
            pen.play(9.0, 13.0, 8.0);
        }
        VbaLoad => {
            pen.poly(&[(2.0, 8.0), (2.0, 21.0), (22.0, 21.0), (22.0, 10.0), (11.0, 10.0), (9.0, 8.0)], true, b);
            pen.arrow(12.0, 18.0, 12.0, 2.0, a);
        }
        VbaManager => {
            for (y, c) in [(3.0, b), (10.0, a), (17.0, b)] {
                pen.rect(2.0, y, 6.0, 5.0, c);
                pen.poly(&[(10.0, y + 2.5), (22.0, y + 2.5)], false, c);
            }
        }
        LayerTranslator => {
            pen.layers(b);
            pen.knock(13.0, 13.0, 11.0, 11.0);
            pen.arrow(14.0, 18.0, 22.5, 18.0, a);
            pen.poly(&[(14.0, 22.0), (18.0, 20.5), (22.0, 22.0)], false, a);
        }
        CheckStandards => {
            pen.book(4.0);
            pen.knock(12.0, 11.0, 12.0, 10.0);
            pen.check(13.0, 16.0, 1.1);
        }
        ConfigureStandards => {
            pen.book(4.0);
            pen.pencil();
        }
        FindNonPurgeable => {
            pen.broom(8.0, 21.0);
            pen.knock(13.0, 10.0, 11.0, 13.0);
            pen.c(17.0, 15.0, 3.8, b);
            pen.thick(19.7, 17.7, 22.5, 20.5, 2.0, b);
        }
        Purge => pen.broom(11.0, 22.0),
        Audit => {
            pen.page(3.0, 2.0, 15.0, 19.0, b);
            for y in [7.0, 10.5] {
                pen.l(6.0, y, 14.0, y);
            }
            pen.check(7.0, 16.0, 1.0);
        }
        PerformanceAnalyzer => {
            pen.arc(11.0, 13.0, 9.0, 0.0, 180.0, b);
            pen.l(2.0, 13.0, 20.0, 13.0);
            pen.thick(11.0, 13.0, 16.0, 7.0, 1.8, pen.pt);
            for (k, h) in [(0.0, 4.0), (1.0, 7.0), (2.0, 5.5)] {
                pen.fill_rect(14.0 + k * 3.0, 22.0 - h, 2.2, h, a);
            }
        }
        _ => return false,
    }
    true
}
