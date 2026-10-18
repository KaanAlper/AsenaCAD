//! The `const fn` helpers every tab's data file (`home.rs`, `insert.rs`, …) builds its items
//! with, so all tabs read the same way.

use super::model::{Combo, Entry, Item, Size, Slider, Toggle};
use crate::icons::Icon;

/// A separator between split-menu entries.
pub(crate) const SEP: Entry = Entry::Separator;

/// A split-menu entry running `cmd`.
pub(crate) const fn e(icon: Icon, label: &'static str, cmd: &'static str) -> Entry {
    Entry::Command { icon, label, cmd }
}

/// Large: 32 px icon, label under it.
pub(crate) const fn large(icon: Icon, label: &'static str, cmd: &'static str) -> Item {
    Item::Button { icon, label, cmd, size: Size::Large, row: 0 }
}

/// Small, icon only.
pub(crate) const fn icon(icon: Icon, label: &'static str, cmd: &'static str, row: u8) -> Item {
    Item::Button { icon, label, cmd, size: Size::Small { label: false }, row }
}

/// Small, icon and label.
pub(crate) const fn labeled(icon: Icon, label: &'static str, cmd: &'static str, row: u8) -> Item {
    Item::Button { icon, label, cmd, size: Size::Small { label: true }, row }
}

pub(crate) const fn toggle(icon: Icon, label: &'static str, cmd: &'static str, size: Size, row: u8, state: Toggle) -> Item {
    Item::Toggle { icon, label, cmd, size, row, state }
}

pub(crate) const fn large_split(label: Option<&'static str>, menu: &'static [Entry]) -> Item {
    Item::Split { label, size: Size::Large, row: 0, menu, default: 0 }
}

pub(crate) const fn small_split(label: Option<&'static str>, show_label: bool, row: u8, menu: &'static [Entry]) -> Item {
    Item::Split { label, size: Size::Small { label: show_label }, row, menu, default: 0 }
}

/// A large drop-down button: the whole button opens `menu`.
pub(crate) const fn large_menu(icon: Icon, label: &'static str, menu: &'static [Entry]) -> Item {
    Item::Menu { icon, label, size: Size::Large, row: 0, menu }
}

/// AutoCAD's Insert opens a gallery of the drawing's blocks; ours lists only the way to the
/// Insert dialog until the gallery is built.
pub(crate) const INSERT_GALLERY: &[Entry] = &[e(Icon::Insert, "More Options...", "insert")];

pub(crate) const fn combo(kind: Combo, width: f32, row: u8) -> Item {
    Item::Combo { kind, width, row }
}

pub(crate) const fn slider(kind: Slider, width: f32, row: u8) -> Item {
    Item::Slider { kind, width, row }
}
