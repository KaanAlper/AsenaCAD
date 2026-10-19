//! The colour, linetype and lineweight pickers of the Layer Properties Manager (the colour
//! button is shared by Quick Properties and the Hatch dialog).

use asenacad_color::{Color, aci_rgb};
use asenacad_doc::Lineweight;
use egui::{Color32, Rect, RichText, Sense, Stroke, pos2, vec2};

use crate::theme::Tokens;

/// `text` laid out on one line, cut short with "…" past `width`.
pub(super) fn elided(ui: &egui::Ui, text: &str, width: f32, color: Color32) -> std::sync::Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_string(), crate::theme::body(), color);
    job.wrap = egui::text::TextWrapping::truncate_at_width(width.max(4.0));
    ui.painter().layout_job(job)
}

fn rgb32(c: Color) -> Color32 {
    let r = c.resolve(Color::Index(7), Color::Index(7));
    Color32::from_rgb(r.0, r.1, r.2)
}

/// A colour swatch.
pub fn swatch(p: &egui::Painter, r: Rect, c: Color) {
    p.rect_filled(r, 1.0, rgb32(c));
    p.rect_stroke(r, 1.0, Stroke::new(1.0, Color32::from_gray(25)), egui::StrokeKind::Inside);
}

/// A colour button (swatch + name) that opens the Select Color popup: the AutoCAD Color Index
/// palette and a true-colour editor. Returns the picked colour. `by` adds ByLayer/ByBlock.
pub fn color_button(ui: &mut egui::Ui, id: egui::Id, current: Color, by: bool, width: f32) -> Option<Color> {
    color_button_sized(ui, id, current, by, vec2(width, 20.0))
}

/// [`color_button`] of a given size; a name too long for it is cut short with "…".
pub fn color_button_sized(ui: &mut egui::Ui, id: egui::Id, current: Color, by: bool, size: egui::Vec2) -> Option<Color> {
    let t = Tokens::get();
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let p = ui.painter();
    if resp.hovered() {
        p.rect_filled(rect, 2.0, t.control_hover.gamma_multiply(0.5));
    }
    let side = (size.y - 6.0).clamp(6.0, 12.0);
    swatch(p, Rect::from_min_size(pos2(rect.left() + 3.0, rect.center().y - side / 2.0), vec2(side, side)), current);
    let name = crate::i18n::t(&current.name()).to_string();
    let text = rect.left() + side + 7.0;
    let galley = elided(ui, &name, rect.right() - text - 2.0, t.text);
    ui.painter().galley(pos2(text, rect.center().y - galley.size().y / 2.0), galley, t.text);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::ColorButton, true, &name));
    let mut out = None;
    egui::Popup::from_toggle_button_response(&resp).id(id).width(300.0).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
        out = color_palette(ui, id, current, by);
        if out.is_some() {
            ui.close();
        }
    });
    out
}

/// The palette body (index colours + true colour).
pub fn color_palette(ui: &mut egui::Ui, id: egui::Id, current: Color, by: bool) -> Option<Color> {
    let t = Tokens::get();
    let tab_id = id.with("tab");
    let mut tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id)).unwrap_or(if matches!(current, Color::True(_)) { 1 } else { 0 });
    let mut out = None;
    ui.horizontal(|ui| {
        for (i, l) in ["Index Color", "True Color"].iter().enumerate() {
            if ui.selectable_label(tab == i as u8, crate::i18n::t(l)).clicked() {
                tab = i as u8;
            }
        }
    });
    ui.data_mut(|d| d.insert_temp(tab_id, tab));
    ui.separator();
    let cell = |ui: &mut egui::Ui, i: u8, size: f32, out: &mut Option<Color>| {
        let (r, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
        let c = aci_rgb(i);
        ui.painter().rect_filled(r.shrink(0.5), 0.0, Color32::from_rgb(c.0, c.1, c.2));
        let sel = current == Color::Index(i);
        if sel || resp.hovered() {
            ui.painter().rect_stroke(r, 0.0, Stroke::new(if sel { 2.0 } else { 1.0 }, Color32::WHITE), egui::StrokeKind::Inside);
        }
        if resp.on_hover_text(crate::tf!("Index color: {index}", index = i)).clicked() {
            *out = Some(Color::Index(i));
        }
    };
    if tab == 0 {
        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
        // The 240 graded colours: 24 hues × 10 shades.
        for shade in [0u8, 2, 4, 6, 8, 1, 3, 5, 7, 9] {
            ui.horizontal(|ui| {
                for hue in 0..24u8 {
                    cell(ui, 10 + hue * 10 + shade, 11.0, &mut out);
                }
            });
        }
        ui.add_space(6.0);
        ui.horizontal(|ui| {
            for i in 1..=9u8 {
                cell(ui, i, 18.0, &mut out);
                ui.add_space(2.0);
            }
            ui.add_space(10.0);
            for i in 250..=255u8 {
                cell(ui, i, 18.0, &mut out);
                ui.add_space(2.0);
            }
        });
        ui.spacing_mut().item_spacing = vec2(6.0, 4.0);
        ui.add_space(6.0);
        if by {
            ui.horizontal(|ui| {
                if ui.button(crate::tl!("ByLayer")).clicked() {
                    out = Some(Color::ByLayer);
                }
                if ui.button(crate::tl!("ByBlock")).clicked() {
                    out = Some(Color::ByBlock);
                }
            });
        }
        ui.label(RichText::new(crate::tf!("Current: {color}", color = crate::i18n::t(&current.name()))).small().color(t.text_dim));
    } else {
        let rid = id.with("rgb");
        let start = current.resolve(Color::Index(7), Color::Index(7));
        let mut c = ui.data_mut(|d| d.get_temp::<[u8; 3]>(rid)).unwrap_or([start.0, start.1, start.2]);
        let mut c32 = Color32::from_rgb(c[0], c[1], c[2]);
        egui::color_picker::color_picker_color32(ui, &mut c32, egui::color_picker::Alpha::Opaque);
        c = [c32.r(), c32.g(), c32.b()];
        ui.horizontal(|ui| {
            for (i, l) in ["R", "G", "B"].iter().enumerate() {
                ui.label(*l);
                if let Some(v) = c.get_mut(i) {
                    ui.add(egui::DragValue::new(v).range(0..=255));
                }
            }
        });
        ui.data_mut(|d| d.insert_temp(rid, c));
        if ui.button(crate::tl!("Apply true color")).clicked() {
            out = Some(Color::True(asenacad_color::Rgb(c[0], c[1], c[2])));
            ui.data_mut(|d| d.remove::<[u8; 3]>(rid));
        }
    }
    out
}

/// A small horizontal bar showing a lineweight.
pub(super) fn lw_sample(p: &egui::Painter, r: Rect, lw: Lineweight, c: Color32) {
    let w = match lw {
        Lineweight::Mm100(v) => (f32::from(v) / 100.0 * 3.0).clamp(1.0, 7.0),
        _ => 1.0,
    };
    p.line_segment([pos2(r.left(), r.center().y), pos2(r.right(), r.center().y)], Stroke::new(w, c));
}

/// A linetype preview drawn from its own dash pattern.
pub(super) fn lt_sample(p: &egui::Painter, r: Rect, lt: Option<&asenacad_doc::Linetype>, c: Color32) {
    let y = r.center().y;
    let Some(lt) = lt.filter(|l| !l.pattern.is_empty() && l.pattern.iter().any(|e| e.length.abs() > 1e-9)) else {
        p.line_segment([pos2(r.left(), y), pos2(r.right(), y)], Stroke::new(1.0, c));
        return;
    };
    let total: f64 = lt.pattern.iter().map(|e| e.length.abs()).sum::<f64>().max(1e-9);
    let k = f64::from(r.width()) / 2.0 / total;
    let mut x = f64::from(r.left());
    let mut guard = 0;
    while x < f64::from(r.right()) && guard < 200 {
        for e in &lt.pattern {
            let l = e.length.abs() * k;
            let x1 = (x + l).min(f64::from(r.right()));
            if e.length > 0.0 {
                p.line_segment([pos2(x as f32, y), pos2(x1 as f32, y)], Stroke::new(1.0, c));
            } else if e.length == 0.0 {
                p.circle_filled(pos2(x as f32, y), 0.8, c);
            }
            x = x1.max(x + 1.0);
            guard += 1;
        }
    }
}
