//! Icons of the ribbon's variants and panel tools (original work, drawn in code like the
//! rest of the set). Same 24×24 grid and colour roles as `icons.rs`: light strokes for existing
//! geometry, blue for what the tool creates, orange for picked points.
//!
//! Families share a motif so their variants read as one set: the Circle/Arc variants mark which
//! points are picked (a ring marks the first pick), the layer tools add a state badge to the
//! layer stack, the edit tools add a pencil, the style tools a brush, the clipboard tools a badge
//! over the clipboard.

mod annotate;
mod appmenu;
mod draw;
mod express;
mod insert;
mod manage;
mod output;
mod palette;
mod panels;
mod parametric;
mod view;

use egui::{Color32, Shape, Stroke};

use super::{Icon, Pen};
use crate::theme::Tokens;

const YELLOW: Color32 = Color32::from_rgb(0xf5, 0xd0, 0x3b);
const GOLD: Color32 = Color32::from_rgb(0xe6, 0xb4, 0x3c);
const ICE: Color32 = Color32::from_rgb(0x9f, 0xd4, 0xff);
const GREEN: Color32 = Color32::from_rgb(0x6f, 0xe0, 0x6f);

/// Paint one of the ribbon icons `icons::paint` hands over: each tab's module paints its own
/// and says false for the others; the Home panel tools come last.
pub(super) fn paint(pen: &Pen, icon: Icon) {
    if !(draw::paint(pen, icon)
        || insert::paint(pen, icon)
        || annotate::paint(pen, icon)
        || parametric::paint(pen, icon)
        || view::paint(pen, icon)
        || manage::paint(pen, icon)
        || output::paint(pen, icon)
        || express::paint(pen, icon)
        || palette::paint(pen, icon)
        || appmenu::paint(pen, icon))
    {
        panels::paint(pen, icon);
    }
}

type P = (f32, f32);

fn bez(p: [P; 4], t: f32) -> P {
    let u = 1.0 - t;
    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    (a * p[0].0 + b * p[1].0 + c * p[2].0 + d * p[3].0, a * p[0].1 + b * p[1].1 + c * p[2].1 + d * p[3].1)
}

/// A point of an ellipse (centre, semi-axes, axis rotation and parameter in degrees, CCW on screen).
fn ell_at(cx: f32, cy: f32, a: f32, b: f32, rot: f32, t: f32) -> P {
    let (st, ct) = t.to_radians().sin_cos();
    let (sr, cr) = rot.to_radians().sin_cos();
    (cx + a * ct * cr - b * st * sr, cy - (a * ct * sr + b * st * cr))
}

impl Pen<'_> {
    fn tint(&self, c: Color32) -> Color32 {
        if self.dim { c.gamma_multiply(0.4) } else { c }
    }
    /// Line from (x1, y1) ending in an arrow tip at (x2, y2).
    fn arrow(&self, x1: f32, y1: f32, x2: f32, y2: f32, c: Color32) {
        let (dx, dy) = (x2 - x1, y2 - y1);
        let l = (dx * dx + dy * dy).sqrt().max(1e-3);
        self.poly(&[(x1, y1), (x2 - dx / l * 2.0, y2 - dy / l * 2.0)], false, c);
        self.arrow_tip(x2, y2, dx, dy, c);
    }
    fn dash(&self, x1: f32, y1: f32, x2: f32, y2: f32, c: Color32) {
        let (dx, dy) = (x2 - x1, y2 - y1);
        let n = ((dx * dx + dy * dy).sqrt() / 3.0).round().max(1.0);
        for i in 0..n as usize {
            let (t0, t1) = (i as f32 / n, (i as f32 + 0.6) / n);
            self.poly(&[(x1 + dx * t0, y1 + dy * t0), (x1 + dx * t1, y1 + dy * t1)], false, c);
        }
    }
    fn dash_rect(&self, x: f32, y: f32, w: f32, h: f32, c: Color32) {
        self.dash(x, y, x + w, y, c);
        self.dash(x + w, y, x + w, y + h, c);
        self.dash(x + w, y + h, x, y + h, c);
        self.dash(x, y + h, x, y, c);
    }
    fn curve(&self, p: [P; 4], c: Color32) {
        let pts: Vec<P> = (0..=20).map(|i| bez(p, i as f32 / 20.0)).collect();
        self.poly(&pts, false, c);
    }
    fn ell(&self, cx: f32, cy: f32, a: f32, b: f32, rot: f32, t0: f32, t1: f32, c: Color32) {
        let n = (((t1 - t0).abs() / 10.0).ceil() as usize).max(2);
        let pts: Vec<P> = (0..=n).map(|i| ell_at(cx, cy, a, b, rot, t0 + (t1 - t0) * i as f32 / n as f32)).collect();
        self.poly(&pts, false, c);
    }
    fn fill_poly(&self, pts: &[P], c: Color32) {
        self.p.add(Shape::convex_polygon(pts.iter().map(|(x, y)| self.at(*x, *y)).collect(), c, Stroke::NONE));
    }
    fn thick(&self, x1: f32, y1: f32, x2: f32, y2: f32, w: f32, c: Color32) {
        self.p.line_segment([self.at(x1, y1), self.at(x2, y2)], Stroke::new(w * self.sc(), c));
    }
    /// Blank out a rectangle with the panel colour, so a badge sits in front of the motif.
    fn knock(&self, x: f32, y: f32, w: f32, h: f32) {
        self.fill_rect(x, y, w, h, Tokens::get().panel);
    }
    /// A small cross marking a point object.
    fn mark(&self, x: f32, y: f32) {
        self.la(x - 1.6, y - 1.6, x + 1.6, y + 1.6);
        self.la(x - 1.6, y + 1.6, x + 1.6, y - 1.6);
    }
    /// The first picked point: a dot inside a ring.
    fn first(&self, x: f32, y: f32) {
        self.c(x, y, 2.5, self.pt);
        self.dot(x, y);
    }
    /// Revision-cloud bumps along `pts` (listed clockwise on screen, so the bumps face out).
    fn cloud(&self, pts: &[P], closed: bool, bump: f32, c: Color32) {
        let n = if closed { pts.len() } else { pts.len() - 1 };
        for i in 0..n {
            let (a, b) = (pts[i], pts[(i + 1) % pts.len()]);
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let k = ((dx * dx + dy * dy).sqrt() / bump).round().max(1.0);
            for j in 0..k as usize {
                let (t0, t1) = (j as f32 / k, (j as f32 + 1.0) / k);
                let (sx, sy, ex, ey) = (a.0 + dx * t0, a.1 + dy * t0, a.0 + dx * t1, a.1 + dy * t1);
                let r = ((ex - sx).powi(2) + (ey - sy).powi(2)).sqrt() / 2.0;
                let phi = (-(ey - sy)).atan2(ex - sx).to_degrees();
                self.arc((sx + ex) / 2.0, (sy + ey) / 2.0, r, phi, phi + 180.0, c);
            }
        }
    }
    /// The edit badge: a pencil in the bottom-right corner, writing towards bottom-left.
    fn pencil(&self) {
        self.knock(13.0, 12.0, 11.0, 12.0);
        self.thick(22.5, 13.0, 16.4, 19.1, 3.0, self.pt);
        self.tri([(15.0, 17.7), (17.8, 20.5), (13.0, 22.5)], self.base);
    }
    /// The style badge: a brush in the bottom-right corner.
    fn brush(&self) {
        self.knock(12.0, 12.0, 12.0, 12.0);
        self.thick(23.0, 12.5, 18.6, 16.9, 1.6, self.base);
        self.fill_poly(&[(17.0, 15.3), (20.2, 18.5), (17.6, 21.6), (13.0, 22.6), (14.2, 18.2)], self.pt);
    }
    /// The layer stack of the layer tools (same as `LayerOff` …), its badge goes at (18, 17).
    fn layers(&self, c: Color32) {
        self.poly(&[(2.0, 9.0), (10.0, 5.0), (18.0, 9.0), (10.0, 13.0)], true, c);
        self.poly(&[(2.0, 13.0), (10.0, 17.0), (18.0, 13.0)], false, c);
    }
    /// An isometric block box with its top-left corner at (x, y), `s` = 1 is the Blocks size.
    fn block(&self, x: f32, y: f32, s: f32, c: Color32) {
        let q = |px: f32, py: f32| (x + px * s, y + py * s);
        self.poly(&[q(8.0, 0.0), q(16.0, 4.0), q(8.0, 8.0), q(0.0, 4.0)], true, c);
        self.poly(&[q(0.0, 4.0), q(0.0, 11.0), q(8.0, 15.0), q(16.0, 11.0), q(16.0, 4.0)], false, c);
        let (a, b) = (q(8.0, 8.0), q(8.0, 15.0));
        self.poly(&[a, b], false, c);
    }
    /// Two overlapping squares: `back` partly hidden behind `front`.
    fn overlap(&self, back: Color32, front: Color32) {
        self.poly(&[(12.0, 7.0), (12.0, 2.0), (2.0, 2.0), (2.0, 12.0), (7.0, 12.0)], false, back);
        self.fill_rect(7.0, 7.0, 10.0, 10.0, front.gamma_multiply(0.3));
        self.rect(7.0, 7.0, 10.0, 10.0, front);
    }
    /// The draw-order arrow at the right: up or down, with a bar when it goes all the way.
    fn order_arrow(&self, up: bool, to_end: bool) {
        let (from, to, bar) = if up { (21.0, 5.0, 2.5) } else { (3.0, 19.0, 21.5) };
        self.arrow(20.5, from, 20.5, to, self.acc);
        if to_end {
            self.thick(17.5, bar, 23.5, bar, 1.6, self.acc);
        }
    }
    /// The clipboard of the clipboard tools; badges go in the bottom-right corner.
    fn clipboard(&self) {
        self.rect(3.0, 4.0, 13.0, 17.0, self.base);
        self.fill_rect(6.5, 2.5, 6.0, 3.5, self.base);
    }
    /// The group motif: a circle and a square inside a dashed frame.
    fn group(&self, frame: Color32) {
        self.c(7.5, 9.0, 3.5, self.base);
        self.rect(10.5, 10.0, 7.0, 7.0, self.base);
        self.dash_rect(2.0, 3.0, 18.0, 16.0, frame);
    }
    /// A small mouse pointer with its tip at (x, y).
    fn pointer(&self, x: f32, y: f32) {
        let pts = [(0.0, 0.0), (0.0, 9.0), (2.2, 7.0), (4.0, 10.3), (5.4, 9.6), (3.6, 6.4), (6.6, 6.2)];
        let v: Vec<P> = pts.iter().map(|(px, py)| (x + px, y + py)).collect();
        self.fill_poly(&[v[0], v[1], v[2], v[6]], Tokens::get().panel);
        self.poly(&v, true, self.base);
    }
}
