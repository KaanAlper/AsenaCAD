//! The title strip on a palette's left edge, as AutoCAD's palettes have it: Close, Auto-hide and
//! the palette's menu at the top, the palette's name written bottom to top near the bottom, and
//! the palette's icon below the name. Dragging the strip moves the palette.

use egui::{Id, Rect, Sense, Ui, pos2, vec2};

use crate::icons::{self, Icon};
use crate::theme::Tokens;

/// The strip's width.
pub const WIDTH: f32 = 22.0;
/// The strip buttons' size.
const BUTTON: f32 = 18.0;

/// Run `add` in a child UI over `rect` without taking space in `ui`'s layout.
pub fn place<R>(ui: &mut Ui, rect: Rect, add: impl FnOnce(&mut Ui) -> R) -> R {
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::top_down(egui::Align::Min)));
    add(&mut child)
}

/// What happened on the strip this frame.
pub struct Strip {
    /// Close was clicked.
    pub close: bool,
    /// The strip's drag response (the palette moves with it).
    pub drag: egui::Response,
}

/// Draw the strip in `rect`. `menu` fills the palette menu; it gets `true` back from `Close`.
pub fn show(ui: &mut Ui, rect: Rect, id: Id, title: &str, icon: Icon, menu: impl FnOnce(&mut Ui) -> bool) -> Strip {
    let t = Tokens::get();
    ui.painter().rect_filled(rect, 0.0, t.chrome_dark);
    // The drag area first: the buttons drawn after it take their clicks.
    let drag = ui.interact(rect, id.with("strip"), Sense::click_and_drag());
    let at = |i: f32| Rect::from_min_size(pos2(rect.center().x - BUTTON / 2.0, rect.top() + 3.0 + i * (BUTTON + 3.0)), vec2(BUTTON, BUTTON));
    let mut close = false;
    place(ui, at(0.0), |ui| {
        close = icons::button(ui, Icon::Close, BUTTON, "Close", false).clicked();
    });
    // Auto-hide (the palette folding to its strip) is not built yet.
    place(ui, at(1.0), |ui| {
        ui.add_enabled_ui(false, |ui| icons::button(ui, Icon::AutoHide, BUTTON, "Auto-hide", false));
    });
    place(ui, at(2.0), |ui| {
        let resp = icons::button(ui, Icon::Menu, BUTTON, "Menu", false);
        egui::Popup::from_toggle_button_response(&resp).id(id.with("menu")).close_behavior(egui::PopupCloseBehavior::CloseOnClick).show(|ui| {
            if menu(ui) {
                close = true;
            }
        });
    });
    // The palette's icon at the bottom, its name above it reading upwards.
    let icon_rect = Rect::from_center_size(pos2(rect.center().x, rect.bottom() - 13.0), vec2(16.0, 16.0));
    icons::paint(ui.painter(), icon_rect, icon, false);
    let galley = ui.painter().layout_no_wrap(crate::i18n::t(title).to_uppercase(), crate::theme::small(), t.text_dim);
    let space = icon_rect.top() - 6.0 - (rect.top() + 3.0 + 3.0 * (BUTTON + 3.0));
    if galley.size().x <= space {
        let origin = pos2(rect.center().x - galley.size().y / 2.0, icon_rect.top() - 6.0);
        ui.painter().add(egui::epaint::TextShape::new(origin, galley, t.text_dim).with_angle(-std::f32::consts::FRAC_PI_2));
    }
    Strip { close, drag }
}
