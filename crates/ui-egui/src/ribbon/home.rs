//! The Home tab: AutoCAD 2026's ten panels with their items in AutoCAD's order (column by
//! column, top to bottom), sizes, rows, split menus, slide-outs and dialog launchers.

use super::model::{Combo, Entry, Item, Panel, Size, Slider, Tab};
use crate::icons::Icon;

const SEP: Entry = Entry::Separator;

const fn e(icon: Icon, label: &'static str, cmd: &'static str) -> Entry {
    Entry::Command { icon, label, cmd }
}

const fn large(icon: Icon, label: &'static str, cmd: &'static str) -> Item {
    Item::Button { icon, label, cmd, size: Size::Large, row: 0 }
}

/// Small, icon only.
const fn icon(icon: Icon, label: &'static str, cmd: &'static str, row: u8) -> Item {
    Item::Button { icon, label, cmd, size: Size::Small { label: false }, row }
}

/// Small, icon and label.
const fn labeled(icon: Icon, label: &'static str, cmd: &'static str, row: u8) -> Item {
    Item::Button { icon, label, cmd, size: Size::Small { label: true }, row }
}

const fn large_split(label: Option<&'static str>, menu: &'static [Entry]) -> Item {
    Item::Split { label, size: Size::Large, row: 0, menu, default: 0 }
}

const fn small_split(label: Option<&'static str>, show_label: bool, row: u8, menu: &'static [Entry]) -> Item {
    Item::Split { label, size: Size::Small { label: show_label }, row, menu, default: 0 }
}

const fn combo(kind: Combo, width: f32, row: u8) -> Item {
    Item::Combo { kind, width, row }
}

// ---- Draw ----

const CIRCLE: &[Entry] = &[
    e(Icon::Circle, "Center, Radius", "circle"),
    e(Icon::CircleDiameter, "Center, Diameter", "circle.cd"),
    SEP,
    e(Icon::Circle2Point, "2-Point", "circle.2p"),
    e(Icon::Circle3Point, "3-Point", "circle.3p"),
    SEP,
    e(Icon::CircleTtr, "Tan, Tan, Radius", "circle.ttr"),
    e(Icon::CircleTtt, "Tan, Tan, Tan", "circle.ttt"),
];

const ARC: &[Entry] = &[
    e(Icon::Arc, "3-Point", "arc"),
    SEP,
    e(Icon::ArcSce, "Start, Center, End", "arc.sce"),
    e(Icon::ArcSca, "Start, Center, Angle", "arc.sca"),
    e(Icon::ArcScl, "Start, Center, Length", "arc.scl"),
    SEP,
    e(Icon::ArcSea, "Start, End, Angle", "arc.sea"),
    e(Icon::ArcSed, "Start, End, Direction", "arc.sed"),
    e(Icon::ArcSer, "Start, End, Radius", "arc.ser"),
    SEP,
    e(Icon::ArcCse, "Center, Start, End", "arc.cse"),
    e(Icon::ArcCsa, "Center, Start, Angle", "arc.csa"),
    e(Icon::ArcCsl, "Center, Start, Length", "arc.csl"),
    SEP,
    e(Icon::ArcContinue, "Continue", "arc.continue"),
];

const RECTANGLE: &[Entry] = &[e(Icon::Rectangle, "Rectangle", "rectang"), e(Icon::Polygon, "Polygon", "polygon")];

const ELLIPSE: &[Entry] = &[
    e(Icon::Ellipse, "Center", "ellipse"),
    e(Icon::EllipseAxis, "Axis, End", "ellipse.axis"),
    SEP,
    e(Icon::EllipseArc, "Elliptical Arc", "ellipse.arc"),
];

const HATCH: &[Entry] = &[e(Icon::Hatch, "Hatch", "hatch"), e(Icon::Gradient, "Gradient", "gradient"), e(Icon::Boundary, "Boundary", "boundary")];

const REVCLOUD: &[Entry] = &[
    e(Icon::RevCloud, "Rectangular", "revcloud"),
    e(Icon::RevCloudPolygonal, "Polygonal", "revcloud.polygonal"),
    e(Icon::RevCloudFreehand, "Freehand", "revcloud.freehand"),
];

const DRAW: Panel = Panel {
    label: "Draw",
    items: &[
        large(Icon::Line, "Line", "line"),
        large(Icon::Polyline, "Polyline", "pline"),
        large_split(Some("Circle"), CIRCLE),
        large_split(Some("Arc"), ARC),
        small_split(None, false, 1, RECTANGLE),
        small_split(None, false, 2, ELLIPSE),
        small_split(None, false, 3, HATCH),
    ],
    slideout: &[
        icon(Icon::Spline, "Spline Fit", "spline", 1),
        icon(Icon::SplineCv, "Spline CV", "spline.cv", 1),
        icon(Icon::XLine, "Construction Line", "xline", 1),
        icon(Icon::Ray, "Ray", "ray", 1),
        icon(Icon::Point, "Multiple Points", "point.multiple", 1),
        icon(Icon::Divide, "Divide", "divide", 1),
        icon(Icon::MeasurePoints, "Measure", "measure", 1),
        icon(Icon::Region, "Region", "region", 2),
        icon(Icon::Wipeout, "Wipeout", "wipeout", 2),
        icon(Icon::Polyline3d, "3D Polyline", "3dpoly", 2),
        icon(Icon::Helix, "Helix", "helix", 2),
        icon(Icon::Donut, "Donut", "donut", 2),
        small_split(Some("Revision Cloud"), false, 2, REVCLOUD),
    ],
    launcher: None,
};

// ---- Modify ----

const TRIM: &[Entry] = &[e(Icon::Trim, "Trim", "trim"), e(Icon::Extend, "Extend", "extend")];

const FILLET: &[Entry] = &[e(Icon::Fillet, "Fillet", "fillet"), e(Icon::Chamfer, "Chamfer", "chamfer"), e(Icon::Blend, "Blend Curves", "blend")];

const ARRAY: &[Entry] = &[
    e(Icon::ArrayRect, "Rectangular Array", "arrayrect"),
    e(Icon::ArrayPath, "Path Array", "arraypath"),
    e(Icon::ArrayPolar, "Polar Array", "arraypolar"),
];

const DRAWORDER: &[Entry] = &[
    e(Icon::DrawOrderFront, "Bring to Front", "draworder.front"),
    e(Icon::DrawOrderBack, "Send to Back", "draworder.back"),
    e(Icon::DrawOrderAbove, "Bring Above Objects", "draworder.above"),
    e(Icon::DrawOrderUnder, "Send Under Objects", "draworder.under"),
    SEP,
    e(Icon::TextToFront, "Bring Text to Front", "texttofront.text"),
    e(Icon::DimsToFront, "Bring Dimensions to Front", "texttofront.dimensions"),
    e(Icon::LeadersToFront, "Bring Leaders to Front", "texttofront.leaders"),
    e(Icon::AnnotationsToFront, "Bring All Annotations to Front", "texttofront.all"),
    e(Icon::HatchToBack, "Send Hatches to Back", "hatchtoback"),
];

const MODIFY: Panel = Panel {
    label: "Modify",
    items: &[
        labeled(Icon::Move, "Move", "move", 1),
        labeled(Icon::Copy, "Copy", "copy", 2),
        labeled(Icon::Stretch, "Stretch", "stretch", 3),
        labeled(Icon::Rotate, "Rotate", "rotate", 1),
        labeled(Icon::Mirror, "Mirror", "mirror", 2),
        labeled(Icon::Scale, "Scale", "scale", 3),
        small_split(None, true, 1, TRIM),
        small_split(None, true, 2, FILLET),
        small_split(Some("Array"), true, 3, ARRAY),
        icon(Icon::Erase, "Erase", "erase", 1),
        icon(Icon::Explode, "Explode", "explode", 2),
        icon(Icon::Offset, "Offset", "offset", 3),
    ],
    slideout: &[
        icon(Icon::SetByLayer, "Set to ByLayer", "setbylayer", 1),
        icon(Icon::ChangeSpace, "Change Space", "chspace", 1),
        icon(Icon::Lengthen, "Lengthen", "lengthen", 1),
        icon(Icon::EditPolyline, "Edit Polyline", "pedit", 1),
        icon(Icon::EditSpline, "Edit Spline", "splinedit", 1),
        icon(Icon::EditHatch, "Edit Hatch", "hatchedit", 1),
        icon(Icon::EditArray, "Edit Array", "arrayedit", 1),
        icon(Icon::Align, "Align", "align", 2),
        icon(Icon::Break, "Break", "break", 2),
        icon(Icon::BreakAtPoint, "Break at Point", "breakatpoint", 2),
        icon(Icon::Join, "Join", "join", 2),
        icon(Icon::Reverse, "Reverse", "reverse", 2),
        icon(Icon::CopyNested, "Copy Nested Objects", "ncopy", 2),
        icon(Icon::Overkill, "Delete Duplicate Objects", "overkill", 2),
        small_split(None, false, 2, DRAWORDER),
    ],
    launcher: None,
};

// ---- Annotation ----

const TEXT: &[Entry] = &[e(Icon::MText, "Multiline Text", "mtext"), e(Icon::Text, "Single Line", "text")];

const DIMENSION: &[Entry] = &[
    e(Icon::DimLinear, "Linear", "dimlinear"),
    e(Icon::DimAligned, "Aligned", "dimaligned"),
    e(Icon::DimAngular, "Angular", "dimangular"),
    SEP,
    e(Icon::DimArc, "Arc Length", "dimarc"),
    e(Icon::DimRadius, "Radius", "dimradius"),
    e(Icon::DimDiameter, "Diameter", "dimdiameter"),
    SEP,
    e(Icon::DimOrdinate, "Ordinate", "dimordinate"),
    e(Icon::DimJogged, "Jogged", "dimjogged"),
];

const LEADER: &[Entry] = &[
    e(Icon::MLeader, "Leader", "mleader"),
    SEP,
    e(Icon::AddLeader, "Add Leader", "mleaderedit.add"),
    e(Icon::RemoveLeader, "Remove Leader", "mleaderedit.remove"),
    SEP,
    e(Icon::AlignLeaders, "Align", "mleaderalign"),
    e(Icon::CollectLeaders, "Collect", "mleadercollect"),
];

const ANNOTATION: Panel = Panel {
    label: "Annotation",
    items: &[
        large_split(Some("Text"), TEXT),
        large(Icon::Dimension, "Dimension", "dim"),
        small_split(None, true, 1, DIMENSION),
        small_split(None, true, 2, LEADER),
        labeled(Icon::Table, "Table", "table", 3),
    ],
    slideout: &[
        icon(Icon::TextStyle, "Text Style", "style", 1),
        combo(Combo::TextStyle, 200.0, 1),
        icon(Icon::DimStyle, "Dimension Style", "dimstyle", 2),
        combo(Combo::DimStyle, 200.0, 2),
        icon(Icon::MLeaderStyle, "Multileader Style", "mleaderstyle", 3),
        combo(Combo::MLeaderStyle, 200.0, 3),
        icon(Icon::TableStyle, "Table Style", "tablestyle", 4),
        combo(Combo::TableStyle, 200.0, 4),
    ],
    launcher: None,
};

// ---- Layers ----

const LAYERS: Panel = Panel {
    label: "Layers",
    items: &[
        large(Icon::LayerProps, "Layer Properties", "layer"),
        combo(Combo::Layer, 208.0, 1),
        icon(Icon::LayerOff, "Off", "layoff", 2),
        icon(Icon::LayerIso, "Isolate", "layiso", 2),
        icon(Icon::LayerFreeze, "Freeze", "layfrz", 2),
        icon(Icon::LayerLock, "Lock", "laylck", 2),
        labeled(Icon::MakeCurrent, "Make Current", "laymcur", 2),
        icon(Icon::LayerOn, "Turn All Layers On", "layon", 3),
        icon(Icon::LayerUniso, "Unisolate", "layuniso", 3),
        icon(Icon::LayerThaw, "Thaw All Layers", "laythw", 3),
        icon(Icon::LayerUnlock, "Unlock", "layulk", 3),
        labeled(Icon::LayerMatch, "Match Layer", "laymch", 3),
    ],
    slideout: &[
        combo(Combo::LayerState, 209.0, 1),
        icon(Icon::LayerPrev, "Previous", "layerp", 2),
        icon(Icon::LayerCur, "Change to Current Layer", "laycur", 2),
        icon(Icon::CopyToLayer, "Copy Objects to New Layer", "copytolayer", 2),
        icon(Icon::LayerWalk, "Layer Walk", "laywalk", 2),
        icon(Icon::LayerVpFreeze, "VP Freeze in All Viewports except Current", "layvpi", 2),
        icon(Icon::LayerMerge, "Merge", "laymrg", 2),
        icon(Icon::LayerDelete, "Delete", "laydel", 2),
        icon(Icon::LockedLayerFade, "Locked Layer fading", "laylockfadectl", 3),
        Item::Slider { kind: Slider::LockedLayerFade, width: 200.0, row: 3 },
    ],
    launcher: None,
};

// ---- Block ----

const DETECT: &[Entry] = &[e(Icon::DetectBlocks, "Detect and Convert", "bdetect"), e(Icon::ConvertBlocks, "Search and Convert", "bconvert")];

const EDIT_ATTRIBUTES: &[Entry] = &[e(Icon::AttEdit, "Single", "attedit"), e(Icon::AttEditMultiple, "Multiple", "-attedit")];

const BLOCK: Panel = Panel {
    label: "Block",
    items: &[
        large(Icon::Insert, "Insert", "insert"),
        large_split(None, DETECT),
        labeled(Icon::BlockCreate, "Create", "block", 1),
        labeled(Icon::BlockEdit, "Edit", "bedit", 2),
        small_split(Some("Edit Attributes"), true, 3, EDIT_ATTRIBUTES),
    ],
    slideout: &[
        icon(Icon::BlockReplace, "Replace", "breplace", 1),
        icon(Icon::AttDef, "Define Attributes", "attdef", 1),
        icon(Icon::AttManager, "Manage Attributes", "battman", 1),
        icon(Icon::AttSync, "Synchronize Attributes", "attsync", 1),
        combo(Combo::AttributeDisplay, 35.0, 1),
        icon(Icon::BasePoint, "Set Base Point", "base", 1),
    ],
    launcher: None,
};

// ---- Properties ----

const PROPERTIES: Panel = Panel {
    label: "Properties",
    items: &[
        large(Icon::MatchProp, "Match Properties", "matchprop"),
        combo(Combo::Color, 200.0, 1),
        combo(Combo::Lineweight, 200.0, 2),
        combo(Combo::Linetype, 200.0, 3),
    ],
    slideout: &[
        combo(Combo::PlotStyle, 200.0, 1),
        combo(Combo::Transparency, 35.0, 2),
        Item::Slider { kind: Slider::Transparency, width: 158.0, row: 2 },
        labeled(Icon::List, "List", "list", 3),
    ],
    launcher: Some("properties"),
};

// ---- Groups ----

const GROUPS: Panel = Panel {
    label: "Groups",
    items: &[
        large(Icon::Group, "Group", "group"),
        icon(Icon::Ungroup, "Ungroup", "ungroup", 1),
        icon(Icon::GroupEdit, "Group Edit", "groupedit", 2),
        icon(Icon::GroupSelect, "Group Selection On/Off", "pickstyle", 3),
    ],
    slideout: &[
        labeled(Icon::GroupManager, "Group Manager", "classicgroup", 1),
        labeled(Icon::GroupBox, "Group Bounding Box", "groupdisplaymode", 2),
    ],
    launcher: None,
};

// ---- Utilities ----

const MEASURE: &[Entry] = &[
    e(Icon::Measure, "Quick", "measuregeom.quick"),
    e(Icon::Distance, "Distance", "dist"),
    e(Icon::MeasureRadius, "Radius", "radius.measure"),
    e(Icon::MeasureAngle, "Angle", "angle.measure"),
    e(Icon::Area, "Area", "area"),
    e(Icon::Volume, "Volume", "measuregeom.volume"),
];

const UTILITIES: Panel = Panel {
    label: "Utilities",
    items: &[
        large_split(Some("Measure"), MEASURE),
        icon(Icon::QuickSelect, "Quick Select", "qselect", 1),
        icon(Icon::SelectAll, "Select All", "selectall", 2),
        icon(Icon::QuickCalc, "Quick Calculator", "ui.panel.quickcalc", 3),
    ],
    slideout: &[labeled(Icon::IdPoint, "ID Point", "id", 1), labeled(Icon::PointStyle, "Point Style...", "ddptype", 2)],
    launcher: None,
};

// ---- Clipboard ----

const PASTE: &[Entry] = &[
    e(Icon::Paste, "Paste", "pasteclip"),
    e(Icon::PasteBlock, "Paste as Block", "pasteblock"),
    e(Icon::PasteHyperlink, "Paste as Hyperlink", "pastehlnk"),
    e(Icon::PasteOrig, "Paste to Original Coordinates", "pasteorig"),
    e(Icon::PasteSpecial, "Paste Special", "pastespec"),
];

const CLIPBOARD: Panel = Panel {
    label: "Clipboard",
    items: &[large_split(None, PASTE), labeled(Icon::Cut, "Cut", "cutclip", 1), labeled(Icon::CopyClip, "Copy Clip", "copyclip", 2)],
    slideout: &[],
    launcher: None,
};

// ---- View ----

const VIEW: Panel = Panel {
    label: "View",
    items: &[large(Icon::ViewBase, "Base", "viewbase")],
    slideout: &[
        icon(Icon::SectionViewStyle, "Section View Style", "viewsectionstyle", 1),
        combo(Combo::SectionViewStyle, 140.0, 1),
        icon(Icon::DetailViewStyle, "Detail View Style", "viewdetailstyle", 2),
        combo(Combo::DetailViewStyle, 140.0, 2),
    ],
    launcher: Some("viewstd"),
};

pub const HOME: Tab = Tab { label: "Home", panels: &[DRAW, MODIFY, ANNOTATION, LAYERS, BLOCK, PROPERTIES, GROUPS, UTILITIES, CLIPBOARD, VIEW] };
