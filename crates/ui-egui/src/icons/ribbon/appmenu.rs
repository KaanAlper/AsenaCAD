//! Application menu icons: the first-level entries that have no ribbon icon, the sub-pane
//! entries (file formats, publishing, plotting, drawing utilities) and the document pane's
//! controls. Shared motifs: a page for a drawing file, a badge with a format's letters, a
//! printer for plotting.

use super::GREEN;
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// A second page behind a first one, both top-left based, for "several drawings".
    fn pages(&self, c: egui::Color32) {
        self.page(7.0, 2.0, 13.0, 16.0, c);
        self.knock(3.0, 6.0, 14.0, 17.0);
        self.page(4.0, 6.0, 13.0, 16.0, c);
    }
    /// A small cross in the bottom-right corner, for closing.
    fn close_badge(&self) {
        self.knock(13.0, 13.0, 11.0, 11.0);
        self.thick(15.0, 15.0, 22.0, 22.0, 1.8, self.pt);
        self.thick(22.0, 15.0, 15.0, 22.0, 1.8, self.pt);
    }
}

/// Application menu icons; false when `icon` belongs to another set.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        DrawingUtilities => {
            pen.page(3.0, 2.0, 14.0, 19.0, b);
            pen.knock(10.0, 9.0, 14.0, 15.0);
            pen.thick(12.0, 21.0, 18.5, 14.5, 2.0, a);
            pen.arc(20.0, 13.0, 3.2, 80.0, 370.0, a);
        }
        CloseDrawing => {
            pen.page(4.0, 2.0, 14.0, 19.0, b);
            pen.close_badge();
        }
        CloseAll => {
            pen.pages(b);
            pen.close_badge();
        }
        WebMobile => {
            // A cloud with a phone in front of it.
            pen.arc(9.0, 11.0, 4.5, 20.0, 200.0, b);
            pen.arc(15.0, 10.0, 5.0, -20.0, 160.0, b);
            pen.l(4.5, 13.5, 20.0, 13.5);
            pen.knock(12.0, 11.0, 10.0, 13.0);
            pen.rect(13.0, 11.0, 8.0, 12.0, a);
            pen.fill_c(17.0, 20.5, 0.9, a);
        }
        SampleFiles => {
            pen.poly(&[(2.0, 6.0), (8.0, 6.0), (10.0, 8.0), (21.0, 8.0), (21.0, 20.0), (2.0, 20.0)], true, b);
            let g = pen.tint(GREEN);
            pen.fill_poly(
                &[
                    (14.0, 10.5),
                    (15.5, 13.5),
                    (18.5, 13.8),
                    (16.2, 15.8),
                    (17.0, 18.8),
                    (14.0, 17.2),
                    (11.0, 18.8),
                    (11.8, 15.8),
                    (9.5, 13.8),
                    (12.5, 13.5),
                ],
                g,
            );
        }
        SaveTemplate | SaveStandards | SaveOther | Export3dDwf | ExportDgn => {
            pen.page(4.0, 2.0, 15.0, 20.0, b);
            let (text, c) = match icon {
                SaveTemplate => ("DWT", a),
                SaveStandards => ("DWS", a),
                SaveOther => ("•••", b),
                Export3dDwf => ("3D", a),
                _ => ("DGN", pen.pt),
            };
            pen.badge(1.0, 12.0, 22.0, 7.0, text, c);
        }
        SaveLayout => {
            pen.page(3.0, 2.0, 18.0, 15.0, b);
            pen.dash_rect(6.0, 5.0, 10.0, 8.0, a);
            pen.rect(3.0, 17.0, 7.0, 4.0, b);
            pen.fill_rect(10.0, 17.0, 7.0, 4.0, a);
        }
        DwgConvert => {
            pen.page(2.0, 2.0, 10.0, 13.0, b);
            pen.page(12.0, 9.0, 10.0, 13.0, a);
            pen.arrow(4.0, 20.0, 10.0, 20.0, a);
            pen.arrow(20.0, 5.0, 14.0, 5.0, b);
        }
        PrintService3d | Print3d => {
            // A cube on a printer's bed; the service adds a cloud.
            pen.block(5.0, 4.0, 0.8, a);
            pen.l(2.0, 21.0, 22.0, 21.0);
            pen.l(4.0, 18.5, 20.0, 18.5);
            if icon == PrintService3d {
                pen.arc(18.0, 6.0, 3.5, 0.0, 180.0, b);
                pen.l(14.5, 6.0, 21.5, 6.0);
            }
        }
        Archive => {
            pen.rect(3.0, 4.0, 18.0, 5.0, b);
            pen.poly(&[(4.5, 9.0), (4.5, 20.0), (19.5, 20.0), (19.5, 9.0)], false, b);
            pen.thick(9.5, 12.5, 14.5, 12.5, 1.8, a);
        }
        ETransmit => {
            pen.poly(&[(3.0, 9.0), (12.0, 5.0), (21.0, 9.0), (21.0, 19.0), (12.0, 22.0), (3.0, 19.0)], true, b);
            pen.poly(&[(3.0, 9.0), (12.0, 12.5), (21.0, 9.0)], false, b);
            pen.l(12.0, 12.5, 12.0, 22.0);
            pen.knock(14.0, 1.0, 10.0, 7.0);
            pen.arrow(14.5, 4.0, 22.5, 4.0, a);
        }
        Email => {
            pen.rect(2.0, 5.0, 20.0, 14.0, b);
            pen.poly(&[(2.0, 5.0), (12.0, 13.0), (22.0, 5.0)], false, a);
        }
        PlotStyles | EditPlotStyles => {
            pen.page(3.0, 2.0, 15.0, 20.0, b);
            for (k, y) in [6.5f32, 10.5, 14.5].iter().enumerate() {
                let w = [1.0, 1.8, 2.6][k];
                pen.thick(6.0, *y, 14.0, *y, w, if k == 1 { a } else { b });
            }
            if icon == EditPlotStyles {
                pen.pencil();
            }
        }
        DrawingProperties => {
            pen.page(3.0, 2.0, 15.0, 20.0, b);
            pen.knock(11.0, 10.0, 13.0, 14.0);
            pen.c(17.0, 17.0, 5.0, a);
            pen.thick(17.0, 16.0, 17.0, 20.0, 1.6, a);
            pen.fill_c(17.0, 13.8, 0.9, a);
        }
        Status => {
            pen.page(3.0, 2.0, 15.0, 20.0, b);
            for (y, w) in [(7.0, 8.0), (10.5, 5.0), (14.0, 9.0), (17.5, 6.0)] {
                pen.l(6.0, y, 6.0 + w, y);
            }
            pen.fill_c(19.0, 19.0, 3.0, pen.tint(GREEN));
        }
        Recover | RecoverXrefs => {
            pen.page(3.0, 2.0, 15.0, 20.0, b);
            pen.knock(10.0, 10.0, 14.0, 14.0);
            pen.arc(17.0, 17.0, 4.5, 120.0, 420.0, a);
            pen.arrow_tip(14.8, 13.1, -1.0, -0.6, a);
            if icon == RecoverXrefs {
                pen.dash(6.0, 7.0, 13.0, 7.0, b);
            }
        }
        RecoveryManager => {
            pen.pages(b);
            pen.knock(11.0, 11.0, 13.0, 13.0);
            pen.c(17.5, 17.5, 5.0, a);
            pen.l(17.5, 14.5, 17.5, 17.5);
            pen.l(17.5, 17.5, 20.0, 19.0);
        }
        RecentDocs => {
            pen.page(3.0, 2.0, 14.0, 19.0, b);
            pen.knock(11.0, 10.0, 13.0, 14.0);
            pen.c(17.0, 17.0, 5.0, a);
            pen.la(17.0, 14.0, 17.0, 17.0);
            pen.la(17.0, 17.0, 19.5, 18.5);
        }
        OpenDocs => {
            pen.pages(b);
            pen.fill_rect(4.0, 6.0, 9.0, 3.0, a);
        }
        Document => {
            pen.page(5.0, 2.0, 14.0, 20.0, b);
            pen.l(8.0, 16.0, 13.0, 9.0);
            pen.la(8.0, 16.0, 16.0, 16.0);
        }
        ViewList => {
            for y in [6.0, 12.0, 18.0] {
                pen.fill_rect(3.0, y - 1.5, 3.0, 3.0, a);
                pen.l(9.0, y, 21.0, y);
            }
        }
        ViewIcons => {
            for (x, y) in [(3.0, 3.0), (13.0, 3.0), (3.0, 13.0), (13.0, 13.0)] {
                pen.rect(x, y, 8.0, 8.0, b);
            }
            pen.fill_rect(5.0, 5.0, 4.0, 4.0, a);
        }
        _ => return false,
    }
    true
}
