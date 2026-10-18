//! The pen the icons draw with: coordinates on the 24×24 design grid, scaled to the icon's rect.

use egui::{Color32, Painter, Pos2, Rect, Stroke, pos2};

/// Draws on the 24×24 icon grid inside `r`, with the icon's line width and colours.
pub(super) struct Pen<'a> {
    pub(super) p: &'a Painter,
    pub(super) r: Rect,
    pub(super) w: f32,
    pub(super) base: Color32,
    pub(super) acc: Color32,
    pub(super) pt: Color32,
    pub(super) dim: bool,
}

impl Pen<'_> {
    pub(super) fn at(&self, x: f32, y: f32) -> Pos2 {
        let s = self.r.width() / 24.0;
        pos2(self.r.min.x + x * s, self.r.min.y + y * s)
    }
    pub(super) fn sc(&self) -> f32 {
        self.r.width() / 24.0
    }
    pub(super) fn st(&self, c: Color32) -> Stroke {
        Stroke::new(self.w, c)
    }
    pub(super) fn l(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.p.line_segment([self.at(x1, y1), self.at(x2, y2)], self.st(self.base));
    }
    pub(super) fn la(&self, x1: f32, y1: f32, x2: f32, y2: f32) {
        self.p.line_segment([self.at(x1, y1), self.at(x2, y2)], self.st(self.acc));
    }
    pub(super) fn poly(&self, pts: &[(f32, f32)], closed: bool, c: Color32) {
        let mut v: Vec<Pos2> = pts.iter().map(|(x, y)| self.at(*x, *y)).collect();
        if closed && let Some(f) = v.first().copied() {
            v.push(f);
        }
        self.p.line(v, self.st(c));
    }
    pub(super) fn c(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.p.circle_stroke(self.at(x, y), r * self.sc(), self.st(c));
    }
    pub(super) fn fill_c(&self, x: f32, y: f32, r: f32, c: Color32) {
        self.p.circle_filled(self.at(x, y), r * self.sc(), c);
    }
    /// Arc in degrees, counter-clockwise on screen (y up visually).
    pub(super) fn arc(&self, x: f32, y: f32, r: f32, a0: f32, a1: f32, c: Color32) {
        let n = (((a1 - a0).abs() / 10.0).ceil() as usize).max(2);
        let pts: Vec<Pos2> = (0..=n)
            .map(|i| {
                let a = (a0 + (a1 - a0) * i as f32 / n as f32).to_radians();
                self.at(x + r * a.cos(), y - r * a.sin())
            })
            .collect();
        self.p.line(pts, self.st(c));
    }
    pub(super) fn rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.poly(&[(x, y), (x + w, y), (x + w, y + h), (x, y + h)], true, c);
    }
    pub(super) fn fill_rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.p.rect_filled(Rect::from_min_max(self.at(x, y), self.at(x + w, y + h)), 0.0, c);
    }
    pub(super) fn dot(&self, x: f32, y: f32) {
        self.fill_c(x, y, 1.6, self.pt);
    }
    pub(super) fn tri(&self, pts: [(f32, f32); 3], c: Color32) {
        self.p.add(egui::Shape::convex_polygon(pts.iter().map(|(x, y)| self.at(*x, *y)).collect(), c, Stroke::NONE));
    }
    pub(super) fn arrow_tip(&self, x: f32, y: f32, dx: f32, dy: f32, c: Color32) {
        let l = (dx * dx + dy * dy).sqrt().max(1e-3);
        let (ux, uy) = (dx / l, dy / l);
        let (nx, ny) = (-uy, ux);
        self.tri([(x, y), (x - ux * 3.0 + nx * 1.4, y - uy * 3.0 + ny * 1.4), (x - ux * 3.0 - nx * 1.4, y - uy * 3.0 - ny * 1.4)], c);
    }
    pub(super) fn text(&self, x: f32, y: f32, s: &str, size: f32, c: Color32) {
        self.p.text(self.at(x, y), egui::Align2::CENTER_CENTER, s, egui::FontId::proportional(size * self.sc()), c);
    }
}
