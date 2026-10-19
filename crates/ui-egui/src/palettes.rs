//! Tool Sets (left) and the Layers / Properties palettes (right).

use asenacad_color::Color;
use egui::{Color32, Rect, Sense, Stroke, pos2, vec2};
use serde_json::json;

use crate::CadApp;
use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// The hatch pattern picker.
pub mod patterns;
/// The Properties palette.
pub mod properties;
/// The QuickCalc palette.
pub mod quickcalc;
/// The title strip on a palette's left edge.
pub mod strip;

type Tool = (Icon, &'static str, &'static str);

/// Tool Sets groups: (name, large tools, small tools).
pub const DRAFTING: &[(&str, &[Tool], &[Tool])] = &[
    (
        "Draw",
        &[
            (Icon::Line, "line", "Line"),
            (Icon::Polyline, "pline", "Polyline"),
            (Icon::Circle, "circle", "Circle"),
            (Icon::Rectangle, "rectang", "Rectangle"),
        ],
        &[
            (Icon::Arc, "arc", "Arc"),
            (Icon::Spline, "spline", "Spline"),
            (Icon::XLine, "xline", "Construction Line"),
            (Icon::Ray, "ray", "Ray"),
            (Icon::Ellipse, "ellipse", "Ellipse"),
            (Icon::Polygon, "polygon", "Polygon"),
            (Icon::Point, "point.multiple", "Multiple Point"),
            (Icon::Donut, "donut", "Donut"),
            (Icon::RevCloud, "revcloud", "Revision Cloud"),
            (Icon::Region, "region", "Region"),
            (Icon::Wipeout, "wipeout", "Wipeout"),
        ],
    ),
    ("Hatch", &[(Icon::Hatch, "hatch", "Hatch"), (Icon::Gradient, "gradient", "Gradient"), (Icon::Boundary, "boundary", "Boundary")], &[]),
    (
        "Block",
        &[
            (Icon::BlockCreate, "block", "Create Block"),
            (Icon::Insert, "insert", "Insert Block"),
            (Icon::BlockEdit, "bedit", "Block Editor"),
            (Icon::AttDef, "attdef", "Define Attributes"),
        ],
        &[],
    ),
    (
        "Modify",
        &[(Icon::Move, "move", "Move")],
        &[
            (Icon::Copy, "copy", "Copy"),
            (Icon::Rotate, "rotate", "Rotate"),
            (Icon::Trim, "trim", "Trim"),
            (Icon::Fillet, "fillet", "Fillet"),
            (Icon::ArrayRect, "arrayrect", "Rectangular Array"),
            (Icon::Mirror, "mirror", "Mirror"),
            (Icon::Scale, "scale", "Scale"),
            (Icon::Extend, "extend", "Extend"),
            (Icon::Offset, "offset", "Offset"),
            (Icon::ArrayPolar, "arraypolar", "Polar Array"),
            (Icon::Stretch, "stretch", "Stretch"),
            (Icon::Chamfer, "chamfer", "Chamfer"),
            (Icon::Explode, "explode", "Explode"),
            (Icon::Break, "break", "Break"),
            (Icon::Join, "join", "Join"),
            (Icon::Erase, "erase", "Erase"),
            (Icon::Lengthen, "lengthen", "Lengthen"),
            (Icon::MatchProp, "matchprop", "Match Properties"),
        ],
    ),
    (
        "Text",
        &[(Icon::Text, "text", "Single Line Text"), (Icon::MText, "mtext", "Multiline Text")],
        &[(Icon::Table, "table", "Table"), (Icon::Search, "find", "Find and Replace")],
    ),
    (
        "Dimension",
        &[(Icon::DimLinear, "dimlinear", "Linear"), (Icon::DimAligned, "dimaligned", "Aligned")],
        &[
            (Icon::DimRadius, "dimradius", "Radius"),
            (Icon::DimDiameter, "dimdiameter", "Diameter"),
            (Icon::DimAngular, "dimangular", "Angular"),
            (Icon::DimArc, "dimarc", "Arc Length"),
            (Icon::DimOrdinate, "dimordinate", "Ordinate"),
            (Icon::DimBaseline, "dimbaseline", "Baseline"),
            (Icon::DimContinue, "dimcontinue", "Continue"),
            (Icon::QDim, "qdim", "Quick Dimension"),
        ],
    ),
    (
        "Leader",
        &[(Icon::MLeader, "mleader", "Multileader")],
        &[
            (Icon::AddLeader, "mleaderedit", "Add Leader"),
            (Icon::RemoveLeader, "mleaderedit.remove", "Remove Leader"),
            (Icon::AlignLeaders, "mleaderalign", "Align"),
        ],
    ),
    ("Table", &[(Icon::Table, "table", "Table")], &[]),
    (
        "Parametric",
        &[(Icon::Constraint, "autoconstrain", "Auto Constrain")],
        &[
            (Icon::Coincident, "gccoincident", "Coincident"),
            (Icon::Parallel, "gcparallel", "Parallel"),
            (Icon::Perpendicular, "gcperpendicular", "Perpendicular"),
            (Icon::Horizontal, "gchorizontal", "Horizontal"),
            (Icon::Vertical, "gcvertical", "Vertical"),
            (Icon::Tangent, "gctangent", "Tangent"),
            (Icon::Concentric, "gcconcentric", "Concentric"),
            (Icon::Equal, "gcequal", "Equal"),
        ],
    ),
];

pub(crate) fn command_known(id: &str) -> bool {
    asenacad_engine::find_command(id).is_some() || crate::menus::is_ui_command(id)
}

pub fn toolsets(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    egui::Frame::NONE.fill(t.panel).show(ui, |ui| {
        let r = ui.max_rect();
        let p = ui.painter().clone();
        // Tabs.
        let tab_h = 28.0;
        let mut x = r.left();
        for name in ["Drafting", "Modeling"] {
            let w = 92.0;
            let tr = Rect::from_min_size(pos2(x, r.top()), vec2(w, tab_h));
            let active = app.ui.toolset_tab == name;
            let resp = ui.interact(tr, ui.id().with(("ts", name)), Sense::click());
            p.rect_filled(tr, 0.0, if active { t.tab_active } else { t.chrome });
            p.text(
                tr.center(),
                egui::Align2::CENTER_CENTER,
                crate::i18n::t(name),
                egui::FontId::proportional(13.5),
                if active { t.text } else { t.text_dim },
            );
            icons::describe_control(ui, &resp, crate::i18n::t(name), Some(active), true);
            if resp.clicked() {
                app.ui.toolset_tab = name.into();
            }
            x += w;
        }
        p.rect_filled(Rect::from_min_max(pos2(x, r.top()), pos2(r.right(), r.top() + tab_h)), 0.0, t.chrome);
        let cr = Rect::from_center_size(pos2(r.right() - 14.0, r.top() + tab_h / 2.0), vec2(14.0, 14.0));
        let cresp = ui.interact(cr, ui.id().with("ts_collapse"), Sense::click());
        icons::paint(&p, cr, Icon::ChevronLeft, false);
        icons::describe_control(ui, &cresp, "Collapse Tool Sets", None, false);
        if cresp.on_hover_text(crate::tl!("Collapse Tool Sets")).clicked() {
            app.ui.show_toolsets = false;
        }
        let body = Rect::from_min_max(pos2(r.left(), r.top() + tab_h), r.max);
        let mut clicked: Option<&'static str> = None;
        let mut toggle_group = None;
        let mut tool_visible: Option<(String, bool)> = None;
        let mut restore: Vec<String> = Vec::new();
        ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                let groups: &[(&str, &[Tool], &[Tool])] = if app.ui.toolset_tab == "Drafting" { DRAFTING } else { MODELING };
                for (name, large, small) in groups {
                    let collapsed = app.ui.collapsed_groups.iter().any(|g| g == name);
                    // Header.
                    let (hr, hresp) = ui.allocate_exact_size(vec2(ui.available_width(), 22.0), Sense::click());
                    let pp = ui.painter();
                    icons::paint(
                        pp,
                        Rect::from_center_size(pos2(hr.left() + 11.0, hr.center().y), vec2(11.0, 11.0)),
                        if collapsed { Icon::ChevronRight } else { Icon::ChevronDown },
                        false,
                    );
                    pp.text(
                        pos2(hr.left() + 22.0, hr.center().y),
                        egui::Align2::LEFT_CENTER,
                        crate::i18n::t(name),
                        egui::FontId::proportional(12.5),
                        t.text,
                    );
                    let gear = Rect::from_center_size(pos2(hr.right() - 12.0, hr.center().y), vec2(13.0, 13.0));
                    let gresp = ui.interact(gear, ui.id().with(("ts_gear", name)), Sense::click());
                    icons::paint(pp, Rect::from_center_size(gear.center(), vec2(11.0, 11.0)), Icon::Gear, false);
                    icons::describe_control(ui, &hresp, name, Some(!collapsed), false);
                    icons::describe_control(ui, &gresp, &format!("{name} options"), None, false);
                    let gresp = gresp.on_hover_text(crate::tl!("Show and hide the tools of this category"));
                    if hresp.clicked() {
                        toggle_group = Some(name.to_string());
                    }
                    let hidden = |cmd: &str| app.ui.hidden_tools.iter().any(|h| h == cmd);
                    egui::Popup::from_toggle_button_response(&gresp).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
                        ui.label(crate::tl!("Show in Tool Sets"));
                        for (_, cmd, tip) in large.iter().chain(small.iter()) {
                            let mut shown = !hidden(cmd);
                            if ui.checkbox(&mut shown, *tip).changed() {
                                tool_visible = Some((cmd.to_string(), shown));
                            }
                        }
                        if ui.selectable_label(false, crate::tl!("Show All")).clicked() {
                            restore = large.iter().chain(small.iter()).map(|(_, cmd, _)| cmd.to_string()).collect();
                        }
                    });
                    if !collapsed {
                        ui.add_space(4.0);
                        // Large icons.
                        if !large.is_empty() {
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.spacing_mut().item_spacing = vec2(8.0, 0.0);
                                for (icon, cmd, tip) in large.iter() {
                                    if hidden(cmd) {
                                        continue;
                                    }
                                    ui.add_enabled_ui(command_known(cmd), |ui| {
                                        if icons::button(ui, *icon, 40.0, tip, false).clicked() {
                                            clicked = Some(cmd);
                                        }
                                    });
                                }
                            });
                        }
                        // Small icons, 8 per row.
                        let visible: Vec<Tool> = small.iter().copied().filter(|(_, cmd, _)| !hidden(cmd)).collect();
                        for row in visible.chunks(8) {
                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                ui.add_space(8.0);
                                ui.spacing_mut().item_spacing = vec2(2.0, 0.0);
                                for (icon, cmd, tip) in row.iter() {
                                    ui.add_enabled_ui(command_known(cmd), |ui| {
                                        if icons::button(ui, *icon, 23.0, tip, false).clicked() {
                                            clicked = Some(cmd);
                                        }
                                    });
                                }
                            });
                        }
                        ui.add_space(6.0);
                    }
                    let (sr, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
                    ui.painter().hline(sr.x_range().shrink(6.0), sr.center().y, Stroke::new(1.0, t.border));
                }
            });
        });
        if let Some((cmd, shown)) = tool_visible {
            if shown {
                app.ui.hidden_tools.retain(|h| h != &cmd);
            } else if !app.ui.hidden_tools.contains(&cmd) {
                app.ui.hidden_tools.push(cmd);
            }
        }
        if !restore.is_empty() {
            app.ui.hidden_tools.retain(|h| !restore.contains(h));
        }
        if let Some(g) = toggle_group {
            if let Some(i) = app.ui.collapsed_groups.iter().position(|x| *x == g) {
                app.ui.collapsed_groups.remove(i);
            } else {
                app.ui.collapsed_groups.push(g);
            }
        }
        if let Some(c) = clicked {
            app.cmd.buffer.clear();
            app.start(c);
        }
    });
}

pub const MODELING: &[(&str, &[Tool], &[Tool])] = &[
    (
        "Primitives",
        &[(Icon::Blocks, "box", "Box"), (Icon::Circle, "cylinder", "Cylinder"), (Icon::Polygon, "pyramid", "Pyramid")],
        &[(Icon::Ellipse, "sphere", "Sphere"), (Icon::Donut, "torus", "Torus"), (Icon::Arc, "cone", "Cone")],
    ),
    (
        "Solid",
        &[(Icon::Extend, "extrude", "Extrude"), (Icon::Rotate, "revolve", "Revolve")],
        &[(Icon::Spline, "sweep", "Sweep"), (Icon::Offset, "loft", "Loft")],
    ),
    ("Boolean", &[(Icon::Join, "union", "Union"), (Icon::Trim, "subtract", "Subtract"), (Icon::Region, "intersect", "Intersect")], &[]),
    ("View", &[(Icon::Orbit, "3dorbit", "Orbit"), (Icon::ZoomExtents, "zoom.extents", "Zoom Extents")], &[]),
];

fn section_header(ui: &mut egui::Ui, title: &str) -> Rect {
    let t = Tokens::get();
    let (hr, _) = ui.allocate_exact_size(vec2(ui.available_width(), 26.0), Sense::hover());
    ui.painter().text(
        pos2(hr.left() + 10.0, hr.center().y),
        egui::Align2::LEFT_CENTER,
        crate::i18n::t(title),
        egui::FontId::proportional(14.0),
        t.text,
    );
    hr
}

fn swatch(p: &egui::Painter, r: Rect, c: Color) {
    let rgb = c.resolve(Color::Index(7), Color::Index(7));
    p.rect_filled(r, 1.0, Color32::from_rgb(rgb.0, rgb.1, rgb.2));
    p.rect_stroke(r, 1.0, Stroke::new(1.0, Color32::from_gray(30)), egui::StrokeKind::Inside);
}

pub fn right_palettes(app: &mut CadApp, ui: &mut egui::Ui) {
    if !app.ui.show_palettes {
        return;
    }
    let t = Tokens::get();
    egui::Panel::right("cc_palettes").exact_size(300.0).resizable(false).frame(egui::Frame::NONE.fill(t.panel)).show(ui, |ui| {
        let r = ui.max_rect();
        // Palette icon tabs.
        let p = ui.painter().clone();
        let tab_h = 30.0;
        p.rect_filled(Rect::from_min_size(r.min, vec2(r.width(), tab_h)), 0.0, t.chrome);
        for (i, icon) in [Icon::Layers, Icon::Blocks, Icon::Properties].iter().enumerate() {
            let br = Rect::from_center_size(pos2(r.left() + 30.0 + i as f32 * 52.0, r.top() + tab_h / 2.0), vec2(20.0, 20.0));
            icons::paint(&p, br, *icon, false);
        }
        let cr = Rect::from_center_size(pos2(r.right() - 14.0, r.top() + tab_h / 2.0), vec2(14.0, 14.0));
        let cresp = ui.interact(cr, ui.id().with("palettes_collapse"), Sense::click());
        icons::paint(&p, cr, Icon::ChevronRight, false);
        icons::describe_control(ui, &cresp, "Collapse Palettes", None, false);
        if cresp.on_hover_text(crate::tl!("Collapse Palettes")).clicked() {
            app.ui.show_palettes = false;
        }
        let body = Rect::from_min_max(pos2(r.left(), r.top() + tab_h), r.max);
        ui.scope_builder(egui::UiBuilder::new().max_rect(body), |ui| {
            egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                layers_section(app, ui);
                ui.add_space(8.0);
                properties::section(app, ui);
            });
        });
    });
}

pub(crate) fn layers_section(app: &mut CadApp, ui: &mut egui::Ui) {
    let t = Tokens::get();
    section_header(ui, "Layers");
    // Layer tools row.
    let mut cmd = None;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.spacing_mut().item_spacing = vec2(3.0, 0.0);
        for (icon, c, tip) in [
            (Icon::LayerProps, "ui.dialog.layers", "Layer Properties"),
            (Icon::MakeCurrent, "laymcur", "Make Object's Layer Current"),
            (Icon::LayerMatch, "laymch", "Match Layer"),
            (Icon::LayerPrev, "layerp", "Previous Layer"),
            (Icon::LayerIso, "layiso", "Isolate"),
            (Icon::LayerOff, "layoff", "Turn Off Layer"),
            (Icon::LayerFreeze, "layfrz", "Freeze Layer"),
            (Icon::LayerLock, "laylck", "Lock Layer"),
            (Icon::Unlock, "layulk", "Unlock Layer"),
        ] {
            if icons::button(ui, icon, 26.0, tip, false).clicked() {
                cmd = Some(c);
            }
        }
    });
    if let Some(c) = cmd {
        if c.starts_with("ui.") {
            app.start(c);
        } else {
            let _ = app.run(c, json!({}));
        }
    }
    ui.add_space(4.0);
    // Current layer combo.
    let Ok(d) = app.session.doc() else { return };
    let cur = d.header.str("CLAYER", "0");
    let layers: Vec<(String, bool, bool, bool, Color)> = d.layers.iter().map(|l| (l.name.clone(), l.on, l.frozen, l.locked, l.color)).collect();
    let cur_info = layers.iter().find(|l| l.0.eq_ignore_ascii_case(&cur)).cloned();
    let mut set_current = None;
    let mut toggle: Option<(String, &str, bool)> = None;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let w = ui.available_width() - 10.0;
        let (rect, resp) = ui.allocate_exact_size(vec2(w, 24.0), Sense::click());
        let p = ui.painter();
        p.rect_filled(rect, 3.0, t.control);
        if let Some((name, on, frozen, locked, color)) = &cur_info {
            let mut x = rect.left() + 6.0;
            for icon in [
                if *on { Icon::Bulb } else { Icon::BulbOff },
                if *locked { Icon::Lock } else { Icon::Unlock },
                if *frozen { Icon::Snowflake } else { Icon::Sun },
            ] {
                icons::paint(p, Rect::from_min_size(pos2(x, rect.top() + 4.0), vec2(16.0, 16.0)), icon, false);
                x += 18.0;
            }
            swatch(p, Rect::from_min_size(pos2(x + 2.0, rect.top() + 7.0), vec2(10.0, 10.0)), *color);
            p.text(pos2(x + 20.0, rect.center().y), egui::Align2::LEFT_CENTER, name, crate::theme::body(), t.text);
        }
        icons::paint(p, Rect::from_center_size(pos2(rect.right() - 12.0, rect.center().y), vec2(12.0, 12.0)), Icon::ChevronDown, false);
        egui::Popup::from_toggle_button_response(&resp).width(w).close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside).show(|ui| {
            for (name, on, frozen, locked, color) in &layers {
                ui.horizontal(|ui| {
                    if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 18.0, "On/Off", false).clicked() {
                        toggle = Some((name.clone(), "on", !*on));
                    }
                    if icons::button(ui, if *frozen { Icon::Snowflake } else { Icon::Sun }, 18.0, "Freeze/Thaw", false).clicked() {
                        toggle = Some((name.clone(), "frozen", !*frozen));
                    }
                    if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 18.0, "Lock/Unlock", false).clicked() {
                        toggle = Some((name.clone(), "locked", !*locked));
                    }
                    let (sr, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                    swatch(ui.painter(), sr, *color);
                    if ui.selectable_label(name.eq_ignore_ascii_case(&cur), name).clicked() {
                        set_current = Some(name.clone());
                    }
                });
            }
        });
    });
    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let w = ui.available_width() - 10.0;
        let (rect, _) = ui.allocate_exact_size(vec2(w, 24.0), Sense::hover());
        ui.painter().rect_filled(rect, 3.0, t.control);
        ui.painter().text(
            pos2(rect.left() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            crate::tl!("Unsaved Layer State"),
            crate::theme::body(),
            t.text,
        );
        icons::paint(ui.painter(), Rect::from_center_size(pos2(rect.right() - 12.0, rect.center().y), vec2(12.0, 12.0)), Icon::ChevronDown, false);
    });
    ui.add_space(4.0);
    let label = if app.ui.show_layer_list { "Hide Layer List" } else { "Show Layer List" };
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        let (rect, resp) = ui.allocate_exact_size(vec2(160.0, 20.0), Sense::click());
        icons::paint(
            ui.painter(),
            Rect::from_center_size(pos2(rect.left() + 7.0, rect.center().y), vec2(11.0, 11.0)),
            if app.ui.show_layer_list { Icon::ChevronDown } else { Icon::ChevronRight },
            false,
        );
        ui.painter().text(pos2(rect.left() + 20.0, rect.center().y), egui::Align2::LEFT_CENTER, crate::i18n::t(label), crate::theme::body(), t.text);
        if resp.clicked() {
            app.ui.show_layer_list = !app.ui.show_layer_list;
        }
    });
    if app.ui.show_layer_list {
        ui.add_space(2.0);
        for (name, on, frozen, locked, color) in &layers {
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                if icons::button(ui, if *on { Icon::Bulb } else { Icon::BulbOff }, 18.0, "On/Off", false).clicked() {
                    toggle = Some((name.clone(), "on", !*on));
                }
                if icons::button(ui, if *frozen { Icon::Snowflake } else { Icon::Sun }, 18.0, "Freeze/Thaw", false).clicked() {
                    toggle = Some((name.clone(), "frozen", !*frozen));
                }
                if icons::button(ui, if *locked { Icon::Lock } else { Icon::Unlock }, 18.0, "Lock/Unlock", false).clicked() {
                    toggle = Some((name.clone(), "locked", !*locked));
                }
                let (sr, _) = ui.allocate_exact_size(vec2(12.0, 12.0), Sense::hover());
                swatch(ui.painter(), sr, *color);
                let r = ui.selectable_label(name.eq_ignore_ascii_case(&cur), name);
                if r.clicked() {
                    set_current = Some(name.clone());
                }
                crate::shortcut_menus::attach(app, &r, |app| crate::shortcut_menus::layer_entries(app, name));
            });
        }
    }
    if let Some(n) = set_current {
        let _ = app.run("layer.current", json!({ "name": n }));
    }
    if let Some((n, k, v)) = toggle {
        let _ = app.run("layer.set", json!({ "name": n, k: v }));
    }
}
