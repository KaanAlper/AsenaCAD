//! Ribbon combo boxes and sliders, bound to the drawing's current properties and styles. Each
//! choice runs the command the Properties palette runs for the same row; a control with nothing
//! to set yet is shown disabled with its value.

use cadcraft_color::Color;
use egui::{Response, Ui, WidgetInfo, WidgetType};
use serde_json::{Value, json};

use super::model::{Combo, Slider};
use crate::CadApp;

/// What a combo box shows and the command each of its entries runs.
struct Binding {
    current: String,
    choices: Vec<(String, &'static str, Value)>,
}

impl Binding {
    fn read_only(current: impl Into<String>) -> Self {
        Self { current: current.into(), choices: Vec::new() }
    }
}

/// ByLayer / ByBlock are translated; layer, linetype and style names are the drawing's own.
fn shown(name: &str) -> String {
    if ["ByLayer", "ByBlock"].contains(&name) { crate::i18n::t(name).to_string() } else { name.to_string() }
}

/// `current` and one entry per name of `list`, each running `cmd` with `params(name)`.
fn names(current: String, list: impl IntoIterator<Item = String>, cmd: &'static str, params: impl Fn(&str) -> Value) -> Binding {
    let choices = list.into_iter().map(|n| (shown(&n), cmd, params(&n))).collect();
    Binding { current: shown(&current), choices }
}

const COLORS: [(&str, i64); 9] =
    [("ByLayer", 256), ("ByBlock", 0), ("Red", 1), ("Yellow", 2), ("Green", 3), ("Cyan", 4), ("Blue", 5), ("Magenta", 6), ("White", 7)];

/// ATTMODE values and their entries (ATTDISP's modes).
const ATTMODES: [(i64, &str, &str); 3] =
    [(1, "Retain Attribute Display", "normal"), (2, "Display All Attributes", "on"), (0, "Hide All Attributes", "off")];

fn binding(app: &CadApp, kind: Combo) -> Binding {
    let Ok(d) = app.session.doc() else { return Binding::read_only("") };
    let h = &d.header;
    let by = ["ByLayer", "ByBlock"].map(String::from);
    match kind {
        Combo::Layer => names(h.str("CLAYER", "0"), d.layers.iter().map(|l| l.name.clone()), "layer.current", |n| json!({ "name": n })),
        Combo::Color => {
            let aci = i16::try_from(h.i64("CECOLOR", 256)).unwrap_or(256);
            let current = crate::i18n::t(&Color::from_aci(aci).name()).to_string();
            let choices = COLORS.iter().map(|(n, c)| (crate::i18n::t(n).to_string(), "color", json!({ "color": c }))).collect();
            Binding { current, choices }
        }
        Combo::Linetype => {
            let others = d.linetypes.iter().map(|l| l.name.clone()).filter(|n| !by.contains(n));
            names(h.str("CELTYPE", "ByLayer"), by.clone().into_iter().chain(others), "linetype", |n| json!({ "current": n }))
        }
        Combo::Lineweight => {
            let lw = |v: i64| cadcraft_doc::Lineweight::from_dxf(i16::try_from(v).unwrap_or(-1)).name();
            let mut choices: Vec<_> =
                ["ByLayer", "ByBlock", "Default"].iter().map(|n| (crate::i18n::t(n).to_string(), "lweight", json!({ "lineweight": n }))).collect();
            choices.extend(
                cadcraft_doc::Lineweight::STANDARD
                    .iter()
                    .map(|v| (format!("{:.2} mm", f64::from(*v) / 100.0), "lweight", json!({ "lineweight": f64::from(*v) / 100.0 }))),
            );
            let current = lw(h.i64("CELWEIGHT", -1));
            Binding { current: crate::i18n::t(&current).to_string(), choices }
        }
        Combo::TextStyle => {
            names(h.str("TEXTSTYLE", "Standard"), d.text_styles.iter().map(|s| s.name.clone()), "style.current", |n| json!({ "name": n }))
        }
        Combo::DimStyle => {
            names(h.str("DIMSTYLE", "Standard"), d.dim_styles.iter().map(|s| s.name.clone()), "dimstyle.current", |n| json!({ "name": n }))
        }
        Combo::MLeaderStyle => names(
            h.str("CMLEADERSTYLE", "Standard"),
            d.mleader_styles.iter().map(|s| s.name.clone()),
            "mleaderstyle",
            |n| json!({ "name": n, "current": true }),
        ),
        Combo::TableStyle => names(
            h.str("CTABLESTYLE", "Standard"),
            d.table_styles.iter().map(|s| s.name.clone()),
            "tablestyle",
            |n| json!({ "name": n, "current": true }),
        ),
        // The drawing doesn't record which saved state was restored last.
        Combo::LayerState => {
            let mut b = names(String::new(), d.layer_states.iter().map(|s| s.name.clone()), "layerstate.restore", |n| json!({ "name": n }));
            b.current = crate::tl!("Unsaved Layer State").to_string();
            b
        }
        // Named plot style tables give the current plot style; colour-dependent ones plot ByColor.
        Combo::PlotStyle => {
            let space = app.session.layout_space();
            let table = space.layout().and_then(|n| d.layout(n)).and_then(|l| cadcraft_doc::plot_style_table(d, &l.page.plot_style_table));
            match table.filter(|t| !t.is_color_dependent()) {
                Some(t) => names(
                    h.str("CPLOTSTYLE", "ByLayer"),
                    by.into_iter().chain(t.styles.iter().map(|s| s.name.clone())),
                    "plotstyle",
                    |n| json!({ "name": n }),
                ),
                None => Binding::read_only(crate::tl!("ByColor")),
            }
        }
        Combo::AttributeDisplay => {
            let mode = h.i64("ATTMODE", 1);
            let current = ATTMODES.iter().find(|(m, _, _)| *m == mode).map_or("Retain Attribute Display", |(_, l, _)| *l);
            let choices = ATTMODES.iter().map(|(_, l, k)| (crate::i18n::t(l).to_string(), "attdisp", json!({ "mode": k }))).collect();
            Binding { current: crate::i18n::t(current).to_string(), choices }
        }
        // CETRANSPARENCY isn't modelled yet: new objects take their layer's transparency.
        Combo::Transparency => Binding::read_only(crate::i18n::t("ByLayer")),
        // Model documentation (VIEWBASE) and its styles don't exist yet.
        Combo::SectionViewStyle | Combo::DetailViewStyle => Binding::read_only(""),
    }
}

/// A combo box of `width` showing the current value of `kind`; choosing an entry runs its command.
pub fn combo(ui: &mut Ui, app: &mut CadApp, kind: Combo, width: f32) -> Response {
    let b = binding(app, kind);
    let label = crate::i18n::t(kind.label());
    let editable = !b.choices.is_empty();
    let mut picked = None;
    let resp = ui
        .add_enabled_ui(editable, |ui| {
            egui::ComboBox::from_id_salt(("ribbon-combo", kind)).width(width).selected_text(b.current.as_str()).show_ui(ui, |ui| {
                for (i, (name, _, _)) in b.choices.iter().enumerate() {
                    if ui.selectable_label(*name == b.current, name.as_str()).clicked() {
                        picked = Some(i);
                    }
                }
            })
        })
        .inner
        .response;
    resp.widget_info(|| {
        let mut info = WidgetInfo::labeled(WidgetType::ComboBox, resp.enabled(), label);
        info.current_text_value = Some(b.current.clone());
        info
    });
    crate::control::record_widget(ui, &resp, label, "combobox", None);
    let resp = resp.on_hover_text(label).on_disabled_hover_text(crate::tl!("Not available yet"));
    if let Some((_, cmd, params)) = picked.and_then(|i| b.choices.get(i)) {
        // Errors are echoed to the command history by `run`.
        let _ = app.run(cmd, params.clone());
    }
    resp
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
    resp.on_hover_text(label).on_disabled_hover_text(crate::tl!("Not available yet"))
}
