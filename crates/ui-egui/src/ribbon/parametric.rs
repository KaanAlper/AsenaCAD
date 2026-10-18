//! The Parametric tab: AutoCAD 2026's three panels with their items in AutoCAD's order (column
//! by column, top to bottom), sizes, rows, split menus, slide-outs and dialog launchers.

use super::build::*;
use super::model::{Entry, Panel, Size, Tab, Toggle};
use crate::icons::Icon;

// ---- Geometric ----

const GEOMETRIC: Panel = Panel {
    label: "Geometric",
    items: &[
        large(Icon::AutoConstrain, "Auto Constrain", "autoconstrain"),
        icon(Icon::Coincident, "Coincident", "gccoincident", 1),
        icon(Icon::Collinear, "Collinear", "gccollinear", 1),
        icon(Icon::Concentric, "Concentric", "gcconcentric", 1),
        icon(Icon::Fix, "Fix", "gcfix", 1),
        icon(Icon::Parallel, "Parallel", "gcparallel", 2),
        icon(Icon::Perpendicular, "Perpendicular", "gcperpendicular", 2),
        icon(Icon::Horizontal, "Horizontal", "gchorizontal", 2),
        icon(Icon::Vertical, "Vertical", "gcvertical", 2),
        icon(Icon::Tangent, "Tangent", "gctangent", 3),
        icon(Icon::Smooth, "Smooth", "gcsmooth", 3),
        icon(Icon::Symmetric, "Symmetric", "gcsymmetric", 3),
        icon(Icon::Equal, "Equal", "gcequal", 3),
        labeled(Icon::BarsShowHide, "Show/Hide", "constraintbar", 1),
        labeled(Icon::BarsShowAll, "Show All", "constraintbar.showall", 2),
        labeled(Icon::BarsHideAll, "Hide All", "constraintbar.hideall", 3),
    ],
    slideout: &[],
    // Our Constraint Settings dialog shows its Geometric, Dimensional and AutoConstrain settings
    // on one page, so both launchers open the same dialog.
    launcher: Some("constraintsettings"),
    launcher_name: Some("Constraint Settings, Geometric"),
};

// ---- Dimensional ----

const LINEAR: &[Entry] =
    &[e(Icon::DcLinear, "Linear", "dclinear"), e(Icon::DcHorizontal, "Horizontal", "dchorizontal"), e(Icon::DcVertical, "Vertical", "dcvertical")];

const DIMENSIONAL: Panel = Panel {
    label: "Dimensional",
    items: &[
        large_split(None, LINEAR),
        large(Icon::DcAligned, "Aligned", "dcaligned"),
        icon(Icon::DcRadius, "Radius", "dcradius", 1),
        icon(Icon::DcDiameter, "Diameter", "dcdiameter", 1),
        icon(Icon::DcAngular, "Angular", "dcangular", 2),
        icon(Icon::DcConvert, "Convert", "dcconvert", 2),
        labeled(Icon::DcShowHide, "Show/Hide", "dcdisplay", 1),
        labeled(Icon::DcShowAll, "Show All", "dcdisplay.showall", 2),
        labeled(Icon::DcHideAll, "Hide All", "dcdisplay.hideall", 3),
    ],
    slideout: &[
        // AutoCAD sets CCONSTRAINTFORM (0 dynamic, 1 annotational); it isn't modelled yet.
        toggle(
            Icon::DynamicConstraints,
            "Dynamic Constraint Mode",
            "cconstraintform.dynamic",
            Size::Small { label: true },
            1,
            Toggle::DynamicConstraints,
        ),
        toggle(
            Icon::AnnotationalConstraints,
            "Annotational Constraint Mode",
            "cconstraintform.annotational",
            Size::Small { label: true },
            2,
            Toggle::AnnotationalConstraints,
        ),
    ],
    launcher: Some("constraintsettings"),
    launcher_name: Some("Constraint Settings, Dimensional"),
};

// ---- Manage ----

const MANAGE: Panel = Panel {
    label: "Manage",
    items: &[
        large(Icon::DelConstraint, "Delete Constraints", "delconstraint"),
        toggle(Icon::ParametersManager, "Parameters Manager", "parameters", Size::Large, 0, Toggle::ParametersPalette),
    ],
    slideout: &[],
    launcher: None,
    launcher_name: None,
};

pub const PARAMETRIC: Tab = Tab { label: "Parametric", panels: &[GEOMETRIC, DIMENSIONAL, MANAGE] };
