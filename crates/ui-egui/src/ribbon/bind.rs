//! Ribbon combo boxes and sliders, bound to the drawing's current properties and styles. Each
//! choice runs the command the Properties palette runs for the same row; a control with nothing
//! to set yet is shown disabled with its value. Each frame reads only the current value; the
//! list of entries is built while it is open.

use cadcraft_color::Color;
use cadcraft_doc::{Drawing, Lineweight};
use egui::{Response, Ui, WidgetInfo, WidgetType};
use serde_json::{Value, json};

use super::model::{Combo, Slider};
use crate::CadApp;

/// One entry of a combo box's list: what it shows, and the command and parameters it runs.
type Choice = (String, &'static str, Value);

/// ByLayer / ByBlock are translated; layer, linetype and style names are the drawing's own.
fn shown(name: &str) -> String {
    if BY.contains(&name) { crate::i18n::t(name).to_string() } else { name.to_string() }
}

/// One entry per name of `list`, each running `cmd` with `params(name)`.
fn names<'a>(list: impl IntoIterator<Item = &'a str>, cmd: &'static str, params: impl Fn(&str) -> Value) -> Vec<Choice> {
    list.into_iter().map(|n| (shown(n), cmd, params(n))).collect()
}

const BY: [&str; 2] = ["ByLayer", "ByBlock"];

const COLORS: [(&str, i64); 9] =
    [("ByLayer", 256), ("ByBlock", 0), ("Red", 1), ("Yellow", 2), ("Green", 3), ("Cyan", 4), ("Blue", 5), ("Magenta", 6), ("White", 7)];

/// ATTMODE values and their entries (ATTDISP's modes).
const ATTMODES: [(i64, &str, &str); 3] =
    [(1, "Retain Attribute Display", "normal"), (2, "Display All Attributes", "on"), (0, "Hide All Attributes", "off")];

/// A lineweight as the combo shows it: the same text for the current value and its entry.
fn lineweight_name(lw: Lineweight) -> String {
    crate::i18n::t(&lw.name()).to_string()
}

/// The plot style table the current layout's page setup names, if any.
fn plot_table_name<'d>(app: &CadApp, d: &'d Drawing) -> Option<&'d str> {
    let space = app.session.layout_space();
    space.layout().and_then(|n| d.layout(n)).map(|l| l.page.plot_style_table.as_str())
}

/// Whether `name` is a named (STB) plot style table, without building the large built-in
/// colour-dependent ones.
fn is_named_table(d: &Drawing, name: &str) -> bool {
    if cadcraft_doc::no_plot_style_table(name) {
        return false;
    }
    let n = name.trim();
    match d.plot_style_tables.iter().find(|t| t.name.eq_ignore_ascii_case(n)) {
        Some(t) => !t.is_color_dependent(),
        None => {
            let stb = n.get(n.len().saturating_sub(4)..).is_some_and(|ext| ext.eq_ignore_ascii_case(".stb"));
            stb && cadcraft_doc::plot_style_table(d, n).is_some_and(|t| !t.is_color_dependent())
        }
    }
}

/// What a combo box shows now, and whether it has entries to choose from. Runs every frame,
/// so it never builds the list.
fn current(app: &CadApp, kind: Combo) -> (String, bool) {
    let Ok(d) = app.session.doc() else { return (String::new(), false) };
    let h = &d.header;
    match kind {
        Combo::Layer => (shown(&h.str("CLAYER", "0")), !d.layers.is_empty()),
        Combo::Color => {
            let aci = i16::try_from(h.i64("CECOLOR", 256)).unwrap_or(256);
            (crate::i18n::t(&Color::from_aci(aci).name()).to_string(), true)
        }
        Combo::Linetype => (shown(&h.str("CELTYPE", "ByLayer")), true),
        Combo::Lineweight => (lineweight_name(Lineweight::from_dxf(i16::try_from(h.i64("CELWEIGHT", -1)).unwrap_or(-1))), true),
        Combo::TextStyle => (shown(&h.str("TEXTSTYLE", "Standard")), !d.text_styles.is_empty()),
        Combo::DimStyle => (shown(&h.str("DIMSTYLE", "Standard")), !d.dim_styles.is_empty()),
        Combo::MLeaderStyle => (shown(&h.str("CMLEADERSTYLE", "Standard")), !d.mleader_styles.is_empty()),
        Combo::TableStyle => (shown(&h.str("CTABLESTYLE", "Standard")), !d.table_styles.is_empty()),
        // The drawing doesn't record which saved state was restored last.
        Combo::LayerState => (crate::tl!("Unsaved Layer State").to_string(), !d.layer_states.is_empty()),
        // Named plot style tables give the current plot style; colour-dependent ones plot ByColor.
        Combo::PlotStyle => match plot_table_name(app, d).is_some_and(|n| is_named_table(d, n)) {
            true => (shown(&h.str("CPLOTSTYLE", "ByLayer")), true),
            false => (crate::tl!("ByColor").to_string(), false),
        },
        Combo::AttributeDisplay => {
            let mode = h.i64("ATTMODE", 1);
            let current = ATTMODES.iter().find(|(m, _, _)| *m == mode).map_or("Retain Attribute Display", |(_, l, _)| *l);
            (crate::i18n::t(current).to_string(), true)
        }
        // CETRANSPARENCY isn't modelled yet: new objects take their layer's transparency.
        Combo::Transparency => (crate::i18n::t("ByLayer").to_string(), false),
        // Model documentation (VIEWBASE) and its styles don't exist yet.
        Combo::SectionViewStyle | Combo::DetailViewStyle => (String::new(), false),
        // FRAME and UOSNAP aren't modelled yet: AutoCAD's defaults, shown disabled.
        Combo::XrefFrames => (crate::tl!("*Frames vary*").to_string(), false),
        Combo::UnderlaySnap => (crate::tl!("Snap to Underlays ON").to_string(), false),
        Combo::FindText => (String::new(), true),
        Combo::TextHeight => (height_text(text_size(app)), true),
        // TEXTLAYER, DIMLAYER, CENTERLAYER and the multileader layer aren't modelled yet: new
        // annotations go on the current layer, which is what "Use Current" means.
        Combo::TextLayer | Combo::DimLayer | Combo::CenterLayer | Combo::MLeaderLayer => (crate::tl!("Use Current").to_string(), false),
        // The drawing doesn't record which view was restored last.
        Combo::NamedView => (crate::tl!("Unsaved View").to_string(), space_views(app, d).next().is_some()),
    }
}

/// The named views of the space shown: model space views in the Model tab, the layout's own in a
/// layout.
fn space_views<'d>(app: &CadApp, d: &'d Drawing) -> impl Iterator<Item = &'d str> {
    let space = app.session.layout_space();
    let layout = space.layout().map(str::to_string);
    d.views.iter().filter(move |v| v.layout == layout).map(|v| v.name.as_str())
}

/// The entries of `kind`'s list, built only while the list is open.
fn choices(app: &CadApp, kind: Combo) -> Vec<Choice> {
    let Ok(d) = app.session.doc() else { return Vec::new() };
    match kind {
        Combo::Layer => names(d.layers.iter().map(|l| l.name.as_str()), "layer.current", |n| json!({ "name": n })),
        Combo::Color => COLORS.iter().map(|(n, c)| (crate::i18n::t(n).to_string(), "color", json!({ "color": c }))).collect(),
        Combo::Linetype => {
            let others = d.linetypes.iter().map(|l| l.name.as_str()).filter(|n| !BY.contains(n));
            names(BY.into_iter().chain(others), "linetype", |n| json!({ "current": n }))
        }
        Combo::Lineweight => [Lineweight::ByLayer, Lineweight::ByBlock, Lineweight::Default]
            .into_iter()
            .chain(Lineweight::STANDARD.iter().map(|v| Lineweight::Mm100(*v)))
            .map(|lw| {
                let value = match lw {
                    Lineweight::Mm100(v) => json!(f64::from(v) / 100.0),
                    other => json!(other.name()),
                };
                (lineweight_name(lw), "lweight", json!({ "lineweight": value }))
            })
            .collect(),
        Combo::TextStyle => names(d.text_styles.iter().map(|s| s.name.as_str()), "style.current", |n| json!({ "name": n })),
        Combo::DimStyle => names(d.dim_styles.iter().map(|s| s.name.as_str()), "dimstyle.current", |n| json!({ "name": n })),
        Combo::MLeaderStyle => names(d.mleader_styles.iter().map(|s| s.name.as_str()), "mleaderstyle", |n| json!({ "name": n, "current": true })),
        Combo::TableStyle => names(d.table_styles.iter().map(|s| s.name.as_str()), "tablestyle", |n| json!({ "name": n, "current": true })),
        Combo::LayerState => names(d.layer_states.iter().map(|s| s.name.as_str()), "layerstate.restore", |n| json!({ "name": n })),
        Combo::PlotStyle => {
            let table = plot_table_name(app, d).and_then(|n| cadcraft_doc::plot_style_table(d, n)).filter(|t| !t.is_color_dependent());
            table.map_or_else(Vec::new, |t| {
                names(BY.into_iter().chain(t.styles.iter().map(|s| s.name.as_str())), "plotstyle", |n| json!({ "name": n }))
            })
        }
        Combo::NamedView => names(space_views(app, d), "view", |n| json!({ "option": "restore", "name": n })),
        Combo::AttributeDisplay => ATTMODES.iter().map(|(_, l, k)| (crate::i18n::t(l).to_string(), "attdisp", json!({ "mode": k }))).collect(),
        Combo::Transparency
        | Combo::SectionViewStyle
        | Combo::DetailViewStyle
        | Combo::XrefFrames
        | Combo::UnderlaySnap
        | Combo::FindText
        | Combo::TextHeight
        | Combo::TextLayer
        | Combo::DimLayer
        | Combo::CenterLayer
        | Combo::MLeaderLayer => Vec::new(),
    }
}

/// A text height as AutoCAD's box shows it: up to four decimals, no trailing zeros ("2.5").
fn height_text(v: f64) -> String {
    let text = format!("{v:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text.is_empty() || text == "-" { "0".to_string() } else { text.to_string() }
}

/// TEXTSIZE, or the default for the drawing's units when the header has none.
fn text_size(app: &CadApp) -> f64 {
    cadcraft_engine::sysvars::get(&app.session, "TEXTSIZE").and_then(|v| v.as_f64()).filter(|v| v.is_finite()).unwrap_or(0.2)
}

/// Our own choice of common text heights for the height list: paper-like sizes in the drawing's
/// units (ISO 3098's series in a metric drawing).
const METRIC_HEIGHTS: [f64; 6] = [1.8, 2.5, 3.5, 5.0, 7.0, 10.0];
const IMPERIAL_HEIGHTS: [f64; 6] = [0.1, 0.125, 0.1875, 0.25, 0.375, 0.5];
/// The height list never grows past this many entries, however many heights the drawing uses.
const MAX_HEIGHTS: usize = 24;

/// The Annotation Text Height list: the current height, the heights the drawing's text and
/// multiline text use, and the common sizes, smallest first, each once.
fn heights(app: &CadApp) -> Vec<f64> {
    let Ok(d) = app.session.doc() else { return Vec::new() };
    let common = if d.header.i64("MEASUREMENT", 0) == 1 { METRIC_HEIGHTS } else { IMPERIAL_HEIGHTS };
    let used = std::iter::once(&d.model).chain(d.layouts.iter().map(|l| &l.entities)).flat_map(|st| st.iter()).filter_map(|e| match &e.kind {
        cadcraft_doc::EntityKind::Text(t) => Some(t.height),
        cadcraft_doc::EntityKind::MText(t) => Some(t.height),
        _ => None,
    });
    let mut out: Vec<f64> = std::iter::once(text_size(app)).chain(common).chain(used).filter(|h| h.is_finite() && *h > 0.0).collect();
    out.sort_by(f64::total_cmp);
    out.dedup_by(|a, b| height_text(*a) == height_text(*b));
    out.truncate(MAX_HEIGHTS);
    out
}

/// The ▾ part of an editable combo box.
const COMBO_ARROW_W: f32 = 18.0;

/// The editable boxes: Find Text (Enter finds the typed text with FIND, which selects the
/// objects it is in) and Annotation Text Height, an editable combo box (Enter or an entry of its
/// ▾ list sets TEXTSIZE). While it isn't being edited, the height box shows the drawing's value.
fn text_field(ui: &mut Ui, app: &mut CadApp, kind: Combo, width: f32) -> Response {
    let label = crate::i18n::t(kind.label());
    let id = egui::Id::new(("ribbon-field", kind));
    let buffer = id.with("text");
    let (value, _) = current(app, kind);
    let focused = ui.memory(|m| m.has_focus(id));
    let mut text = match kind {
        Combo::TextHeight if !focused => value.clone(),
        _ => ui.data(|d| d.get_temp::<String>(buffer)).unwrap_or_default(),
    };
    let enabled = app.session.doc().is_ok();
    let list = kind == Combo::TextHeight;
    let hint = if kind == Combo::FindText { crate::tl!("Find text") } else { "" };
    let edit = egui::TextEdit::singleline(&mut text).id(id).hint_text(hint);
    let box_w = if list { width - COMBO_ARROW_W } else { width };
    let (resp, arrow) = ui
        .add_enabled_ui(enabled, |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let h = ui.available_height();
            let resp = ui.add_sized(egui::vec2(box_w, h), edit);
            let arrow = list.then(|| ui.allocate_exact_size(egui::vec2(COMBO_ARROW_W, h), egui::Sense::click()).1);
            (resp, arrow)
        })
        .inner;
    let entered = resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
    resp.widget_info(|| {
        let mut info = WidgetInfo::labeled(WidgetType::TextEdit, resp.enabled(), label);
        info.current_text_value = Some(text.clone());
        info
    });
    crate::control::record_widget(ui, &resp, label, "textbox", None);
    let typed = text.trim().to_string();
    ui.data_mut(|d| d.insert_temp(buffer, text));
    let mut picked = None;
    if let Some(arrow) = arrow {
        let t = crate::theme::Tokens::get();
        if arrow.hovered() && arrow.enabled() {
            ui.painter().rect_filled(arrow.rect, 2.0, t.ribbon_hover);
        }
        let chevron = egui::Rect::from_center_size(arrow.rect.center(), egui::Vec2::splat(10.0));
        crate::icons::paint(ui.painter(), chevron, crate::icons::Icon::ChevronDown, !arrow.enabled());
        let options = crate::tf!("{label} options", label = label);
        crate::icons::describe_control(ui, &arrow, &options, None, false);
        egui::Popup::from_toggle_button_response(&arrow).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
            ui.set_min_width(width - 8.0);
            for h in heights(app) {
                let shown = height_text(h);
                if ui.selectable_label(shown == value, shown.as_str()).clicked() {
                    picked = Some(h);
                }
            }
        });
    }
    // Errors are echoed to the command history by `run`; a typed height that isn't a positive
    // number is left alone and the box shows the drawing's value again.
    let height = picked.or_else(|| (entered && list).then(|| typed.parse::<f64>().ok()).flatten().filter(|v| v.is_finite() && *v > 0.0));
    if let Some(v) = height {
        let _ = app.run("setvar", json!({ "name": "TEXTSIZE", "value": v }));
    }
    if entered && kind == Combo::FindText && !typed.is_empty() {
        let _ = app.run("find", json!({ "find": typed }));
    }
    resp.on_hover_text(label)
}

/// A combo box of `width` showing the current value of `kind`; choosing an entry runs its command.
pub fn combo(ui: &mut Ui, app: &mut CadApp, kind: Combo, width: f32) -> Response {
    if matches!(kind, Combo::FindText | Combo::TextHeight) {
        return text_field(ui, app, kind, width);
    }
    let (current, editable) = current(app, kind);
    let label = crate::i18n::t(kind.label());
    let mut picked = None;
    let resp = ui
        .add_enabled_ui(editable, |ui| {
            egui::ComboBox::from_id_salt(("ribbon-combo", kind)).width(width).selected_text(current.as_str()).show_ui(ui, |ui| {
                for (name, cmd, params) in choices(app, kind) {
                    if ui.selectable_label(name == current, name.as_str()).clicked() {
                        picked = Some((cmd, params));
                    }
                }
            })
        })
        .inner
        .response;
    resp.widget_info(|| {
        let mut info = WidgetInfo::labeled(WidgetType::ComboBox, resp.enabled(), label);
        info.current_text_value = Some(current.clone());
        info
    });
    crate::control::record_widget(ui, &resp, label, "combobox", None);
    let resp = not_available_yet(resp.on_hover_text(label));
    if let Some((cmd, params)) = picked {
        // Errors are echoed to the command history by `run`.
        let _ = app.run(cmd, params);
    }
    resp
}

/// A combo box or slider with nothing to set yet.
fn not_available_yet(resp: Response) -> Response {
    resp.on_disabled_hover_ui(|ui| {
        ui.label(crate::tl!("Not available yet"));
    })
}

/// The system variable a slider shows and the value AutoCAD starts with.
const fn slider_var(kind: Slider) -> (&'static str, f64) {
    match kind {
        Slider::LockedLayerFade => ("LAYLOCKFADECTL", 50.0),
        Slider::Transparency => ("CETRANSPARENCY", 0.0),
        Slider::XrefFade => ("XDWGFADECTL", 50.0),
    }
}

/// A 0–90 slider with its value at the right. None of the variables is modelled by the engine yet,
/// so the slider shows the value (the drawing's, else AutoCAD's default) and is disabled.
pub fn slider(ui: &mut Ui, app: &CadApp, kind: Slider, width: f32) -> Response {
    let (var, default) = slider_var(kind);
    let mut value = cadcraft_engine::sysvars::get(&app.session, var).and_then(|v| v.as_f64()).filter(|v| v.is_finite()).unwrap_or(default);
    let label = crate::i18n::t(kind.label());
    let resp = ui
        .add_enabled_ui(false, |ui| {
            ui.spacing_mut().slider_width = (width - 44.0).max(40.0);
            ui.add(egui::Slider::new(&mut value, 0.0..=90.0).integer())
        })
        .inner;
    resp.widget_info(|| {
        let mut info = WidgetInfo::labeled(WidgetType::Slider, resp.enabled(), label);
        info.value = Some(value);
        info
    });
    crate::control::record_widget(ui, &resp, label, "slider", None);
    not_available_yet(resp.on_hover_text(label))
}
