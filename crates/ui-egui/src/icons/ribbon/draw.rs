//! Draw and Modify panel variants.

use super::{P, bez, ell_at};
use crate::icons::{Icon, Pen};

/// The arc every Arc variant shares: centre C, start S (right, the arc runs CCW) and end E.
const C: P = (12.0, 17.5);
const R: f32 = 9.5;
const S: P = (21.18, 15.04);
const E: P = (2.82, 15.04);

/// Draw/Modify icons; false when `icon` belongs to another panel.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        CircleDiameter => {
            pen.c(12.0, 12.0, 8.0, a);
            pen.l(6.3, 17.7, 17.7, 6.3);
            pen.dot(12.0, 12.0);
            pen.dot(17.7, 6.3);
        }
        Circle2Point => {
            pen.c(12.0, 12.0, 8.0, a);
            pen.dot(4.0, 12.0);
            pen.dot(20.0, 12.0);
        }
        Circle3Point => {
            pen.c(12.0, 12.0, 8.0, a);
            pen.dot(12.0, 4.0);
            pen.dot(5.07, 16.0);
            pen.dot(18.93, 16.0);
        }
        CircleTtr => {
            pen.l(2.0, 20.0, 22.0, 20.0);
            pen.l(4.0, 22.0, 4.0, 2.0);
            pen.c(11.0, 13.0, 7.0, a);
            pen.l(11.0, 13.0, 15.95, 8.05);
            pen.dot(11.0, 20.0);
            pen.dot(4.0, 13.0);
        }
        CircleTtt => {
            pen.poly(&[(1.5, 20.5), (22.5, 20.5), (12.0, 2.5)], true, b);
            pen.c(12.0, 14.5, 6.0, a);
            pen.dot(12.0, 20.5);
            pen.dot(6.8, 11.5);
            pen.dot(17.2, 11.5);
        }
        ArcSce | ArcSca | ArcScl | ArcSea | ArcSed | ArcSer | ArcCse | ArcCsa | ArcCsl => arc_variant(pen, icon),
        ArcContinue => {
            pen.l(2.0, 20.0, 11.0, 20.0);
            pen.arc(11.0, 13.0, 7.0, 270.0, 450.0, a);
            pen.arrow_tip(9.5, 6.0, -1.0, 0.0, a);
            pen.dot(11.0, 20.0);
        }
        EllipseAxis => {
            pen.ell(12.0, 12.0, 9.5, 5.0, 30.0, 0.0, 360.0, a);
            let (p0, p1, q) =
                (ell_at(12.0, 12.0, 9.5, 5.0, 30.0, 0.0), ell_at(12.0, 12.0, 9.5, 5.0, 30.0, 180.0), ell_at(12.0, 12.0, 9.5, 5.0, 30.0, 90.0));
            pen.l(p0.0, p0.1, p1.0, p1.1);
            pen.dot(p0.0, p0.1);
            pen.dot(p1.0, p1.1);
            pen.dot(q.0, q.1);
        }
        EllipseArc => {
            pen.ell(12.0, 13.0, 9.5, 6.0, 0.0, 0.0, 210.0, a);
            for k in 0..4 {
                let t = 220.0 + k as f32 * 36.0;
                pen.ell(12.0, 13.0, 9.5, 6.0, 0.0, t, t + 16.0, b);
            }
            let p = ell_at(12.0, 13.0, 9.5, 6.0, 0.0, 210.0);
            pen.dot(21.5, 13.0);
            pen.dot(p.0, p.1);
        }
        SplineCv => {
            let cv = [(3.0, 20.0), (7.0, 4.0), (17.0, 20.0), (21.0, 4.0)];
            pen.poly(&cv, false, b);
            pen.curve(cv, a);
            for (x, y) in cv {
                pen.dot(x, y);
            }
        }
        Divide => {
            let p = [(2.0, 18.0), (7.0, 4.0), (17.0, 4.0), (22.0, 18.0)];
            pen.curve(p, b);
            for t in [0.25, 0.5, 0.75] {
                let (x, y) = bez(p, t);
                pen.mark(x, y);
            }
        }
        MeasurePoints => {
            pen.l(2.0, 16.0, 22.0, 16.0);
            for x in [7.5, 13.0, 18.5] {
                pen.mark(x, 16.0);
            }
            pen.l(2.0, 8.0, 7.5, 8.0);
            pen.l(2.0, 6.0, 2.0, 10.0);
            pen.l(7.5, 6.0, 7.5, 10.0);
        }
        Polyline3d => {
            let floor = [(2.0, 17.0), (12.0, 12.0), (22.0, 17.0), (12.0, 22.0)];
            for i in 0..4 {
                let (p, q) = (floor[i], floor[(i + 1) % 4]);
                pen.dash(p.0, p.1, q.0, q.1, b);
            }
            let pts = [(6.0, 19.0), (16.0, 14.0), (16.0, 4.0), (8.0, 1.5)];
            pen.poly(&pts, false, a);
            for (x, y) in pts {
                pen.dot(x, y);
            }
        }
        Helix => {
            let pts: Vec<P> = (0..=72)
                .map(|i| {
                    let t = (i as f32 * 15.0).to_radians();
                    (12.0 + 7.5 * t.cos(), 19.5 - 15.0 * i as f32 / 72.0 + 2.5 * t.sin())
                })
                .collect();
            pen.poly(&pts, false, a);
            pen.dash(12.0, 2.0, 12.0, 23.0, b);
        }
        RevCloudPolygonal => {
            let pts = [(3.5, 19.5), (12.0, 4.0), (20.5, 19.5)];
            pen.cloud(&pts, true, 5.0, a);
            for (x, y) in pts {
                pen.dot(x, y);
            }
        }
        RevCloudFreehand => {
            let pts: Vec<P> = (0..9)
                .map(|i| {
                    let t = (i as f32 * 36.0 + 100.0).to_radians();
                    let r = 7.5 + 1.4 * (2.0 * t).sin() + 0.8 * (3.0 * t).cos();
                    (11.5 + r * t.cos(), 11.5 + r * t.sin())
                })
                .collect();
            pen.cloud(&pts, false, 3.4, a);
            pen.dot(pts[8].0, pts[8].1);
        }
        Blend => {
            pen.l(2.0, 6.0, 9.0, 6.0);
            pen.l(15.0, 18.0, 22.0, 18.0);
            pen.curve([(9.0, 6.0), (15.0, 6.0), (9.0, 18.0), (15.0, 18.0)], a);
            pen.dot(9.0, 6.0);
            pen.dot(15.0, 18.0);
        }
        ArrayPath => {
            let p = [(2.0, 21.0), (5.0, 5.0), (17.0, 3.0), (22.0, 16.0)];
            pen.curve(p, b);
            for (k, t) in [0.0f32, 0.36, 0.68, 1.0].iter().enumerate() {
                let (x, y) = bez(p, *t);
                let x = x.clamp(3.5, 20.5);
                pen.knock(x - 2.0, y - 2.0, 4.0, 4.0);
                pen.rect(x - 2.0, y - 2.0, 4.0, 4.0, if k == 0 { b } else { a });
            }
        }
        SetByLayer => {
            pen.layers(b);
            pen.arrow(13.0, 15.0, 15.8, 17.8, a);
            pen.fill_rect(16.0, 16.0, 6.0, 6.0, a);
        }
        ChangeSpace => {
            pen.rect(2.0, 3.0, 20.0, 18.0, b);
            pen.rect(10.0, 6.0, 10.0, 9.0, b);
            pen.c(5.5, 17.0, 2.0, a);
            pen.arrow(7.5, 15.0, 12.5, 11.0, a);
        }
        EditPolyline => {
            pen.poly(&[(2.0, 16.0), (6.0, 4.0), (11.0, 12.0), (16.0, 4.0)], false, a);
            pen.pencil();
        }
        EditSpline => {
            pen.curve([(2.0, 12.0), (6.0, -1.0), (10.0, 21.0), (16.0, 4.0)], a);
            pen.pencil();
        }
        EditHatch => {
            pen.rect(2.0, 2.0, 12.0, 12.0, b);
            pen.la(2.0, 8.0, 8.0, 2.0);
            pen.la(2.0, 14.0, 14.0, 2.0);
            pen.la(8.0, 14.0, 14.0, 8.0);
            pen.pencil();
        }
        EditArray => {
            for (x, y) in [(2.0, 2.0), (9.0, 2.0), (2.0, 9.0), (9.0, 9.0)] {
                pen.rect(x, y, 4.5, 4.5, if x + y < 5.0 { b } else { a });
            }
            pen.pencil();
        }
        Align => {
            pen.l(2.0, 21.0, 22.0, 21.0);
            let sq: Vec<P> = (0..4)
                .map(|i| {
                    let t = (35.0 + i as f32 * 90.0f32).to_radians();
                    (7.0 + 4.6 * t.cos(), 8.0 - 4.6 * t.sin())
                })
                .collect();
            pen.poly(&sq, true, b);
            pen.rect(13.0, 13.5, 7.5, 7.5, a);
            pen.arc(16.0, 9.0, 5.0, 150.0, 20.0, a);
            pen.arrow_tip(20.8, 7.8, 0.4, 1.0, a);
            pen.dot(13.0, 21.0);
            pen.dot(20.5, 21.0);
        }
        BreakAtPoint => {
            pen.l(2.0, 12.0, 11.0, 12.0);
            pen.la(13.0, 12.0, 22.0, 12.0);
            pen.l(12.0, 6.0, 12.0, 9.0);
            pen.l(12.0, 15.0, 12.0, 18.0);
            pen.dot(12.0, 12.0);
        }
        Reverse => {
            pen.arrow(3.0, 7.0, 21.5, 7.0, b);
            pen.arrow(21.0, 16.0, 2.5, 16.0, a);
            pen.dot(3.0, 7.0);
            pen.dot(21.0, 16.0);
        }
        CopyNested => {
            pen.block(1.0, 2.0, 0.85, b);
            pen.rect(5.0, 8.5, 4.0, 4.0, b);
            pen.arrow(10.0, 12.5, 14.5, 16.0, a);
            pen.rect(15.5, 16.0, 6.0, 6.0, a);
        }
        Overkill => {
            pen.l(2.0, 8.0, 22.0, 8.0);
            pen.l(2.0, 11.0, 22.0, 11.0);
            pen.la(8.0, 14.5, 16.0, 22.5);
            pen.la(16.0, 14.5, 8.0, 22.5);
        }
        DrawOrderFront | DrawOrderAbove => {
            pen.overlap(b, a);
            pen.order_arrow(true, icon == DrawOrderFront);
        }
        DrawOrderBack | DrawOrderUnder => {
            pen.overlap(a, b);
            pen.order_arrow(false, icon == DrawOrderBack);
        }
        _ => return false,
    }
    true
}

/// Arc variants: dots on the picked points (a ring on the first pick) and a mark for the value.
fn arc_variant(pen: &Pen, icon: Icon) {
    use Icon::*;
    let b = pen.base;
    pen.arc(C.0, C.1, R, 15.0, 165.0, pen.acc);
    let center_first = matches!(icon, ArcCse | ArcCsa | ArcCsl);
    let center = matches!(icon, ArcSce | ArcSca | ArcScl) || center_first;
    match icon {
        ArcSca | ArcSea | ArcCsa => {
            pen.l(C.0, C.1, S.0, S.1);
            pen.l(C.0, C.1, E.0, E.1);
            pen.arc(C.0, C.1, 4.0, 15.0, 165.0, b);
        }
        ArcScl | ArcCsl => pen.l(S.0, S.1, E.0, E.1),
        ArcSed => pen.arrow(S.0, S.1, 19.2, 7.7, b),
        ArcSer => pen.arrow(C.0, C.1, 16.75, 9.27, b),
        _ => {}
    }
    if center_first {
        pen.first(C.0, C.1);
        pen.dot(S.0, S.1);
    } else {
        pen.first(S.0, S.1);
        if center {
            pen.dot(C.0, C.1);
        }
    }
    if matches!(icon, ArcSce | ArcSea | ArcSed | ArcSer | ArcCse) {
        pen.dot(E.0, E.1);
    }
}
