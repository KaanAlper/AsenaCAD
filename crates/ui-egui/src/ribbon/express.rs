//! The Express Tools tab: AutoCAD 2026's eight panels of the Express Tools add-on with their
//! items in AutoCAD's order (column by column, top to bottom), sizes, rows, split menus and
//! slide-outs. Each item names its Express Tools command; the ones AsenaCAD has (NCOPY,
//! FLATTEN, ALIASEDIT) run, the others are listed without a command (shown disabled).

use super::build::*;
use super::model::{Entry, Panel, Tab};
use crate::icons::Icon;

// ---- Blocks ----

const BLOCKS: Panel = Panel {
    label: "Blocks",
    items: &[
        large(Icon::ExplodeAttributes, "Explode Attributes", "burst"),
        large(Icon::BlockReplace, "Replace Block", "blockreplace"),
        labeled(Icon::ListProperties, "List Properties", "xlist", 1),
        labeled(Icon::ImportAttributes, "Import Attributes", "attin", 2),
        labeled(Icon::ExportAttributes, "Export Attributes", "attout", 3),
    ],
    slideout: &[
        labeled(Icon::BlockToXref, "Convert block to xref", "blocktoxref", 1),
        labeled(Icon::CopyNested, "Copy Nested Objects", "ncopy", 2),
        labeled(Icon::ExtendNested, "Extend to Nested Objects", "bextend", 3),
        labeled(Icon::TrimNested, "Trim to Nested Objects", "btrim", 4),
    ],
    launcher: None,
    launcher_name: None,
};

// ---- Text ----

/// The text modifiers. Justify is JUSTIFYTEXT, which took over the Express Tools' own one.
const MODIFY_TEXT: &[Entry] = &[
    e(Icon::TextExplode, "Explode", "txtexp"),
    e(Icon::TextCase, "Change Case", "tcase"),
    e(Icon::TextRotate, "Rotate", "torient"),
    e(Icon::TextFit, "Fit", "textfit"),
    e(Icon::Justify, "Justify", "justifytext"),
];

const TEXT: Panel = Panel {
    label: "Text",
    items: &[
        large(Icon::ArcText, "Arc Aligned", "arctext"),
        large_split(Some("Modify Text"), MODIFY_TEXT),
        labeled(Icon::CombineText, "Convert to Mtext", "txt2mtxt", 1),
        labeled(Icon::AutoNumber, "Auto Number", "tcount", 2),
        labeled(Icon::EncloseText, "Enclose in Object", "tcircle", 3),
    ],
    slideout: &[
        labeled(Icon::RemoteText, "Remote Text", "rtext", 1),
        labeled(Icon::TextMask, "Text Mask", "textmask", 2),
        labeled(Icon::TextUnmask, "Unmask Text", "textunmask", 3),
    ],
    launcher: None,
    launcher_name: None,
};

// ---- Modify ----

const MODIFY: Panel = Panel {
    label: "Modify",
    items: &[labeled(Icon::MoveCopyRotate, "Move/Copy/Rotate", "mocoro", 1), labeled(Icon::StretchMultiple, "Stretch Multiple", "mstretch", 2)],
    slideout: &[
        labeled(Icon::Flatten, "Flatten objects", "flatten", 1),
        labeled(Icon::ExtendedClip, "Extended Clip", "clipit", 2),
        labeled(Icon::ExtendedOffset, "Extended Offset", "exoffset", 3),
        labeled(Icon::ShapeToBlock, "Convert Shape to Block", "shp2blk", 4),
    ],
    launcher: None,
    launcher_name: None,
};

// ---- Layout ----

const LAYOUT: Panel = Panel {
    label: "Layout",
    items: &[
        labeled(Icon::AlignSpace, "Align Space", "alignspace", 1),
        labeled(Icon::SyncViewports, "Synchronize Viewports", "vpsync", 2),
        labeled(Icon::MergeLayout, "Merge Layout", "layoutmerge", 3),
    ],
    slideout: &[labeled(Icon::ViewportScale, "List Viewport Scale", "vpscale", 1)],
    launcher: None,
    launcher_name: None,
};

// ---- Draw ----

const DRAW: Panel = Panel {
    label: "Draw",
    items: &[large(Icon::BreakLine, "Break-line Symbol", "breakline"), large(Icon::SuperHatch, "Super Hatch", "superhatch")],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Dimension ----

const ANNOTATION_ATTACHMENT: &[Entry] = &[
    e(Icon::AttachLeader, "Attach Leader to Annotation", "qlattach"),
    e(Icon::AttachLeaders, "Global Attach Leader to Annotation", "qlattachset"),
    e(Icon::DetachLeaders, "Detach Leaders from Annotation", "qldetachset"),
];

const DIMENSION: Panel = Panel {
    label: "Dimension",
    items: &[
        large_split(Some("Annotation Attachment"), ANNOTATION_ATTACHMENT),
        // DIMREASSOC puts back the measured value of dimensions whose text was overridden.
        labeled(Icon::ResetDimText, "Reset Text", "dimreassoc", 1),
        labeled(Icon::DimStyleImport, "Import Style", "dimim", 2),
        labeled(Icon::DimStyleExport, "Export Style", "dimex", 3),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

// ---- Tools ----

const TOOLS: Panel = Panel {
    label: "Tools",
    items: &[
        large(Icon::CommandAliases, "Command Aliases", "aliasedit"),
        labeled(Icon::AttachXdata, "Attach Xdata", "xdata", 1),
        labeled(Icon::ListXdata, "List Xdata", "xdlist", 2),
        labeled(Icon::SysVarEditor, "System Variables", "sysvdlg", 3),
    ],
    slideout: &[
        labeled(Icon::ExtendedPlan, "Extended Plan", "explan", 1),
        labeled(Icon::MakeLinetype, "Make Linetype", "mkltype", 2),
        labeled(Icon::MakeShape, "Make Shape", "mkshape", 3),
    ],
    launcher: None,
    launcher_name: None,
};

// ---- Web ----

/// AutoCAD's URL Options has a ▾ whose list wasn't captured: these are the Express Tools'
/// commands for the hyperlinks of the drawing's objects.
const URL_OPTIONS: &[Entry] = &[
    e(Icon::ShowUrls, "Show URLs", "showurls"),
    e(Icon::ChangeUrls, "Change URLs", "churls"),
    e(Icon::ReplaceUrls, "Find and Replace URLs", "repurls"),
];

const WEB: Panel =
    Panel { label: "Web", items: &[large_split(Some("URL Options"), URL_OPTIONS)], slideout: &[], launcher: None, launcher_name: None };

pub const EXPRESS: Tab = Tab { label: "Express Tools", panels: &[BLOCKS, TEXT, MODIFY, LAYOUT, DRAW, DIMENSION, TOOLS, WEB] };
