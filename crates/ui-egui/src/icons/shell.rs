//! Icons of the window shell around the drawing: the status bar items AutoCAD's Drafting &
//! Annotation workspace shows (and the ones its Customization menu can add), the command line's
//! controls, the navigation bar and the drawing window's buttons (original work, drawn in code
//! like the rest of the set, on the same 24×24 grid).

use super::{Icon, Pen};

/// Paint one of the shell icons `icons::paint` hands over.
pub(super) fn paint(pen: &Pen, icon: Icon) {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        AutoScale => {
            // A triangle (annotative) with a small scale stair beside it.
            pen.poly(&[(9.0, 4.0), (16.0, 18.0), (2.0, 18.0)], true, b);
            pen.poly(&[(15.0, 21.0), (15.0, 17.0), (18.0, 17.0), (18.0, 13.0), (21.0, 13.0), (21.0, 21.0)], true, a);
        }
        AnnoMonitor => {
            pen.l(12.0, 4.0, 12.0, 20.0);
            pen.l(4.0, 12.0, 20.0, 12.0);
            pen.c(12.0, 12.0, 4.0, a);
        }
        Isolate => {
            // A light bulb.
            pen.arc(12.0, 10.0, 6.0, -40.0, 220.0, b);
            pen.poly(&[(7.4, 13.9), (9.0, 17.0), (15.0, 17.0), (16.6, 13.9)], false, b);
            pen.l(9.5, 19.5, 14.5, 19.5);
            pen.la(10.5, 21.5, 13.5, 21.5);
        }
        CleanScreen => {
            pen.rect(3.0, 5.0, 18.0, 14.0, b);
            for (x, y, dx, dy) in [(6.0, 8.0, 1.0, 1.0), (18.0, 8.0, -1.0, 1.0), (6.0, 16.0, 1.0, -1.0), (18.0, 16.0, -1.0, -1.0)] {
                pen.poly(&[(x + 3.0 * dx, y), (x, y), (x, y + 3.0 * dy)], false, a);
            }
        }
        SelCycling => {
            pen.rect(3.0, 3.0, 11.0, 11.0, b);
            pen.rect(10.0, 10.0, 11.0, 11.0, b);
            pen.arc(12.0, 12.0, 4.0, 30.0, 300.0, a);
        }
        Osnap3d => {
            pen.poly(&[(12.0, 3.0), (20.0, 7.0), (20.0, 16.0), (12.0, 20.0), (4.0, 16.0), (4.0, 7.0)], true, b);
            pen.rect(9.0, 9.0, 6.0, 6.0, a);
        }
        DynUcs => {
            pen.l(5.0, 19.0, 19.0, 19.0);
            pen.l(5.0, 19.0, 5.0, 5.0);
            pen.la(5.0, 19.0, 15.0, 9.0);
            pen.arrow_tip(16.0, 8.0, 1.0, -1.0, a);
        }
        SelFilter => {
            pen.poly(&[(3.0, 4.0), (21.0, 4.0), (14.0, 12.0), (14.0, 20.0), (10.0, 18.0), (10.0, 12.0)], true, b);
        }
        Gizmo => {
            pen.la(12.0, 12.0, 21.0, 12.0);
            pen.l(12.0, 12.0, 12.0, 3.0);
            pen.l(12.0, 12.0, 5.0, 19.0);
            pen.fill_c(12.0, 12.0, 1.8, b);
        }
        GraphicsPerf => {
            pen.arc(12.0, 15.0, 8.0, 0.0, 180.0, b);
            pen.la(12.0, 15.0, 17.0, 9.0);
            pen.fill_c(12.0, 15.0, 1.5, a);
        }
        Units => {
            pen.l(3.0, 15.0, 21.0, 15.0);
            for (k, x) in [3.0f32, 7.5, 12.0, 16.5, 21.0].iter().enumerate() {
                let top = if k % 2 == 0 { 9.0 } else { 12.0 };
                pen.l(*x, top, *x, 15.0);
            }
        }
        Wrench => {
            // A handle and a head with its jaw open to the upper right.
            pen.l(5.0, 19.0, 12.8, 11.2);
            pen.arc(15.5, 8.5, 4.2, 80.0, 370.0, b);
            pen.fill_c(5.0, 19.0, 1.3, b);
        }
        Prompt => {
            // A command window: ">_" in a box.
            pen.rect(2.0, 5.0, 20.0, 14.0, b);
            pen.poly(&[(6.0, 9.0), (9.0, 12.0), (6.0, 15.0)], false, b);
            pen.l(11.0, 15.0, 16.0, 15.0);
        }
        NavWheel => {
            pen.c(12.0, 12.0, 9.0, b);
            pen.c(12.0, 12.0, 4.0, b);
            for (x1, y1, x2, y2) in [(12.0, 3.0, 12.0, 8.0), (12.0, 16.0, 12.0, 21.0), (3.0, 12.0, 8.0, 12.0), (16.0, 12.0, 21.0, 12.0)] {
                pen.l(x1, y1, x2, y2);
            }
        }
        ShowMotion => {
            // A film strip with a play triangle.
            pen.rect(3.0, 5.0, 18.0, 14.0, b);
            for x in [5.0, 9.0, 13.0, 17.0] {
                pen.l(x, 5.0, x, 7.0);
                pen.l(x, 17.0, x, 19.0);
            }
            pen.tri([(10.0, 9.0), (15.0, 12.0), (10.0, 15.0)], b);
        }
        VpMinimize => pen.l(6.0, 17.0, 18.0, 17.0),
        VpRestore => {
            pen.rect(5.0, 9.0, 10.0, 10.0, b);
            pen.poly(&[(9.0, 9.0), (9.0, 5.0), (19.0, 5.0), (19.0, 15.0), (15.0, 15.0)], false, b);
        }
        _ => {}
    }
}
