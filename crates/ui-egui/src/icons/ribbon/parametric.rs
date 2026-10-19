//! Parametric tab tools: geometric and dimensional constraints and their display. Shared motifs:
//! a small padlock (a constraint that holds) on the dimensional tools, the constraint bar (a
//! strip of two cells) on the geometric display tools, and an eye for showing and hiding.

use egui::Color32;

use super::{GOLD, P};
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// A small padlock with its body's top-left at (x, y), 6 × 5 grid units.
    fn padlock(&self, x: f32, y: f32) {
        let g = self.tint(GOLD);
        self.fill_rect(x, y, 6.0, 5.0, g);
        self.arc(x + 3.0, y, 2.0, 0.0, 180.0, g);
    }
    /// A constraint bar: a strip of two cells, top-left at (x, y).
    fn cbar(&self, x: f32, y: f32) {
        self.rect(x, y, 12.0, 6.0, self.base);
        self.poly(&[(x + 6.0, y), (x + 6.0, y + 6.0)], false, self.base);
        self.fill_rect(x + 1.5, y + 1.5, 3.0, 3.0, self.acc);
        self.fill_rect(x + 7.5, y + 1.5, 3.0, 3.0, self.acc);
    }
    /// An eye centred at (x, y): `open` draws the pupil, otherwise a slash crosses it.
    fn eye(&self, x: f32, y: f32, open: bool, c: Color32) {
        self.ell(x, y, 5.5, 3.2, 0.0, 0.0, 360.0, c);
        if open {
            self.fill_c(x, y, 1.6, c);
        } else {
            self.poly(&[(x - 5.0, y + 4.0), (x + 5.0, y - 4.0)], false, c);
        }
    }
    /// The visibility badge in the bottom-right corner: an open eye (Show/Hide, Show All) or a
    /// crossed one (Hide All).
    fn visibility(&self, icon: Icon) {
        self.knock(10.5, 13.0, 13.5, 11.0);
        self.eye(17.5, 18.5, !matches!(icon, Icon::BarsHideAll | Icon::DcHideAll), self.pt);
    }
}

/// Whether a display tool acts on every constraint (Show All, Hide All): its motif is doubled.
fn all(icon: Icon) -> bool {
    matches!(icon, Icon::BarsShowAll | Icon::BarsHideAll | Icon::DcShowAll | Icon::DcHideAll)
}

/// Parametric tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        AutoConstrain => {
            pen.poly(&[(3.0, 4.0), (3.0, 16.0), (15.0, 16.0)], false, b);
            pen.fill_rect(1.5, 14.5, 3.0, 3.0, a);
            pen.padlock(4.0, 6.0);
            pen.fill_poly(&[(19.0, 9.0), (14.5, 16.0), (18.0, 16.0)], pen.pt);
            pen.fill_poly(&[(18.0, 15.0), (21.5, 15.0), (16.5, 22.5)], pen.pt);
        }
        Collinear => {
            pen.l(3.0, 20.0, 10.0, 13.5);
            pen.la(14.0, 10.0, 21.0, 3.5);
        }
        Smooth => {
            pen.l(2.0, 18.0, 11.0, 18.0);
            pen.curve([(11.0, 18.0), (17.0, 18.0), (20.0, 12.0), (21.0, 4.0)], a);
            pen.dot(11.0, 18.0);
        }
        Symmetric => {
            pen.dash(12.0, 2.0, 12.0, 22.0, b);
            pen.l(4.0, 6.0, 8.5, 18.0);
            pen.la(20.0, 6.0, 15.5, 18.0);
        }
        BarsShowHide | BarsShowAll | BarsHideAll => {
            pen.cbar(1.5, 2.5);
            if all(icon) {
                pen.cbar(1.5, 10.0);
            }
            pen.visibility(icon);
        }
        DcLinear => {
            dc_dim(pen, 3.0, 21.0, 7.0);
            pen.padlock(9.0, 14.0);
        }
        DcHorizontal => {
            pen.l(3.0, 21.0, 21.0, 14.0);
            pen.la(5.0, 6.0, 19.0, 6.0);
            pen.arrow_tip(3.0, 6.0, -1.0, 0.0, a);
            pen.arrow_tip(21.0, 6.0, 1.0, 0.0, a);
            pen.padlock(9.0, 9.0);
        }
        DcVertical => {
            pen.l(14.0, 3.0, 21.0, 21.0);
            pen.la(5.0, 5.0, 5.0, 19.0);
            pen.arrow_tip(5.0, 3.0, 0.0, -1.0, a);
            pen.arrow_tip(5.0, 21.0, 0.0, 1.0, a);
            pen.padlock(8.0, 10.0);
        }
        DcAligned => {
            pen.l(3.0, 21.0, 17.0, 7.0);
            pen.la(7.0, 13.0, 15.0, 5.0);
            pen.arrow_tip(5.5, 14.5, -1.0, 1.0, a);
            pen.arrow_tip(16.5, 3.5, 1.0, -1.0, a);
            pen.padlock(15.0, 15.0);
        }
        DcRadius => {
            pen.arc(5.0, 19.0, 15.0, 0.0, 90.0, b);
            pen.la(5.0, 19.0, 15.0, 9.0);
            pen.arrow_tip(15.6, 8.4, 1.0, -1.0, a);
            pen.padlock(4.0, 8.0);
        }
        DcDiameter => {
            pen.c(11.0, 11.0, 8.5, b);
            pen.la(5.5, 16.5, 16.5, 5.5);
            pen.arrow_tip(5.0, 17.0, -1.0, 1.0, a);
            pen.arrow_tip(17.0, 5.0, 1.0, -1.0, a);
            pen.padlock(16.0, 17.0);
        }
        DcAngular => {
            pen.l(3.0, 20.0, 21.0, 20.0);
            pen.l(3.0, 20.0, 15.0, 6.0);
            pen.arc(3.0, 20.0, 11.0, 0.0, 49.0, a);
            pen.arrow_tip(14.0, 20.0, 0.0, 1.0, a);
            pen.padlock(15.0, 9.0);
        }
        DcConvert => {
            dim_small(pen);
            pen.arrow(7.0, 13.0, 7.0, 19.5, a);
            pen.arrow(7.0, 19.5, 12.0, 19.5, a);
            pen.padlock(15.0, 16.0);
        }
        DcShowHide | DcShowAll | DcHideAll => {
            dc_dim(pen, 2.0, 14.0, 12.0);
            pen.padlock(2.5, 3.0);
            if all(icon) {
                pen.padlock(10.0, 3.0);
            }
            pen.visibility(icon);
        }
        DynamicConstraints => {
            pen.la(3.0, 20.0, 3.0, 9.0);
            pen.la(21.0, 20.0, 21.0, 9.0);
            pen.dash(5.0, 12.0, 19.0, 12.0, a);
            pen.arrow_tip(3.0, 12.0, -1.0, 0.0, a);
            pen.arrow_tip(21.0, 12.0, 1.0, 0.0, a);
            pen.padlock(9.0, 2.5);
            pen.l(3.0, 20.0, 21.0, 20.0);
        }
        AnnotationalConstraints => {
            pen.l(3.0, 20.0, 21.0, 20.0);
            dc_dim(pen, 3.0, 21.0, 14.0);
            pen.padlock(2.0, 4.0);
            pen.rect(10.0, 3.0, 12.0, 7.0, b);
            pen.text(16.0, 6.6, "d1", 6.0, b);
        }
        DelConstraint => {
            pen.padlock(4.0, 9.0);
            pen.l(2.0, 18.0, 13.0, 18.0);
            pen.thick(14.0, 12.0, 21.0, 19.0, 2.2, pen.pt);
            pen.thick(14.0, 19.0, 21.0, 12.0, 2.2, pen.pt);
        }
        ParametersManager => {
            pen.grid(2.0, 3.0, 20.0, 16.0, 2, 4, b);
            for y in [10.0, 14.0, 18.0] {
                pen.la(13.5, y - 0.8, 16.5, y - 0.8);
                pen.la(13.5, y + 0.8, 16.5, y + 0.8);
            }
            let rows: [P; 3] = [(7.0, 10.0), (7.0, 14.0), (7.0, 18.0)];
            for (x, y) in rows {
                pen.text(x, y, "d", 5.5, b);
            }
        }
        _ => return false,
    }
    true
}

/// A linear dimension from `x0` to `x1` with its line at `y` and short extension lines below.
fn dc_dim(pen: &Pen, x0: f32, x1: f32, y: f32) {
    let a = pen.acc;
    pen.la(x0, y - 2.0, x0, y + 5.0);
    pen.la(x1, y - 2.0, x1, y + 5.0);
    pen.la(x0 + 2.0, y, x1 - 2.0, y);
    pen.arrow_tip(x0, y, -1.0, 0.0, a);
    pen.arrow_tip(x1, y, 1.0, 0.0, a);
}

/// An associative dimension (no padlock) in the top-left corner: what Convert starts from.
fn dim_small(pen: &Pen) {
    dc_dim(pen, 2.0, 14.0, 5.0);
    pen.text(8.0, 2.6, "10", 4.5, pen.base);
}
