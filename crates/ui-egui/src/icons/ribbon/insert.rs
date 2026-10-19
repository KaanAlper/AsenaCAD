//! Insert tab tools: block definition, references and underlays, import, fields, data links and
//! the geographic location. Shared motifs: a page with a folded corner for files and
//! references, a small table for data links, a globe for locations.

use egui::Color32;

use super::{GREEN, ICE, P};
use crate::icons::{Icon, Pen};

impl Pen<'_> {
    /// A sheet with its top-right corner folded, top-left at (x, y).
    pub(super) fn page(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        let f = (w * 0.3).min(5.0);
        self.solid(&[(x, y), (x + w - f, y), (x + w, y + f), (x + w, y + h), (x, y + h)], c);
        self.poly(&[(x + w - f, y), (x + w - f, y + f), (x + w, y + f)], false, c);
    }
    /// A table of `cols` × `rows` cells, top-left at (x, y), with a filled title row.
    pub(super) fn grid(&self, x: f32, y: f32, w: f32, h: f32, cols: u8, rows: u8, c: Color32) {
        let rh = h / f32::from(rows.max(1));
        self.fill_rect(x, y, w, rh, c.gamma_multiply(0.35));
        self.rect(x, y, w, h, c);
        for r in 1..rows {
            let yy = y + rh * f32::from(r);
            self.poly(&[(x, yy), (x + w, yy)], false, c);
        }
        let cw = w / f32::from(cols.max(1));
        for k in 1..cols {
            let xx = x + cw * f32::from(k);
            self.poly(&[(xx, y + rh), (xx, y + h)], false, c);
        }
    }
    /// A globe of radius `r` centred at (x, y).
    pub(super) fn globe(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.c(x, y, r, c);
        self.ell(x, y, r * 0.45, r, 0.0, 0.0, 360.0, c);
        self.poly(&[(x - r, y), (x + r, y)], false, c);
    }
    /// Two chain links along the diagonal, centred at (x, y).
    pub(super) fn chain(&self, x: f32, y: f32, c: Color32) {
        self.ell(x - 2.2, y + 2.2, 4.2, 2.0, 45.0, 0.0, 360.0, c);
        self.ell(x + 2.2, y - 2.2, 4.2, 2.0, 45.0, 0.0, 360.0, c);
    }
    /// A map pin with its tip at (x, y).
    pub(super) fn map_pin(&self, x: f32, y: f32) {
        self.fill_poly(&[(x, y), (x - 3.2, y - 5.5), (x + 3.2, y - 5.5)], self.pt);
        self.fill_c(x, y - 7.0, 3.6, self.pt);
        self.fill_c(x, y - 7.0, 1.4, crate::theme::Tokens::get().panel);
    }
}

/// Insert tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        WriteBlock => {
            pen.block(1.5, 3.0, 0.75, b);
            pen.page(13.0, 9.0, 9.0, 12.0, b);
            pen.arrow(8.0, 17.0, 16.5, 17.0, a);
        }
        XClip => {
            pen.page(2.0, 2.0, 15.0, 19.0, b);
            pen.la(5.0, 8.0, 10.0, 13.0);
            pen.la(5.0, 16.0, 13.0, 8.0);
            let clip: [P; 5] = [(8.0, 6.0), (22.0, 6.0), (22.0, 22.0), (12.0, 22.0), (8.0, 15.0)];
            pen.poly(&clip, true, pen.tint(GREEN));
        }
        UnderlayAdjust => {
            pen.page(2.0, 2.0, 15.0, 19.0, b);
            pen.c(15.5, 15.5, 6.0, a);
            pen.fill_poly(&[(15.5, 9.5), (19.7, 11.3), (21.5, 15.5), (19.7, 19.7), (15.5, 21.5)], a);
        }
        UnderlayLayers => {
            pen.layers(b);
            pen.page(14.0, 11.0, 8.0, 11.0, a);
        }
        RefEdit => {
            pen.page(2.0, 2.0, 13.0, 17.0, b);
            pen.rect(5.0, 8.0, 6.0, 6.0, a);
            pen.pencil();
        }
        XrefFade => {
            pen.page(2.0, 2.0, 17.0, 20.0, b);
            pen.fill_rect(5.0, 9.0, 11.0, 10.0, b.gamma_multiply(0.25));
            pen.rect(5.0, 9.0, 11.0, 10.0, b.gamma_multiply(0.5));
            pen.la(7.0, 16.0, 14.0, 11.0);
        }
        PdfImport | DgnImport => {
            pen.page(6.0, 2.0, 16.0, 20.0, b);
            pen.fill_rect(9.0, 12.0, 13.0, 7.0, pen.pt);
            pen.text(15.5, 15.6, if icon == PdfImport { "PDF" } else { "DGN" }, 5.5, crate::theme::Tokens::get().panel);
            pen.arrow(1.0, 6.0, 10.5, 6.0, a);
        }
        RecognizeShx => {
            pen.dash(3.0, 20.0, 7.0, 4.0, b);
            pen.dash(7.0, 4.0, 11.0, 20.0, b);
            pen.dash(4.5, 14.0, 9.5, 14.0, b);
            pen.arrow(12.0, 12.0, 15.0, 12.0, a);
            pen.text(19.0, 12.5, "A", 13.0, a);
        }
        ShxSettings => {
            pen.dash(2.0, 14.0, 5.5, 2.0, b);
            pen.dash(5.5, 2.0, 9.0, 14.0, b);
            pen.dash(3.2, 10.0, 7.8, 10.0, b);
            for (y, k) in [(15.0, 17.0), (19.0, 13.0), (23.0, 19.0)] {
                pen.la(10.0, y - 1.5, 22.0, y - 1.5);
                pen.fill_c(k, y - 1.5, 1.6, pen.pt);
            }
        }
        CombineText => {
            for y in [4.0, 8.0, 12.0] {
                pen.l(2.0, y, 11.0, y);
            }
            pen.arrow(12.0, 8.0, 15.5, 8.0, a);
            pen.rect(13.0, 12.0, 9.0, 10.0, a);
            pen.text(17.5, 17.2, "A", 8.5, a);
        }
        Field | UpdateFields => {
            pen.fill_rect(2.0, 6.0, 15.0, 10.0, b.gamma_multiply(0.3));
            pen.rect(2.0, 6.0, 15.0, 10.0, b);
            pen.text(9.5, 11.2, "#", 9.0, a);
            if icon == UpdateFields {
                pen.arc(18.0, 17.0, 4.5, 20.0, 300.0, pen.tint(GREEN));
                pen.arrow_tip(22.2, 15.4, 0.4, -1.0, pen.tint(GREEN));
            } else {
                pen.l(2.0, 19.5, 22.0, 19.5);
            }
        }
        OleObject => {
            pen.rect(2.0, 3.0, 18.0, 15.0, b);
            pen.l(2.0, 6.5, 20.0, 6.5);
            for (x, h) in [(6.0, 4.0), (10.0, 7.0), (14.0, 5.5)] {
                pen.fill_rect(x, 16.0 - h, 2.6, h, a);
            }
            pen.rect(13.0, 13.0, 9.0, 9.0, pen.tint(ICE));
        }
        Hyperlink => {
            pen.globe(9.0, 9.0, 7.0, b);
            pen.chain(16.5, 16.5, a);
        }
        DataLink => {
            pen.grid(2.0, 3.0, 14.0, 12.0, 2, 3, b);
            pen.chain(16.5, 16.5, a);
        }
        DataDownload | DataUpload => {
            pen.grid(2.0, 3.0, 13.0, 14.0, 2, 3, b);
            let (from, to) = if icon == DataDownload { (9.0, 21.5) } else { (21.5, 9.0) };
            pen.arrow(19.5, from, 19.5, to, a);
        }
        DataExtract => {
            pen.grid(8.0, 8.0, 14.0, 14.0, 2, 3, b);
            pen.page(2.0, 2.0, 9.0, 11.0, b);
            pen.arrow(6.0, 15.0, 6.0, 21.0, a);
            pen.arrow(6.0, 21.0, 9.5, 21.0, a);
        }
        GeoMap => {
            pen.globe(11.0, 13.0, 8.5, b);
            pen.map_pin(17.0, 13.0);
        }
        GeoFile => {
            pen.page(2.0, 2.0, 14.0, 19.0, b);
            pen.l(5.0, 8.0, 12.0, 8.0);
            pen.l(5.0, 12.0, 10.0, 12.0);
            pen.map_pin(17.5, 21.0);
        }
        _ => return false,
    }
    true
}
