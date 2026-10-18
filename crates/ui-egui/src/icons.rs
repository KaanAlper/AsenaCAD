//! AsenaCAD's icon set, drawn in code (original work, MIT OR Apache-2.0 with the codebase).
//!
//! Icons are line drawings on a 24×24 design grid: light strokes for existing geometry, blue for
//! the geometry a command creates and orange dots for picked points.

use craft_ui::buttons::{ButtonVisuals, IconButton, IconButtonStyle};
use egui::{Color32, Painter, Rect, Stroke};

use crate::theme::Tokens;

mod glyphs;
mod pen;
mod ribbon;
mod shell;

use glyphs::paint_pen;
use pen::Pen;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    New,
    Open,
    Save,
    SaveAs,
    Undo,
    Redo,
    Plot,
    Publish,
    PageSetup,
    Preview,
    Import,
    Export,
    Attach,
    Share,
    ZoomWindow,
    Pan,
    Orbit,
    ZoomExtents,
    Properties,
    Blocks,
    Layers,
    Help,
    Line,
    Polyline,
    Circle,
    Arc,
    Rectangle,
    Polygon,
    Ellipse,
    Spline,
    Point,
    XLine,
    Ray,
    Donut,
    RevCloud,
    Hatch,
    Gradient,
    Boundary,
    Region,
    Wipeout,
    Table,
    Text,
    MText,
    Move,
    Copy,
    Rotate,
    Scale,
    Mirror,
    Offset,
    Trim,
    Extend,
    Fillet,
    Chamfer,
    Explode,
    Erase,
    Stretch,
    ArrayRect,
    ArrayPolar,
    Break,
    Join,
    Lengthen,
    MatchProp,
    DimLinear,
    DimAligned,
    DimRadius,
    DimDiameter,
    DimAngular,
    DimArc,
    DimOrdinate,
    DimBaseline,
    DimContinue,
    QDim,
    MLeader,
    AddLeader,
    RemoveLeader,
    AlignLeaders,
    BlockCreate,
    Insert,
    BlockEdit,
    AttDef,
    LayerProps,
    LayerOff,
    LayerIso,
    LayerFreeze,
    LayerLock,
    LayerMatch,
    MakeCurrent,
    LayerPrev,
    Bulb,
    BulbOff,
    Lock,
    Unlock,
    Sun,
    Snowflake,
    Grid,
    Snap,
    Ortho,
    Polar,
    Osnap,
    OTrack,
    Lineweight,
    Transparency,
    Isodraft,
    DynInput,
    QuickProps,
    Gear,
    Annotation,
    Workspace,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
    /// A ribbon panel's dialog launcher: an arrow pointing down-right into a corner.
    Launcher,
    /// A slide-out's pin: pinned (upright pushpin) and unpinned (lying on its side).
    Pin,
    Unpin,
    Plus,
    Switcher,
    Close,
    Search,
    Menu,
    Constraint,
    Coincident,
    Parallel,
    Perpendicular,
    Horizontal,
    Vertical,
    Tangent,
    Concentric,
    Equal,
    Fix,
    Measure,
    Distance,
    Area,
    List,
    PickAdd,
    SelectObjects,
    QuickSelect,
    // Home ribbon variants and panel tools, drawn in `icons/ribbon.rs`.
    CircleDiameter,
    Circle2Point,
    Circle3Point,
    CircleTtr,
    CircleTtt,
    ArcSce,
    ArcSca,
    ArcScl,
    ArcSea,
    ArcSed,
    ArcSer,
    ArcCse,
    ArcCsa,
    ArcCsl,
    ArcContinue,
    EllipseAxis,
    EllipseArc,
    SplineCv,
    Divide,
    MeasurePoints,
    Polyline3d,
    Helix,
    RevCloudPolygonal,
    RevCloudFreehand,
    Blend,
    ArrayPath,
    SetByLayer,
    ChangeSpace,
    EditPolyline,
    EditSpline,
    EditHatch,
    EditArray,
    Align,
    BreakAtPoint,
    Reverse,
    CopyNested,
    Overkill,
    DrawOrderFront,
    DrawOrderBack,
    DrawOrderAbove,
    DrawOrderUnder,
    TextToFront,
    DimsToFront,
    LeadersToFront,
    AnnotationsToFront,
    HatchToBack,
    Dimension,
    DimJogged,
    CollectLeaders,
    TextStyle,
    DimStyle,
    MLeaderStyle,
    TableStyle,
    LayerOn,
    LayerUniso,
    LayerThaw,
    LayerUnlock,
    LayerCur,
    CopyToLayer,
    LayerWalk,
    LayerVpFreeze,
    LayerMerge,
    LayerDelete,
    LockedLayerFade,
    DetectBlocks,
    ConvertBlocks,
    AttEdit,
    AttEditMultiple,
    BlockReplace,
    AttManager,
    AttSync,
    BasePoint,
    Group,
    Ungroup,
    GroupEdit,
    GroupSelect,
    GroupManager,
    GroupBox,
    MeasureRadius,
    MeasureAngle,
    Volume,
    SelectAll,
    QuickCalc,
    IdPoint,
    PointStyle,
    Paste,
    PasteBlock,
    PasteHyperlink,
    PasteOrig,
    PasteSpecial,
    Cut,
    CopyClip,
    ViewBase,
    SectionViewStyle,
    DetailViewStyle,
    // Insert tab tools, drawn in `icons/ribbon/insert.rs`.
    WriteBlock,
    XClip,
    UnderlayAdjust,
    UnderlayLayers,
    RefEdit,
    XrefFade,
    PdfImport,
    DgnImport,
    RecognizeShx,
    ShxSettings,
    CombineText,
    Field,
    UpdateFields,
    OleObject,
    Hyperlink,
    DataLink,
    DataDownload,
    DataUpload,
    DataExtract,
    GeoMap,
    GeoFile,
    // Annotate tab tools, drawn in `icons/ribbon/annotate.rs`.
    CheckSpelling,
    TextAlign,
    Justify,
    ScaleText,
    DimBreak,
    DimSpace,
    DimJogLine,
    DimInspect,
    DimUpdate,
    DimReassociate,
    Tolerance,
    DimOblique,
    DimTextAngle,
    DimTextLeft,
    DimTextCenter,
    DimTextRight,
    DimOverride,
    CenterMark,
    Centerline,
    AddScale,
    DeleteScale,
    ScaleList,
    AddDeleteScales,
    SyncScales,
    AutoScale,
    AnnoMonitor,
    Isolate,
    CleanScreen,
    SelCycling,
    Osnap3d,
    DynUcs,
    SelFilter,
    Gizmo,
    GraphicsPerf,
    Units,
    Wrench,
    Prompt,
    NavWheel,
    ShowMotion,
    VpMinimize,
    VpRestore,
    ZoomIn,
    ZoomOut,
    ZoomAll,
    ZoomPrevious,
    ZoomRealtime,
}

/// Paint `icon` into `rect` (square), with `dim` for disabled.
pub fn paint(p: &Painter, rect: Rect, icon: Icon, dim: bool) {
    let t = Tokens::get();
    let alpha = |c: Color32| if dim { c.gamma_multiply(0.4) } else { c };
    let pen =
        Pen { p, r: rect, w: (rect.width() / 16.0).clamp(1.0, 1.6), base: alpha(t.icon), acc: alpha(t.icon_accent), pt: alpha(t.icon_point), dim };
    paint_pen(&pen, icon);
}

/// Paint `icon` into `rect` in one colour (every stroke, accent and point), for glyphs on a
/// surface of their own such as the navigation bar's light strip.
pub fn paint_tinted(p: &Painter, rect: Rect, icon: Icon, color: Color32) {
    let pen = Pen { p, r: rect, w: (rect.width() / 16.0).clamp(1.0, 1.6), base: color, acc: color, pt: color, dim: false };
    paint_pen(&pen, icon);
}

/// A shared button interaction with AsenaCAD's original multicolor icon painting.
pub fn button(ui: &mut egui::Ui, icon: Icon, size: f32, tooltip: &str, selected: bool) -> egui::Response {
    let t = Tokens::get();
    let visuals = |fill| ButtonVisuals { fill, stroke: Stroke::NONE, icon: t.icon };
    let style = IconButtonStyle {
        normal: visuals(Color32::TRANSPARENT),
        hovered: visuals(t.control_hover.gamma_multiply(0.6)),
        pressed: None,
        selected: visuals(t.tab_active),
        corner_radius: egui::CornerRadius::same(3),
        focus_stroke: Stroke::new(1.0, t.accent),
    };
    let resp = IconButton::new(crate::i18n::t(tooltip), egui::vec2(size, size), &style).selected(selected).show(ui, |ui, rect, _| {
        // A single tint would erase the blue geometry and orange picked points. Keep the
        // original painter and its explicit dimming on the same parent-enabled UI.
        paint(ui.painter(), rect.shrink(size * 0.08), icon, !ui.is_enabled());
    });
    crate::control::record_widget(ui, &resp, tooltip, "button", Some(selected));
    resp.on_hover_text(crate::i18n::t(tooltip))
}

/// Labels and focus outlines for controls whose existing geometry is painted by the app.
/// Pointer focus is opt-in for navigation tabs; drafting buttons keep their command-line flow.
pub fn describe_control(ui: &egui::Ui, response: &egui::Response, label: &str, selected: Option<bool>, tab: bool) {
    let label = crate::i18n::t(label);
    response.widget_info(|| match selected {
        Some(selected) => egui::WidgetInfo::selected(egui::WidgetType::Button, response.enabled(), selected, label),
        None => egui::WidgetInfo::labeled(egui::WidgetType::Button, response.enabled(), label),
    });
    if tab {
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::Tab);
            node.clear_toggled();
            if let Some(selected) = selected {
                node.set_selected(selected);
            }
        });
        if response.clicked() {
            response.request_focus();
        }
    }
    if response.has_focus() && response.rect.width() > 4.0 && response.rect.height() > 4.0 {
        ui.painter().rect_stroke(response.rect.shrink(2.0), 2.0, Stroke::new(1.0, Tokens::get().accent), egui::StrokeKind::Inside);
    }
    crate::control::record_widget(ui, response, label, if tab { "tab" } else { "button" }, selected);
}
