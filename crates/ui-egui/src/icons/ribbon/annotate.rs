//! Annotate tab tools: text checks and alignment, the dimension edit tools, centerlines and
//! annotation scaling. Shared motifs: the Home tab's small linear dimension for the dimension
//! tools, and the annotative triangle (as on the status bar's annotation tools) with a badge for
//! the scaling tools.

use super::panels::dim;
use super::{GREEN, P};
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// The annotative triangle of the scaling tools; badges go at the right.
    fn anno(&self) {
        self.poly(&[(8.0, 3.0), (14.5, 17.0), (1.5, 17.0)], true, self.base);
        self.l(8.0, 8.0, 8.0, 13.0);
    }
    /// A circular arrow (update, synchronize) around (x, y).
    fn refresh(&self, x: f32, y: f32, r: f32) {
        let g = self.tint(GREEN);
        self.arc(x, y, r, 30.0, 320.0, g);
        self.arrow_tip(x + r * 0.87, y - r * 0.5, 0.5, 0.87, g);
    }
}

/// Annotate tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        CheckSpelling => {
            pen.text(9.0, 7.5, "abc", 9.0, b);
            pen.poly(&[(5.0, 15.0), (9.5, 20.0), (20.0, 10.0)], false, pen.tint(GREEN));
        }
        TextAlign => {
            for (x, y) in [(5.0, 18.0), (12.0, 12.0), (19.0, 6.0)] {
                pen.text(x, y, "A", 9.0, b);
            }
            pen.dash(2.0, 22.0, 22.0, 2.0, a);
        }
        Justify => {
            for (y, w) in [(5.0, 16.0), (10.0, 12.0), (15.0, 16.0)] {
                pen.l(4.0, y, 4.0 + w, y);
            }
            pen.first(12.0, 20.0);
        }
        ScaleText => {
            pen.text(5.5, 16.0, "A", 8.0, b);
            pen.text(16.0, 12.0, "A", 17.0, b);
            pen.arrow(3.0, 22.0, 10.0, 22.0, a);
        }
        DimBreak => {
            pen.la(3.0, 19.0, 3.0, 5.0);
            pen.la(21.0, 19.0, 21.0, 5.0);
            pen.la(5.0, 9.0, 9.5, 9.0);
            pen.la(14.5, 9.0, 19.0, 9.0);
            pen.arrow_tip(3.0, 9.0, -1.0, 0.0, a);
            pen.arrow_tip(21.0, 9.0, 1.0, 0.0, a);
            pen.l(12.0, 2.0, 12.0, 22.0);
        }
        DimSpace => {
            dim(pen, 3.0, 13.0, 15.0, 13.0, 21.0);
            dim(pen, 3.0, 21.0, 5.0, 3.0, 21.0);
            pen.l(17.0, 7.0, 17.0, 13.0);
            pen.arrow_tip(17.0, 7.0, 0.0, -1.0, b);
            pen.arrow_tip(17.0, 13.0, 0.0, 1.0, b);
        }
        DimJogLine => {
            pen.la(3.0, 21.0, 3.0, 6.0);
            pen.la(21.0, 21.0, 21.0, 6.0);
            pen.poly(&[(5.0, 10.0), (10.0, 10.0), (11.5, 7.0), (12.5, 13.0), (14.0, 10.0), (19.0, 10.0)], false, a);
            pen.arrow_tip(3.0, 10.0, -1.0, 0.0, a);
            pen.arrow_tip(21.0, 10.0, 1.0, 0.0, a);
        }
        DimInspect => {
            dim(pen, 2.0, 22.0, 15.0, 13.0, 21.0);
            let frame: [P; 6] = [(7.0, 2.0), (17.0, 2.0), (19.5, 6.0), (17.0, 10.0), (7.0, 10.0), (4.5, 6.0)];
            pen.poly(&frame, true, b);
            pen.text(12.0, 6.2, "1.5", 6.0, b);
        }
        DimUpdate => {
            dim(pen, 2.0, 16.0, 6.0, 2.0, 12.0);
            pen.refresh(16.5, 16.5, 5.0);
        }
        DimReassociate => {
            dim(pen, 2.0, 16.0, 6.0, 2.0, 12.0);
            pen.chain(16.5, 16.5, b);
        }
        Tolerance => {
            pen.rect(2.0, 7.0, 20.0, 10.0, b);
            pen.l(9.0, 7.0, 9.0, 17.0);
            pen.l(17.0, 7.0, 17.0, 17.0);
            pen.c(5.5, 12.0, 2.3, a);
            pen.la(4.2, 12.0, 6.8, 12.0);
            pen.la(5.5, 10.7, 5.5, 13.3);
            pen.text(13.0, 12.2, ".1", 6.5, b);
            pen.text(19.5, 12.2, "A", 6.5, b);
        }
        DimOblique => {
            pen.la(3.0, 21.0, 8.0, 5.0);
            pen.la(16.0, 21.0, 21.0, 5.0);
            pen.la(7.0, 10.0, 18.0, 10.0);
            pen.arrow_tip(6.4, 10.0, -1.0, 0.0, a);
            pen.arrow_tip(19.4, 10.0, 1.0, 0.0, a);
        }
        DimTextAngle => {
            dim(pen, 3.0, 21.0, 18.0, 15.0, 22.0);
            pen.l(6.0, 14.0, 17.0, 3.0);
            pen.text(11.0, 8.0, "A", 8.0, a);
        }
        DimTextLeft | DimTextCenter | DimTextRight => {
            dim(pen, 3.0, 21.0, 18.0, 13.0, 22.0);
            let x = match icon {
                DimTextLeft => 4.0,
                DimTextCenter => 8.0,
                _ => 12.0,
            };
            pen.fill_rect(x, 5.0, 8.0, 6.0, b.gamma_multiply(0.35));
            pen.rect(x, 5.0, 8.0, 6.0, b);
        }
        DimOverride => {
            dim(pen, 2.0, 16.0, 6.0, 2.0, 12.0);
            for (dx, dy) in [(0.0, 4.5), (3.9, 2.25), (3.9, -2.25)] {
                pen.thick(17.0 - dx, 17.0 - dy, 17.0 + dx, 17.0 + dy, 1.4, pen.pt);
            }
        }
        CenterMark => {
            pen.c(12.0, 12.0, 7.0, b);
            pen.la(9.0, 12.0, 15.0, 12.0);
            pen.la(12.0, 9.0, 12.0, 15.0);
            for (x1, y1, x2, y2) in [(1.0, 12.0, 6.0, 12.0), (18.0, 12.0, 23.0, 12.0), (12.0, 1.0, 12.0, 6.0), (12.0, 18.0, 12.0, 23.0)] {
                pen.la(x1, y1, x2, y2);
            }
        }
        Centerline => {
            pen.l(3.0, 7.0, 21.0, 7.0);
            pen.l(3.0, 17.0, 21.0, 17.0);
            pen.la(1.0, 12.0, 7.0, 12.0);
            pen.la(9.0, 12.0, 11.0, 12.0);
            pen.la(13.0, 12.0, 19.0, 12.0);
            pen.la(21.0, 12.0, 23.0, 12.0);
        }
        AddScale => {
            pen.anno();
            pen.thick(18.5, 13.0, 18.5, 22.0, 2.0, pen.tint(GREEN));
            pen.thick(14.0, 17.5, 23.0, 17.5, 2.0, pen.tint(GREEN));
        }
        DeleteScale => {
            pen.anno();
            pen.thick(15.0, 14.0, 22.0, 21.0, 2.0, pen.pt);
            pen.thick(15.0, 21.0, 22.0, 14.0, 2.0, pen.pt);
        }
        ScaleList => {
            pen.anno();
            for y in [12.0, 16.0, 20.0] {
                pen.la(15.0, y, 22.0, y);
            }
        }
        AddDeleteScales => {
            pen.anno();
            pen.la(16.0, 14.0, 22.0, 14.0);
            pen.la(19.0, 11.0, 19.0, 17.0);
            pen.la(16.0, 20.5, 22.0, 20.5);
        }
        SyncScales => {
            pen.anno();
            pen.refresh(17.5, 17.0, 4.5);
        }
        _ => return false,
    }
    true
}
