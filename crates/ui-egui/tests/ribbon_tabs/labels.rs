//! Large labels wrap as in AutoCAD's screenshots.

use super::*;

// ---- Large labels ----

/// The texts the ribbon paints with `tab` open at 1906 px, as laid out ("\n" = line break).
fn painted_labels(tab: &str) -> Vec<String> {
    fn collect(shape: &egui::Shape, out: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(t) => out.push(t.galley.text().to_string()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|s| collect(s, out)),
            _ => {}
        }
    }
    let mut app = CadApp::new(Session::default(), Services::default());
    app.ui.ribbon_tab = tab.into();
    let mut h = Harness::builder().with_size(vec2(1906.0, 300.0)).build_ui_state(|ui, app: &mut CadApp| cadcraft_ui_egui::ribbon::show(app, ui), app);
    h.run_steps(3);
    let mut out = Vec::new();
    h.output().shapes.iter().for_each(|s| collect(&s.shape, &mut out));
    out
}

#[test]
fn large_labels_wrap_as_in_autocads_screenshots() {
    // Every large label of each tab as AutoCAD 2026 shows it at 1906 px: one word on one line;
    // more words on two, broken at the last space that keeps the first line short ("Detect and /
    // Convert", but "Add / Current Scale").
    for (tab, labels) in [
        (
            "Home",
            &[
                "Line",
                "Polyline",
                "Circle",
                "Arc",
                "Text",
                "Dimension",
                "Layer\nProperties",
                "Insert",
                "Detect and\nConvert",
                "Match\nProperties",
                "Group",
                "Measure",
                "Paste",
                "Base",
            ][..],
        ),
        (
            "Insert",
            &[
                "Insert",
                "Detect and\nConvert",
                "Create\nBlock",
                "Define\nAttributes",
                "Manage\nAttributes",
                "Block\nEditor",
                "Attach",
                "Clip",
                "Adjust",
                "PDF\nImport",
                "Combine\nText",
                "Field",
                "Data\nLink",
                "Set\nLocation",
            ][..],
        ),
        (
            "Annotate",
            &[
                "Multiline\nText",
                "Dimension",
                "Center\nMark",
                "Centerline",
                "Multileader",
                "Table",
                "Wipeout",
                "Revision\nCloud",
                "Add\nCurrent Scale",
            ][..],
        ),
        ("Parametric", &["Auto\nConstrain", "Linear", "Aligned", "Delete\nConstraints", "Parameters\nManager"][..]),
        (
            "View",
            &[
                "UCS\nIcon",
                "View\nCube",
                "Navigation\nBar",
                "Viewport\nConfiguration",
                "DWG\nCompare",
                "Activity\nInsights",
                "Tool\nPalettes",
                "Properties",
                "Blocks",
                "Count",
                "Command\nMacros",
                "Sheet Set\nManager",
                "Switch\nWindows",
                "File\nTabs",
                "Layout\nTabs",
            ][..],
        ),
        (
            "Manage",
            &[
                "Record",
                "User\nInterface",
                "Tool\nPalettes",
                "Load\nApplication",
                "Run\nScript",
                "Find\nNon-Purgeable Items",
                "Performance\nAnalyzer",
            ][..],
        ),
        ("Output", &["Plot", "Batch\nPlot", "Preview", "Export"][..]),
        ("Collaborate", &["Share\nDrawing", "Shared\nViews", "Push to\nAutodesk Docs", "Traces\nPalette", "Markup\nImport", "DWG\nCompare"][..]),
        (
            "Express Tools",
            &[
                "Explode\nAttributes",
                "Replace\nBlock",
                "Arc\nAligned",
                "Modify\nText",
                "Break-line\nSymbol",
                "Super\nHatch",
                "Annotation\nAttachment",
                "Command\nAliases",
                "URL\nOptions",
            ][..],
        ),
    ] {
        let painted = painted_labels(tab);
        let missing: Vec<_> = labels.iter().filter(|l| !painted.iter().any(|p| p == *l)).collect();
        assert!(missing.is_empty(), "{tab}: not painted as AutoCAD wraps them: {missing:?}");
    }
}
