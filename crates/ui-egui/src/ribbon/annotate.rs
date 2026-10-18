//! The Annotate tab: AutoCAD 2026's seven panels with their items in AutoCAD's order (column by
//! column, top to bottom), sizes, rows, split menus, slide-outs and dialog launchers.

use super::build::*;
use super::model::{Combo, Entry, Panel, Tab};
use crate::icons::Icon;

// ---- Text ----

const TEXT: &[Entry] = &[e(Icon::MText, "Multiline Text", "mtext"), e(Icon::Text, "Single Line", "text")];

const TEXT_PANEL: Panel = Panel {
    label: "Text",
    items: &[
        large_split(None, TEXT),
        icon(Icon::CheckSpelling, "Check Spelling", "spell", 1),
        icon(Icon::TextAlign, "Text Align", "textalign", 2),
        icon(Icon::Justify, "Justify", "justifytext", 3),
        combo(Combo::TextStyle, 200.0, 1),
        combo(Combo::FindText, 200.0, 2),
        combo(Combo::TextHeight, 200.0, 3),
    ],
    slideout: &[labeled(Icon::ScaleText, "Scale", "scaletext", 1), combo(Combo::TextLayer, 200.0, 2)],
    launcher: Some("style"),
};

// ---- Dimensions ----

/// AutoCAD's Annotate list ends Jogged, Ordinate (the Home tab's has them the other way round).
const LINEAR: &[Entry] = &[
    e(Icon::DimLinear, "Linear", "dimlinear"),
    e(Icon::DimAligned, "Aligned", "dimaligned"),
    e(Icon::DimAngular, "Angular", "dimangular"),
    SEP,
    e(Icon::DimArc, "Arc Length", "dimarc"),
    e(Icon::DimRadius, "Radius", "dimradius"),
    e(Icon::DimDiameter, "Diameter", "dimdiameter"),
    SEP,
    e(Icon::DimJogged, "Jogged", "dimjogged"),
    e(Icon::DimOrdinate, "Ordinate", "dimordinate"),
];

const CONTINUE: &[Entry] = &[e(Icon::DimContinue, "Continue", "dimcontinue"), e(Icon::DimBaseline, "Baseline", "dimbaseline")];

const DIMENSIONS: Panel = Panel {
    label: "Dimensions",
    items: &[
        large(Icon::Dimension, "Dimension", "dim"),
        combo(Combo::DimStyle, 200.0, 1),
        icon(Icon::DimBreak, "Break", "dimbreak", 1),
        icon(Icon::DimSpace, "Adjust Space", "dimspace", 1),
        icon(Icon::DimJogLine, "Jog Line", "dimjogline", 1),
        combo(Combo::DimLayer, 200.0, 2),
        icon(Icon::DimInspect, "Inspect", "diminspect", 2),
        icon(Icon::DimUpdate, "Update", "dimstyle.update", 2),
        icon(Icon::DimReassociate, "Reassociate", "dimreassociate", 2),
        small_split(None, true, 3, LINEAR),
        labeled(Icon::QDim, "Quick", "qdim", 3),
        small_split(None, true, 3, CONTINUE),
    ],
    slideout: &[
        icon(Icon::Tolerance, "Tolerance", "tolerance", 1),
        icon(Icon::DimOblique, "Oblique", "dimedit.oblique", 1),
        icon(Icon::DimTextAngle, "Text Angle", "dimtedit.angle", 1),
        icon(Icon::DimTextLeft, "Left Justify", "dimtedit.left", 1),
        icon(Icon::DimTextCenter, "Center Justify", "dimtedit.center", 1),
        icon(Icon::DimTextRight, "Right Justify", "dimtedit.right", 1),
        icon(Icon::DimOverride, "Override", "dimoverride", 1),
    ],
    launcher: Some("dimstyle"),
};

// ---- Centerlines ----

const CENTERLINES: Panel = Panel {
    label: "Centerlines",
    items: &[large(Icon::CenterMark, "Center Mark", "centermark"), large(Icon::Centerline, "Centerline", "centerline")],
    slideout: &[combo(Combo::CenterLayer, 200.0, 1)],
    launcher: None,
};

// ---- Leaders ----

const LEADERS: Panel = Panel {
    label: "Leaders",
    items: &[
        large(Icon::MLeader, "Multileader", "mleader"),
        combo(Combo::MLeaderStyle, 200.0, 1),
        combo(Combo::MLeaderLayer, 200.0, 2),
        icon(Icon::AlignLeaders, "Align", "mleaderalign", 3),
        icon(Icon::CollectLeaders, "Collect", "mleadercollect", 3),
        icon(Icon::AddLeader, "Add Leader", "mleaderedit.add", 3),
        icon(Icon::RemoveLeader, "Remove Leader", "mleaderedit.remove", 3),
    ],
    slideout: &[],
    launcher: Some("mleaderstyle"),
};

// ---- Tables ----

const TABLES: Panel = Panel {
    label: "Tables",
    items: &[
        large(Icon::Table, "Table", "table"),
        combo(Combo::TableStyle, 200.0, 1),
        icon(Icon::DataDownload, "Download from Source", "datalinkupdate", 2),
        labeled(Icon::DataExtract, "Extract Data", "dataextraction", 2),
        icon(Icon::DataUpload, "Upload to Source", "datalinkupdate.write", 3),
        labeled(Icon::DataLink, "Link Data", "datalink", 3),
    ],
    slideout: &[],
    launcher: Some("tablestyle"),
};

// ---- Markup ----

const MARKUP: Panel = Panel {
    label: "Markup",
    items: &[large(Icon::Wipeout, "Wipeout", "wipeout"), large_split(Some("Revision Cloud"), super::home::REVCLOUD)],
    slideout: &[],
    launcher: None,
};

// ---- Annotation Scaling ----

const ADD_SCALE: &[Entry] =
    &[e(Icon::AddScale, "Add Current Scale", "aiobjectscaleadd"), e(Icon::DeleteScale, "Delete Current Scale", "aiobjectscaleremove")];

const SCALING: Panel = Panel {
    label: "Annotation Scaling",
    items: &[
        large_split(None, ADD_SCALE),
        labeled(Icon::ScaleList, "Scale List", "scalelistedit", 1),
        labeled(Icon::AddDeleteScales, "Add/Delete Scales", "objectscale", 2),
        labeled(Icon::SyncScales, "Sync Scale Positions", "annoreset", 3),
    ],
    slideout: &[],
    launcher: None,
};

pub const ANNOTATE: Tab = Tab { label: "Annotate", panels: &[TEXT_PANEL, DIMENSIONS, CENTERLINES, LEADERS, TABLES, MARKUP, SCALING] };
