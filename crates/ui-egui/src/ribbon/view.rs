//! The View tab: AutoCAD 2026's seven panels with their items in AutoCAD's order (column by
//! column, top to bottom), sizes, rows, split menus, slide-outs and dialog launchers.

use super::build::*;
use super::model::{Combo, Entry, Panel, Size, Tab, Toggle};
use crate::icons::Icon;

const SMALL: Size = Size::Small { label: false };
const LABELED: Size = Size::Small { label: true };

// ---- Viewport Tools ----

const VIEWPORT_TOOLS: Panel = Panel {
    label: "Viewport Tools",
    items: &[
        large_toggle(Icon::UcsIcon, "UCS Icon", "ui.toggle.ucsicon", Toggle::UcsIcon),
        large_toggle(Icon::ViewCube, "View Cube", "ui.toggle.viewcube", Toggle::ViewCube),
        large_toggle(Icon::NavBar, "Navigation Bar", "navbar", Toggle::NavBar),
    ],
    slideout: &[toggle(Icon::ShowMotion, "Show Motion", "navsmotion", LABELED, 1, Toggle::NotBuilt)],
    launcher: None,
    launcher_name: None,
};

// ---- Named Views ----

const NAMED_VIEWS: Panel = Panel {
    label: "Named Views",
    items: &[
        combo(Combo::NamedView, 200.0, 1),
        labeled(Icon::NewView, "New View", "newview", 2),
        labeled(Icon::ViewManager, "View Manager", "view", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Model Viewports ----

/// The tiled viewport arrangements: VPORTS 1 and 4, and VPORTS 2 or 3 with an arrangement.
/// Four: Right and Four: Left are arrangements -VPORTS doesn't have yet.
const VIEWPORT_CONFIGURATION: &[Entry] = &[
    e(Icon::VpSingle, "Single", "vports.1"),
    e(Icon::VpTwoV, "Two: Vertical", "vports.2v"),
    e(Icon::VpTwoH, "Two: Horizontal", "vports.2h"),
    e(Icon::VpThreeR, "Three: Right", "vports.3r"),
    e(Icon::VpThreeL, "Three: Left", "vports.3l"),
    e(Icon::VpThreeA, "Three: Above", "vports.3a"),
    e(Icon::VpThreeB, "Three: Below", "vports.3b"),
    e(Icon::VpThreeV, "Three: Vertical", "vports.3v"),
    e(Icon::VpThreeH, "Three: Horizontal", "vports.3h"),
    e(Icon::VpFourE, "Four: Equal", "vports.4"),
    e(Icon::VpFourR, "Four: Right", "vports.4r"),
    e(Icon::VpFourL, "Four: Left", "vports.4l"),
];

const MODEL_VIEWPORTS: Panel = Panel {
    label: "Model Viewports",
    items: &[
        large_split(Some("Viewport Configuration"), VIEWPORT_CONFIGURATION),
        labeled(Icon::VpNamed, "Named", "vports", 1),
        labeled(Icon::VpJoin, "Join", "vports.join", 2),
        // -VPORTS Toggle: back and forth between one viewport and the last arrangement.
        labeled(Icon::VpToggle, "Restore", "vports.toggle", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Review, History ----

const REVIEW: Panel =
    Panel { label: "Review", items: &[large(Icon::DwgCompare, "DWG Compare", "compare")], slideout: &[], launcher: None, launcher_name: None };

const HISTORY: Panel = Panel {
    label: "History",
    items: &[large_toggle(Icon::ActivityInsights, "Activity Insights", "activityinsights", Toggle::NotBuilt)],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Palettes ----

const PALETTES: Panel = Panel {
    label: "Palettes",
    items: &[
        large_toggle(Icon::ToolPalettes, "Tool Palettes", "toolpalettes", Toggle::NotBuilt),
        large_toggle(Icon::Properties, "Properties", "properties", Toggle::PropertiesPalette),
        large_toggle(Icon::Blocks, "Blocks", "ui.dialog.blocks", Toggle::BlocksPalette),
        // COUNTLIST reports the counts on the command line; there is no Count palette yet.
        large_toggle(Icon::CountPalette, "Count", "countlist", Toggle::NotBuilt),
        large_toggle(Icon::CommandMacros, "Command Macros", "commandmacros", Toggle::NotBuilt),
        large_toggle(Icon::SheetSet, "Sheet Set Manager", "sheetset", Toggle::NotBuilt),
        toggle(Icon::CommandLine, "Command Line", "ui.toggle.cmdline", SMALL, 1, Toggle::CommandLine),
        toggle(Icon::MarkupSet, "Markup Set Manager", "markup", SMALL, 1, Toggle::NotBuilt),
        toggle(Icon::LayerProps, "Layer Properties", "layer", SMALL, 2, Toggle::LayerPalette),
        toggle(Icon::QuickCalc, "Quick Calculator", "ui.panel.quickcalc", SMALL, 2, Toggle::QuickCalcPalette),
        toggle(Icon::DesignCenter, "Design Center", "adcenter", SMALL, 3, Toggle::NotBuilt),
        toggle(Icon::XrefPalette, "External References", "xref", SMALL, 3, Toggle::XrefPalette),
    ],
    slideout: &[
        large(Icon::TextWindow, "Text Window", "textscr"),
        toggle(Icon::MaterialsBrowser, "Materials Browser", "matbrowseropen", LABELED, 1, Toggle::NotBuilt),
        toggle(Icon::MaterialsEditor, "Materials Editor", "mateditoropen", LABELED, 2, Toggle::NotBuilt),
        toggle(Icon::VisualStyles, "Visual Styles", "visualstyles", LABELED, 3, Toggle::NotBuilt),
        toggle(Icon::RenderSettings, "Render Settings", "rpref", SMALL, 1, Toggle::NotBuilt),
        toggle(Icon::LightsInModel, "Lights in Model", "lightlist", SMALL, 2, Toggle::NotBuilt),
        toggle(Icon::SunProperties, "Sun Properties", "sunproperties", SMALL, 3, Toggle::NotBuilt),
    ],
    launcher: None,
    launcher_name: None,
};

// ---- Interface ----

/// AutoCAD lists the open drawings here; our static list has no entry for each drawing yet.
const SWITCH_WINDOWS: &[Entry] = &[e(Icon::SwitchWindows, "Switch Windows", "switchwindows")];

const INTERFACE: Panel = Panel {
    label: "Interface",
    items: &[
        large_menu(Icon::SwitchWindows, "Switch Windows", SWITCH_WINDOWS),
        large_toggle(Icon::FileTabs, "File Tabs", "ui.toggle.filetabs", Toggle::FileTabs),
        large_toggle(Icon::LayoutTabs, "Layout Tabs", "ui.toggle.layouttabs", Toggle::LayoutTabs),
        // SYSWINDOWS arranges the drawing windows; AsenaCAD shows one drawing at a time.
        labeled(Icon::TileHorizontal, "Tile Horizontally", "syswindows.tilehorz", 1),
        labeled(Icon::TileVertical, "Tile Vertically", "syswindows.tilevert", 2),
        labeled(Icon::Cascade, "Cascade", "syswindows.cascade", 3),
    ],
    slideout: &[],
    launcher: Some("options"),
    launcher_name: Some("Options, Display Tab"),
};

pub const VIEW: Tab = Tab { label: "View", panels: &[VIEWPORT_TOOLS, NAMED_VIEWS, MODEL_VIEWPORTS, REVIEW, HISTORY, PALETTES, INTERFACE] };
