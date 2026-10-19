//! The Properties palette with objects selected: General for every selection, then, for one
//! object, its kind's groups with AutoCAD's group and row names (Geometry, Text, Pattern, Misc,
//! Custom). Edits run `properties.set` (dynamic block properties `blocks.reference.set`).

use asenacad_doc::EntityKind;
use serde_json::{Value, json};

use super::super::patterns;
use super::{COLORS, Rows, edit_field, menu, number, pick_menu, read_only, read_only_row, report_bad, row};
use crate::CadApp;

type Custom = Option<(String, Vec<asenacad_engine::doc::dynblock::CustomProperty>)>;

/// The dynamic block name and custom properties of block reference `h`, worked out once per
/// drawing revision.
fn custom_properties(ui: &egui::Ui, app: &CadApp, h: asenacad_engine::doc::Handle) -> Custom {
    let st = app.session.state().ok()?;
    let id = egui::Id::new(("dynamic-block-custom", st.uid, st.revision, h));
    if let Some(c) = ui.data(|d| d.get_temp::<Custom>(id)) {
        return c;
    }
    let c = asenacad_engine::doc::dynblock::custom_properties(&st.doc, h);
    ui.data_mut(|d| d.insert_temp(id, c.clone()));
    c
}

/// A numeric row whose committed value goes to `properties.set` as `key`.
#[allow(clippy::too_many_arguments)]
fn num(ui: &mut egui::Ui, label: &str, key: &str, shown: String, set: &mut Option<Value>, bad: &mut Option<String>, ids: &Vec<String>) {
    row(ui, label, true, &shown, |ui| {
        if let Some(f) = number(edit_field(ui, ui.id().with((key, ids)), shown.clone()), bad) {
            *set = Some(json!({ "handles": ids, key: f }));
        }
    });
}

pub(super) fn show(app: &mut CadApp, ui: &mut egui::Ui) {
    let sel = app.session.selection();
    let custom = match sel.as_slice() {
        [h] => custom_properties(ui, app, *h),
        _ => None,
    };
    let Ok(d) = app.session.doc() else { return };
    let h = &d.header;
    let (lu, lp) = (h.i64("LUNITS", 2), h.i64("LUPREC", 4));
    let len = |v: f64| asenacad_engine::units::format_distance(v, lu, lp);
    let real = |v: f64| format!("{v:.4}");
    let mut bad: Option<String> = None;
    let first = sel.first().and_then(|h| d.entity(*h)).map(|e| (**e).clone());
    let Some(e) = first else { return };
    let same = |f: &dyn Fn(&asenacad_doc::Entity) -> String| -> String {
        let vals: Vec<String> = sel.iter().filter_map(|h| d.entity(*h)).map(|e| f(e)).collect();
        match vals.first() {
            Some(v0) if vals.iter().all(|v| v == v0) => v0.clone(),
            _ => "*VARIES*".into(),
        }
    };
    let shown = |s: &str| if ["ByLayer", "ByBlock", "*VARIES*"].contains(&s) { crate::i18n::t(s).to_string() } else { s.to_string() };
    let color = shown(&same(&|e| e.common.color.name()));
    let layer = same(&|e| e.common.layer.clone());
    let linetype = same(&|e| e.common.linetype.clone());
    let lts = same(&|e| format!("{:.4}", e.common.ltscale));
    let lw = shown(&same(&|e| e.common.lineweight.name()));
    let tr = same(&|e| e.common.transparency.name());
    let ids: Vec<String> = sel.iter().map(|h| h.hex()).collect();
    let mut set: Option<Value> = None;
    let mut custom_set: Option<Value> = None;
    let collapsed = &mut app.ui.collapsed_groups;
    let mut rows = Rows::new();
    if rows.group(ui, collapsed, "General") {
        row(ui, "Color", true, &color, |ui| {
            menu(ui, &color, |ui| {
                for (n, c) in COLORS.into_iter().chain([("Color 8", 8), ("Color 9", 9)]) {
                    if ui.button(crate::i18n::t(n)).clicked() {
                        set = Some(json!({ "handles": ids, "color": c }));
                        ui.close();
                    }
                }
            });
        });
        row(ui, "Layer", true, &layer, |ui| {
            if let Some(l) = pick_menu(ui, &layer, d.layers.iter().map(|l| l.name.clone())) {
                set = Some(json!({ "handles": ids, "layer": l }));
            }
        });
        row(ui, "Linetype", true, &shown(&linetype), |ui| {
            let items = ["ByLayer", "ByBlock"]
                .map(String::from)
                .into_iter()
                .chain(d.linetypes.iter().map(|l| l.name.clone()).filter(|l| !["ByLayer", "ByBlock"].contains(&l.as_str())));
            if let Some(l) = pick_menu(ui, &linetype, items) {
                set = Some(json!({ "handles": ids, "linetype": l }));
            }
        });
        row(ui, "Linetype scale", true, &lts, |ui| {
            if let Some(f) = number(edit_field(ui, ui.id().with(("lts", &ids)), lts.clone()), &mut bad) {
                set = Some(json!({ "handles": ids, "ltscale": f }));
            }
        });
        row(ui, "Lineweight", true, &lw, |ui| {
            menu(ui, &lw, |ui| {
                for n in ["ByLayer", "ByBlock", "Default"] {
                    if ui.button(crate::i18n::t(n)).clicked() {
                        set = Some(json!({ "handles": ids, "lineweight": n }));
                        ui.close();
                    }
                }
                for v in asenacad_doc::Lineweight::STANDARD {
                    if ui.button(format!("{:.2} mm", f64::from(v) / 100.0)).clicked() {
                        set = Some(json!({ "handles": ids, "lineweight": f64::from(v) / 100.0 }));
                        ui.close();
                    }
                }
            });
        });
        row(ui, "Transparency", true, &shown(&tr), |ui| {
            menu(ui, &shown(&tr), |ui| {
                for v in ["ByLayer", "ByBlock"] {
                    if ui.button(crate::i18n::t(v)).clicked() {
                        set = Some(json!({ "handles": ids, "transparency": v }));
                        ui.close();
                    }
                }
                ui.separator();
                // A fixed percentage, committed when the drag or edit ends.
                let tid = egui::Id::new(("prop_tr", &ids));
                let cur = tr.parse::<u8>().ok();
                let mut pct = ui.data_mut(|d| d.get_temp::<u8>(tid)).or(cur).unwrap_or(0);
                ui.horizontal(|ui| {
                    ui.label(crate::tl!("Percent"));
                    let r = ui.add(egui::DragValue::new(&mut pct).range(0..=90).speed(0.5));
                    if r.dragged() || r.has_focus() {
                        ui.data_mut(|d| d.insert_temp(tid, pct));
                    }
                    if r.drag_stopped() || r.lost_focus() {
                        if cur != Some(pct) {
                            set = Some(json!({ "handles": ids, "transparency": pct }));
                        }
                        ui.data_mut(|d| d.remove::<u8>(tid));
                    }
                });
            });
        });
    }
    if sel.len() == 1 {
        match &e.kind {
            EntityKind::Line(l) => {
                if rows.group(ui, collapsed, "Geometry") {
                    for (label, key, v, point) in [
                        ("Start X", "sx", l.a.x, "start"),
                        ("Start Y", "sy", l.a.y, "start"),
                        ("End X", "ex", l.b.x, "end"),
                        ("End Y", "ey", l.b.y, "end"),
                    ] {
                        let shown_v = len(v);
                        row(ui, label, true, &shown_v, |ui| {
                            if let Some(f) = number(edit_field(ui, ui.id().with((key, &ids)), shown_v.clone()), &mut bad) {
                                let p = if point == "start" { l.a } else { l.b };
                                let xy = if key.ends_with('x') { [f, p.y] } else { [p.x, f] };
                                set = Some(json!({ "handles": ids, point: xy }));
                            }
                        });
                    }
                    let dlt = l.b.xy() - l.a.xy();
                    read_only_row(ui, "Delta X", &len(dlt.x));
                    read_only_row(ui, "Delta Y", &len(dlt.y));
                    // Length and angle keep the start point and move the end point.
                    num(ui, "Length", "length", len(dlt.len()), &mut set, &mut bad, &ids);
                    num(ui, "Angle", "angle", real(dlt.angle().to_degrees()), &mut set, &mut bad, &ids);
                }
            }
            EntityKind::Circle(c) => {
                if rows.group(ui, collapsed, "Geometry") {
                    num(ui, "Center X", "cx", len(c.center.x), &mut set, &mut bad, &ids);
                    num(ui, "Center Y", "cy", len(c.center.y), &mut set, &mut bad, &ids);
                    num(ui, "Radius", "radius", len(c.radius), &mut set, &mut bad, &ids);
                    num(ui, "Diameter", "diameter", len(c.radius * 2.0), &mut set, &mut bad, &ids);
                    num(ui, "Circumference", "circumference", len(c.radius * std::f64::consts::TAU), &mut set, &mut bad, &ids);
                    num(ui, "Area", "area", real(c.radius * c.radius * std::f64::consts::PI), &mut set, &mut bad, &ids);
                    if let Some(v) = &mut set
                        && let Some(o) = v.as_object_mut()
                    {
                        if let Some(x) = o.remove("cx") {
                            o.insert("center".into(), json!([x, c.center.y]));
                        }
                        if let Some(y) = o.remove("cy") {
                            o.insert("center".into(), json!([c.center.x, y]));
                        }
                    }
                }
            }
            EntityKind::Arc(a) => {
                if rows.group(ui, collapsed, "Geometry") {
                    num(ui, "Radius", "radius", len(a.radius), &mut set, &mut bad, &ids);
                    num(ui, "Start angle", "startAngle", real(a.start.to_degrees()), &mut set, &mut bad, &ids);
                    num(ui, "End angle", "endAngle", real(a.end.to_degrees()), &mut set, &mut bad, &ids);
                }
            }
            EntityKind::Text(tx) => {
                if rows.group(ui, collapsed, "Text") {
                    row(ui, "Contents", true, &tx.value, |ui| {
                        if let Some(s) = edit_field(ui, ui.id().with(("txt", &ids)), tx.value.clone()) {
                            set = Some(json!({ "handles": ids, "text": s }));
                        }
                    });
                    num(ui, "Height", "height", len(tx.height), &mut set, &mut bad, &ids);
                    num(ui, "Rotation", "rotation", real(tx.rotation.to_degrees()), &mut set, &mut bad, &ids);
                    num(ui, "Width factor", "widthFactor", real(tx.width_factor), &mut set, &mut bad, &ids);
                }
            }
            EntityKind::MText(tx) => {
                if rows.group(ui, collapsed, "Text") {
                    row(ui, "Contents", true, &tx.contents, |ui| {
                        if let Some(s) = edit_field(ui, ui.id().with(("mtxt", &ids)), tx.contents.clone()) {
                            set = Some(json!({ "handles": ids, "text": s }));
                        }
                    });
                    num(ui, "Text height", "height", len(tx.height), &mut set, &mut bad, &ids);
                    num(ui, "Defined width", "width", len(tx.width), &mut set, &mut bad, &ids);
                }
            }
            EntityKind::LwPolyline(pl) => {
                let g = asenacad_geom::Polyline { vertices: pl.vertices.clone(), closed: pl.closed };
                if rows.group(ui, collapsed, "Geometry") {
                    num(ui, "Global width", "width", len(pl.const_width), &mut set, &mut bad, &ids);
                    if pl.closed {
                        read_only_row(ui, "Area", &format!("{:.4}", g.area().abs()));
                    }
                    read_only_row(ui, "Length", &len(g.len()));
                }
                if rows.group(ui, collapsed, "Misc") {
                    let label = crate::i18n::t(if pl.closed { "Yes" } else { "No" }).to_string();
                    row(ui, "Closed", true, &label, |ui| {
                        menu(ui, &label, |ui| {
                            for (n, v) in [("Yes", true), ("No", false)] {
                                if ui.button(crate::i18n::t(n)).clicked() {
                                    if v != pl.closed {
                                        set = Some(json!({ "handles": ids, "closed": v }));
                                    }
                                    ui.close();
                                }
                            }
                        });
                    });
                }
            }
            EntityKind::Hatch(hh) => {
                if rows.group(ui, collapsed, "Pattern") {
                    row(ui, "Pattern name", true, &hh.pattern, |ui| {
                        menu(ui, &hh.pattern, |ui| {
                            if let Some(name) = patterns::menu(ui, &hh.pattern) {
                                set = Some(json!({ "handles": ids, "pattern": name }));
                                ui.close();
                            }
                        });
                    });
                    num(ui, "Angle", "angle", real(hh.angle.to_degrees()), &mut set, &mut bad, &ids);
                    num(ui, "Scale", "scale", real(hh.scale), &mut set, &mut bad, &ids);
                }
            }
            EntityKind::Dimension(dm) => {
                if rows.group(ui, collapsed, "Misc") {
                    row(ui, "Dim style", true, &dm.style, |ui| {
                        if let Some(n) = pick_menu(ui, &dm.style, d.dim_styles.iter().map(|st| st.name.clone())) {
                            set = Some(json!({ "handles": ids, "dimStyle": n }));
                        }
                    });
                }
                if rows.group(ui, collapsed, "Text") {
                    row(ui, "Text override", true, &dm.text, |ui| {
                        if let Some(s) = edit_field(ui, ui.id().with(("dimtxt", &ids)), dm.text.clone()) {
                            set = Some(json!({ "handles": ids, "textOverride": s }));
                        }
                    });
                }
            }
            EntityKind::Insert(i) => {
                if rows.group(ui, collapsed, "Geometry") {
                    read_only_row(ui, "Position X", &len(i.insert.x));
                    read_only_row(ui, "Position Y", &len(i.insert.y));
                    read_only_row(ui, "Position Z", &len(i.insert.z));
                    num(ui, "Scale", "scale", real(i.scale.x), &mut set, &mut bad, &ids);
                }
                if rows.group(ui, collapsed, "Misc") {
                    // A dynamic block reference shows its dynamic block's name, not its anonymous
                    // block's (`*U12`), and its custom properties below.
                    let name = custom.as_ref().map_or_else(|| i.block.clone(), |(n, _)| n.clone());
                    read_only_row(ui, "Name", &name);
                    num(ui, "Rotation", "rotation", real(i.rotation.to_degrees()), &mut set, &mut bad, &ids);
                }
                if let Some((_, props)) = custom.as_ref().filter(|(_, p)| !p.is_empty())
                    && rows.group(ui, collapsed, "Custom")
                {
                    for p in props {
                        row(ui, &p.name, p.editable, &p.value, |ui| {
                            let picked = match (p.editable, p.choices.is_empty()) {
                                (true, false) => pick_menu(ui, &p.value, p.choices.iter().cloned()),
                                (true, true) => edit_field(ui, ui.id().with(("dynprop", e.handle, &p.name)), p.value.clone()),
                                (false, _) => {
                                    read_only(ui, &p.value);
                                    None
                                }
                            };
                            if let Some(v) = picked.filter(|v| *v != p.value) {
                                custom_set = Some(json!({ "handle": e.handle.hex(), "name": p.name, "value": v }));
                            }
                        });
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(p) = set {
        let _ = app.run("properties.set", p);
    }
    report_bad(app, bad);
    if let Some(p) = custom_set {
        let _ = app.run("blocks.reference.set", p);
    }
}
