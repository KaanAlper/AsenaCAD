//! Data model of the ribbon: static tables, so tests and the i18n checks can walk them.
//!
//! Layout rule for small items (what the widgets and `layout.rs` follow): consecutive small
//! items whose `row` never decreases form one block of up to three rows (slide-outs: four);
//! within a block the items of a row flow left to right. A row number lower than the previous
//! item's, or a large item, starts the next block to the right. Large items fill the content
//! height and use `row: 0`.

use crate::icons::Icon;

/// Large: 72 px tall, 32 px icon, label under it (may wrap to 2 lines).
/// Small: 22 px row item, 16 px icon, optionally with the label to the right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Large,
    Small { label: bool },
}

/// One entry of a split button's or drop-down's menu.
///
/// Every AutoCAD variant (Circle "2-Point", Arc "Start, End, Radius", "Polar Array", …) is its
/// own command id in the engine (`circle.2p`, `arc.ser`, `arraypolar`), so an entry only names a
/// command; widgets run it with `app.start(cmd)`.
#[derive(Clone, Copy, Debug)]
pub enum Entry {
    Command { icon: Icon, label: &'static str, cmd: &'static str },
    Separator,
}

/// What a combo box (or data-bound drop-down) shows and edits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Combo {
    Layer,
    Color,
    Linetype,
    Lineweight,
    TextStyle,
    DimStyle,
    MLeaderStyle,
    TableStyle,
    LayerState,
    PlotStyle,
    /// ByLayer / ByBlock / value: how new objects get their transparency (CETRANSPARENCY).
    Transparency,
    /// Retain / display all / hide all attributes (ATTMODE, the ATTDISP command).
    AttributeDisplay,
    SectionViewStyle,
    DetailViewStyle,
}

impl Combo {
    /// The control's name (accessibility, control channel). The box itself shows the value.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Layer => "Layer",
            Self::Color => "Object Color",
            Self::Linetype => "Linetype",
            Self::Lineweight => "Lineweight",
            Self::TextStyle => "Text Style",
            Self::DimStyle => "Dimension Style",
            Self::MLeaderStyle => "Multileader Style",
            Self::TableStyle => "Table Style",
            Self::LayerState => "Layer State",
            Self::PlotStyle => "Plot Style",
            Self::Transparency => "Transparency Mode",
            Self::AttributeDisplay => "Attribute Display",
            Self::SectionViewStyle => "Section View Style",
            Self::DetailViewStyle => "Detail View Style",
        }
    }
}

/// A slider with a value box at its right end.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Slider {
    /// How much locked layers fade (LAYLOCKFADECTL, 0–90 %).
    LockedLayerFade,
    /// Transparency of new objects (CETRANSPARENCY, 0–90).
    Transparency,
}

impl Slider {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LockedLayerFade => "Layer fading",
            Self::Transparency => "Transparency",
        }
    }
}

/// What a toggle button's pressed look follows (read by `widgets::is_on`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Toggle {
    /// The Layer Properties Manager is open.
    LayerPalette,
    /// Locked layers are faded (LAYLOCKFADECTL > 0).
    LockedLayerFade,
    /// Picking a group member selects the group (PICKSTYLE bit 1).
    GroupSelection,
    /// Selected groups show one bounding box (GROUPDISPLAYMODE 2).
    GroupBox,
    /// The QuickCalc palette is open.
    QuickCalcPalette,
}

#[derive(Clone, Copy, Debug)]
pub enum Item {
    Button {
        icon: Icon,
        label: &'static str,
        cmd: &'static str,
        size: Size,
        row: u8,
    },
    /// A button that shows pressed while `state` is on; clicking runs `cmd` (which flips it).
    Toggle {
        icon: Icon,
        label: &'static str,
        cmd: &'static str,
        size: Size,
        row: u8,
        state: Toggle,
    },
    /// Face runs the last-used entry (initially `menu[default]`); ▾ opens the menu.
    /// `label: None` — the face shows the remembered entry's label (Trim → Extend);
    /// `Some(l)` — the face keeps `l` and only its icon follows the entry (Circle, Arc, Array).
    Split {
        label: Option<&'static str>,
        size: Size,
        row: u8,
        menu: &'static [Entry],
        default: usize,
    },
    Combo {
        kind: Combo,
        width: f32,
        row: u8,
    },
    Slider {
        kind: Slider,
        width: f32,
        row: u8,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Panel {
    pub label: &'static str,
    pub items: &'static [Item],
    pub slideout: &'static [Item],
    /// Dialog launcher arrow in the title bar: the command it runs.
    pub launcher: Option<&'static str>,
}

#[derive(Clone, Copy, Debug)]
pub struct Tab {
    pub label: &'static str,
    pub panels: &'static [Panel],
}

/// Ribbon tabs in AutoCAD's order. M1a builds Home; the other tabs come in M1b.
pub fn tabs() -> &'static [Tab] {
    &[crate::ribbon::home::HOME]
}

fn entry_commands(menu: &'static [Entry]) -> impl Iterator<Item = &'static str> {
    menu.iter().filter_map(|e| match e {
        Entry::Command { cmd, .. } => Some(*cmd),
        Entry::Separator => None,
    })
}

/// Every command id the ribbon references (faces, menus, launchers), for tests and reports.
pub fn command_ids() -> Vec<&'static str> {
    let mut ids = Vec::new();
    for tab in tabs() {
        for panel in tab.panels {
            for item in panel.items.iter().chain(panel.slideout) {
                match item {
                    Item::Button { cmd, .. } | Item::Toggle { cmd, .. } => ids.push(*cmd),
                    Item::Split { menu, .. } => ids.extend(entry_commands(menu)),
                    Item::Combo { .. } | Item::Slider { .. } => {}
                }
            }
            ids.extend(panel.launcher);
        }
    }
    ids
}

/// Every user-visible label of the ribbon (tabs, panels, items, menu entries, control names),
/// for the translation coverage check.
pub fn labels() -> Vec<&'static str> {
    let mut out = Vec::new();
    for tab in tabs() {
        out.push(tab.label);
        for panel in tab.panels {
            out.push(panel.label);
            for item in panel.items.iter().chain(panel.slideout) {
                match item {
                    Item::Button { label, .. } | Item::Toggle { label, .. } => out.push(*label),
                    Item::Split { label, menu, .. } => {
                        out.extend(*label);
                        out.extend(menu.iter().filter_map(|e| match e {
                            Entry::Command { label, .. } => Some(*label),
                            Entry::Separator => None,
                        }));
                    }
                    Item::Combo { kind, .. } => out.push(kind.label()),
                    Item::Slider { kind, .. } => out.push(kind.label()),
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ribbon items that reference an AutoCAD command AsenaCAD doesn't have yet.
    /// They render disabled; docs/ui-parity.md lists them. Shrink this list, never grow it silently.
    pub(crate) const UNBUILT: &[&str] = &[
        "setbylayer",         // Modify ▸ Set to ByLayer
        "copytolayer",        // Layers ▸ Copy Objects to New Layer
        "laywalk",            // Layers ▸ Layer Walk
        "layvpi",             // Layers ▸ VP Freeze in All Viewports except Current
        "laylockfadectl",     // Layers ▸ Locked Layer fading (toggle)
        "bdetect",            // Block ▸ Detect and Convert
        "bconvert",           // Block ▸ Search and Convert
        "breplace",           // Block ▸ Replace
        "classicgroup",       // Groups ▸ Group Manager
        "groupdisplaymode",   // Groups ▸ Group Bounding Box (toggle)
        "measuregeom.quick",  // Utilities ▸ Measure ▸ Quick
        "measuregeom.volume", // Utilities ▸ Measure ▸ Volume
        "pastehlnk",          // Clipboard ▸ Paste ▸ Paste as Hyperlink
        "pastespec",          // Clipboard ▸ Paste ▸ Paste Special
        "viewbase",           // View ▸ Base
        "viewsectionstyle",   // View ▸ Section View Style
        "viewdetailstyle",    // View ▸ Detail View Style
        "viewstd",            // View ▸ Drafting Standard (dialog launcher)
    ];

    #[test]
    fn every_ribbon_command_exists_or_is_listed_unbuilt() {
        let missing: Vec<_> = command_ids().into_iter().filter(|id| !crate::palettes::command_known(id) && !UNBUILT.contains(id)).collect();
        assert!(missing.is_empty(), "add these to UNBUILT or build them: {missing:?}");
    }

    #[test]
    fn unbuilt_list_has_no_stale_entries() {
        let stale: Vec<_> = UNBUILT.iter().filter(|id| crate::palettes::command_known(id)).collect();
        assert!(stale.is_empty(), "these exist now, remove them from UNBUILT: {stale:?}");
        let referenced = command_ids();
        let unused: Vec<_> = UNBUILT.iter().filter(|id| !referenced.contains(id)).collect();
        assert!(unused.is_empty(), "no ribbon item uses these, remove them from UNBUILT: {unused:?}");
    }

    #[test]
    fn home_has_autocads_panels_in_order() {
        let labels: Vec<_> = tabs()[0].panels.iter().map(|p| p.label).collect();
        assert_eq!(labels, ["Draw", "Modify", "Annotation", "Layers", "Block", "Properties", "Groups", "Utilities", "Clipboard", "View"]);
    }

    #[test]
    fn small_items_use_rows_1_to_3_and_splits_have_a_valid_default() {
        for p in tabs().iter().flat_map(|t| t.panels) {
            for item in p.items {
                match item {
                    Item::Button { size: Size::Small { .. }, row, .. }
                    | Item::Toggle { size: Size::Small { .. }, row, .. }
                    | Item::Split { size: Size::Small { .. }, row, .. }
                    | Item::Combo { row, .. }
                    | Item::Slider { row, .. } => {
                        assert!((1..=3).contains(row), "{}: row {row}", p.label)
                    }
                    Item::Button { size: Size::Large, row, .. }
                    | Item::Toggle { size: Size::Large, row, .. }
                    | Item::Split { size: Size::Large, row, .. } => {
                        assert_eq!(*row, 0, "{}: large items use row 0", p.label)
                    }
                }
            }
            for item in p.slideout {
                let (Item::Button { row, .. }
                | Item::Toggle { row, .. }
                | Item::Split { row, .. }
                | Item::Combo { row, .. }
                | Item::Slider { row, .. }) = item;
                assert!((1..=4).contains(row), "{} slide-out: row {row}", p.label);
            }
            for item in p.items.iter().chain(p.slideout) {
                if let Item::Split { menu, default, .. } = item {
                    assert!(matches!(menu.get(*default), Some(Entry::Command { .. })), "{}: bad split default", p.label);
                    assert!(!matches!(menu.first(), Some(Entry::Separator)), "{}: menu starts with a separator", p.label);
                    assert!(!matches!(menu.last(), Some(Entry::Separator)), "{}: menu ends with a separator", p.label);
                }
            }
        }
    }

    #[test]
    fn home_split_menus_match_autocads_entries() {
        let menu = |label: &str| -> Vec<&'static str> {
            tabs()[0]
                .panels
                .iter()
                .flat_map(|p| p.items.iter().chain(p.slideout))
                .find_map(|i| match i {
                    Item::Split { label: l, menu, default, .. } => {
                        let face = l.or_else(|| match menu.get(*default) {
                            Some(Entry::Command { label, .. }) => Some(*label),
                            _ => None,
                        });
                        (face == Some(label))
                            .then(|| menu.iter().map(|e| if let Entry::Command { label, .. } = e { *label } else { "---" }).collect())
                    }
                    _ => None,
                })
                .unwrap_or_default()
        };
        assert_eq!(menu("Circle"), ["Center, Radius", "Center, Diameter", "---", "2-Point", "3-Point", "---", "Tan, Tan, Radius", "Tan, Tan, Tan"]);
        assert_eq!(menu("Array"), ["Rectangular Array", "Path Array", "Polar Array"]);
        assert_eq!(menu("Center"), ["Center", "Axis, End", "---", "Elliptical Arc"]);
        assert_eq!(menu("Arc").len(), 15);
    }
}
