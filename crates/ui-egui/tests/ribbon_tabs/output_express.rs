//! Output, Collaborate and Express Tools.

use super::*;

#[test]
fn output_matches_autocad() {
    check_tab(
        "Output",
        &[
            WantPanel {
                label: "Plot",
                items: &[
                    b("Plot", "large", 0, "plot"),
                    // PUBLISH: AutoCAD's Batch Plot opens the Publish dialog.
                    b("Batch Plot", "large", 0, "publish"),
                    b("Preview", "large", 0, "preview"),
                    b("Page Setup Manager", "small+label", 1, "pagesetup"),
                    b("View Details", "small+label", 2, "viewplotdetails"),
                    b("Plotter Manager", "small+label", 3, "plottermanager"),
                ],
                slideout: &[],
                launcher: Some(("plot", "Plot, Plot Options Dialog")),
            },
            WantPanel {
                label: "Export to DWF/PDF",
                items: &[
                    sp("Export", "large", 0, &[("DWFx", "exportdwfx"), ("DWF", "exportdwf"), ("PDF", "exportpdf")], 0),
                    // Both lists are named "EplotExport" in AutoCAD; they show "Export: Display"
                    // and "Page Setup: Current".
                    cb("EplotExport", "ExportArea", 1, 200.0),
                    b("Export Window", "small", 1, "eplotexport.window"),
                    cb("EplotExport", "ExportPageSetup", 2, 200.0),
                    b("Page Setup", "small", 2, "eplotexport.pagesetup"),
                    b("Preview", "small", 3, "eplotexport.preview"),
                    b("Export to DWF Options", "small", 3, "exportsettings.dwf"),
                    b("Export to PDF Options", "small", 3, "exportsettings.pdf"),
                ],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn output_is_as_wide_as_autocads() {
    check_widths("Output", &[("Plot", 268.0), ("Export to DWF/PDF", 294.0)]);
}

#[test]
fn collaborate_matches_autocad() {
    check_tab(
        "Collaborate",
        &[
            WantPanel {
                label: "Share",
                items: &[b("Share Drawing", "large", 0, "share"), tg("Shared Views", "large", 0, "sharedviews")],
                slideout: &[],
                launcher: None,
            },
            WantPanel { label: "Autodesk Docs", items: &[tg("Push to Autodesk Docs", "large", 0, "pushtodocsopen")], slideout: &[], launcher: None },
            WantPanel {
                label: "Traces",
                items: &[tg("Traces Palette", "large", 0, "tracepaletteopen"), b("Markup Import", "large", 0, "markupimport")],
                slideout: &[],
                launcher: None,
            },
            WantPanel { label: "Compare", items: &[b("DWG Compare", "large", 0, "compare")], slideout: &[], launcher: None },
        ],
    );
}

#[test]
fn collaborate_is_as_wide_as_autocads() {
    check_widths("Collaborate", &[("Share", 97.0), ("Autodesk Docs", 90.0), ("Traces", 94.0), ("Compare", 60.0)]);
}

#[test]
fn express_tools_matches_autocad() {
    check_tab(
        "Express Tools",
        &[
            WantPanel {
                label: "Blocks",
                items: &[
                    b("Explode Attributes", "large", 0, "burst"),
                    b("Replace Block", "large", 0, "blockreplace"),
                    b("List Properties", "small+label", 1, "xlist"),
                    b("Import Attributes", "small+label", 2, "attin"),
                    b("Export Attributes", "small+label", 3, "attout"),
                ],
                slideout: &[
                    b("Convert block to xref", "small+label", 1, "blocktoxref"),
                    b("Copy Nested Objects", "small+label", 2, "ncopy"),
                    b("Extend to Nested Objects", "small+label", 3, "bextend"),
                    b("Trim to Nested Objects", "small+label", 4, "btrim"),
                ],
                launcher: None,
            },
            WantPanel {
                label: "Text",
                items: &[
                    b("Arc Aligned", "large", 0, "arctext"),
                    sp(
                        "Modify Text",
                        "large",
                        0,
                        &[("Explode", "txtexp"), ("Change Case", "tcase"), ("Rotate", "torient"), ("Fit", "textfit"), ("Justify", "justifytext")],
                        0,
                    ),
                    b("Convert to Mtext", "small+label", 1, "txt2mtxt"),
                    b("Auto Number", "small+label", 2, "tcount"),
                    b("Enclose in Object", "small+label", 3, "tcircle"),
                ],
                slideout: &[
                    b("Remote Text", "small+label", 1, "rtext"),
                    b("Text Mask", "small+label", 2, "textmask"),
                    b("Unmask Text", "small+label", 3, "textunmask"),
                ],
                launcher: None,
            },
            WantPanel {
                label: "Modify",
                // AutoCAD centres these two rows in the panel's height.
                items: &[b("Move/Copy/Rotate", "small+label", 1, "mocoro"), b("Stretch Multiple", "small+label", 2, "mstretch")],
                slideout: &[
                    b("Flatten objects", "small+label", 1, "flatten"),
                    b("Extended Clip", "small+label", 2, "clipit"),
                    b("Extended Offset", "small+label", 3, "exoffset"),
                    b("Convert Shape to Block", "small+label", 4, "shp2blk"),
                ],
                launcher: None,
            },
            WantPanel {
                label: "Layout",
                items: &[
                    b("Align Space", "small+label", 1, "alignspace"),
                    b("Synchronize Viewports", "small+label", 2, "vpsync"),
                    b("Merge Layout", "small+label", 3, "layoutmerge"),
                ],
                slideout: &[b("List Viewport Scale", "small+label", 1, "vpscale")],
                launcher: None,
            },
            WantPanel {
                label: "Draw",
                items: &[b("Break-line Symbol", "large", 0, "breakline"), b("Super Hatch", "large", 0, "superhatch")],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Dimension",
                items: &[
                    sp(
                        "Annotation Attachment",
                        "large",
                        0,
                        &[
                            ("Attach Leader to Annotation", "qlattach"),
                            ("Global Attach Leader to Annotation", "qlattachset"),
                            ("Detach Leaders from Annotation", "qldetachset"),
                        ],
                        0,
                    ),
                    b("Reset Text", "small+label", 1, "dimreassoc"),
                    b("Import Style", "small+label", 2, "dimim"),
                    b("Export Style", "small+label", 3, "dimex"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Tools",
                items: &[
                    b("Command Aliases", "large", 0, "aliasedit"),
                    b("Attach Xdata", "small+label", 1, "xdata"),
                    b("List Xdata", "small+label", 2, "xdlist"),
                    b("System Variables", "small+label", 3, "sysvdlg"),
                ],
                slideout: &[
                    b("Extended Plan", "small+label", 1, "explan"),
                    b("Make Linetype", "small+label", 2, "mkltype"),
                    b("Make Shape", "small+label", 3, "mkshape"),
                ],
                launcher: None,
            },
            WantPanel {
                label: "Web",
                // A drop-down button (the whole button opens the list), as observed in AutoCAD.
                items: &[mn(
                    "URL Options",
                    "large",
                    0,
                    &[("Show URLs", "showurls"), ("Change URLs", "churls"), ("Find and Replace URLs", "repurls")],
                )],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn express_tools_is_as_wide_as_autocads() {
    check_widths(
        "Express Tools",
        &[
            ("Blocks", 232.0),
            ("Text", 234.0),
            ("Modify", 137.0),
            ("Layout", 154.0),
            ("Draw", 102.0),
            ("Dimension", 185.0),
            ("Tools", 186.0),
            ("Web", 65.0),
        ],
    );
}
