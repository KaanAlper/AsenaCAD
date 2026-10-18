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
    }
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
        Combo::AttributeDisplay => ATTMODES.iter().map(|(_, l, k)| (crate::i18n::t(l).to_string(), "attdisp", json!({ "mode": k }))).collect(),
        Combo::Transparency | Combo::SectionViewStyle | Combo::DetailViewStyle => Vec::new(),
    }
}

/// A combo box of `width` showing the current value of `kind`; choosing an entry runs its command.
pub fn combo(ui: &mut Ui, app: &mut CadApp, kind: Combo, width: f32) -> Response {
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
    }
}

/// A 0–90 slider with its value at the right. Neither variable is modelled by the engine yet,
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
