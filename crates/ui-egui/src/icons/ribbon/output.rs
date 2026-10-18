//! Output and Collaborate tab tools: plot details, the plotter manager, DWF/PDF export and its
//! options, and the online collaboration tools. Shared motifs: a page for sheets, a badge with
//! the format's letters, a small cog for options.

use egui::Color32;

use super::GREEN;
use crate::icons::{Icon, Pen};
use crate::theme::Tokens;

impl Pen<'_> {
    /// A coloured badge with `text` in the panel colour, top-left at (x, y).
    fn badge(&self, x: f32, y: f32, w: f32, h: f32, text: &str, c: Color32) {
        self.fill_rect(x, y, w, h, c);
        self.text(x + w / 2.0, y + h / 2.0, text, h * 0.78, Tokens::get().panel);
    }
    /// An options cog of radius `r` centred at (x, y).
    fn cog(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.c(x, y, r * 0.55, c);
        for k in 0..6 {
            let (s, co) = (k as f32 * 60.0_f32).to_radians().sin_cos();
            self.thick(x + co * r * 0.6, y + s * r * 0.6, x + co * r, y + s * r, 1.8, c);
        }
    }
    /// A small printer, top-left at (x, y), 14 wide.
    fn printer(&self, x: f32, y: f32, c: Color32) {
        self.rect(x + 3.0, y, 8.0, 4.0, c);
        self.rect(x, y + 4.0, 14.0, 7.0, c);
        self.rect(x + 3.0, y + 8.0, 8.0, 5.0, c);
    }
}

/// Output and Collaborate tab icons; false when `icon` belongs to another tab.
pub(super) fn paint(pen: &Pen, icon: Icon) -> bool {
    let (b, a) = (pen.base, pen.acc);
    use Icon::*;
    match icon {
        ViewDetails => {
            pen.page(3.0, 2.0, 13.0, 19.0, b);
            for y in [8.0, 11.5, 15.0] {
                pen.l(5.5, y, 11.5, y);
            }
            pen.knock(13.0, 12.0, 11.0, 11.0);
            pen.c(18.0, 17.5, 4.5, a);
            pen.thick(18.0, 16.5, 18.0, 20.0, 1.6, a);
            pen.fill_c(18.0, 14.8, 0.9, a);
        }
        PlotterManager => {
            pen.printer(2.0, 3.0, b);
            pen.poly(&[(12.0, 14.0), (12.0, 22.0), (23.0, 22.0), (23.0, 15.5), (17.5, 15.5), (16.0, 14.0)], true, a);
        }
        ExportDwfx | ExportDwf | ExportPdf => {
            pen.page(4.0, 2.0, 15.0, 20.0, b);
            let (text, c) = match icon {
                ExportDwfx => ("DWFx", a),
                ExportDwf => ("DWF", a),
                _ => ("PDF", pen.pt),
            };
            pen.badge(1.0, 12.0, 22.0, 7.0, text, c);
        }
        ExportWindow => {
            pen.dash_rect(2.0, 2.0, 16.0, 13.0, a);
            pen.pointer(12.0, 10.0);
        }
        ExportPageSetup => {
            pen.page(2.0, 2.0, 13.0, 18.0, b);
            pen.knock(11.0, 11.0, 13.0, 13.0);
            pen.cog(17.5, 17.5, 5.0, a);
        }
        ExportPreview => {
            pen.page(2.0, 2.0, 13.0, 18.0, b);
            pen.badge(4.0, 5.0, 9.0, 4.0, "PDF", pen.pt);
            pen.knock(12.0, 11.0, 12.0, 13.0);
            pen.c(16.0, 16.0, 4.0, a);
            pen.thick(19.0, 19.0, 22.5, 22.5, 2.0, a);
        }
        DwfOptions | PdfOptions => {
            let (text, c) = if icon == DwfOptions { ("DWF", a) } else { ("PDF", pen.pt) };
            pen.page(2.0, 2.0, 14.0, 19.0, b);
            pen.badge(1.0, 6.0, 15.0, 6.0, text, c);
            pen.knock(13.0, 12.0, 11.0, 12.0);
            pen.cog(18.0, 17.5, 4.5, b);
        }
        SharedViews => {
            pen.block(2.0, 3.0, 0.9, b);
            let g = pen.tint(GREEN);
            pen.poly(&[(20.5, 13.0), (15.0, 17.0), (20.5, 21.0)], false, g);
            for (x, y) in [(20.5, 13.0), (15.0, 17.0), (20.5, 21.0)] {
                pen.fill_c(x, y, 2.0, g);
            }
        }
        PushToDocs => {
            pen.globe(10.5, 13.0, 8.5, b);
            pen.knock(16.0, 2.0, 8.0, 12.0);
            pen.arrow(20.0, 13.0, 20.0, 2.0, a);
        }
        TracesPalette => {
            pen.rect(5.0, 6.0, 17.0, 14.0, b);
            pen.fill_rect(2.0, 3.0, 17.0, 14.0, Tokens::get().panel);
            pen.rect(2.0, 3.0, 17.0, 14.0, b);
            pen.curve([(4.0, 12.0), (8.0, 5.0), (12.0, 16.0), (17.0, 8.0)], a);
        }
        MarkupImport => {
            pen.cloud(&[(2.0, 4.0), (14.0, 4.0), (14.0, 12.0), (2.0, 12.0)], true, 4.0, pen.pt);
            pen.poly(&[(9.0, 16.0), (9.0, 22.0), (23.0, 22.0), (23.0, 16.0)], false, b);
            pen.arrow(17.0, 6.0, 17.0, 19.0, a);
        }
        _ => return false,
    }
    true
}
