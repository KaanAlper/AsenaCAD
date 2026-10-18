//! Parametric, View and Manage.

use super::*;

// ---- Parametric ----

#[test]
fn parametric_matches_autocad() {
    check_tab(
        "Parametric",
        &[
            WantPanel {
                label: "Geometric",
                items: &[
                    b("Auto Constrain", "large", 0, "autoconstrain"),
                    b("Coincident", "small", 1, "gccoincident"),
                    b("Collinear", "small", 1, "gccollinear"),
                    b("Concentric", "small", 1, "gcconcentric"),
                    b("Fix", "small", 1, "gcfix"),
                    b("Parallel", "small", 2, "gcparallel"),
                    b("Perpendicular", "small", 2, "gcperpendicular"),
                    b("Horizontal", "small", 2, "gchorizontal"),
                    b("Vertical", "small", 2, "gcvertical"),
                    b("Tangent", "small", 3, "gctangent"),
                    b("Smooth", "small", 3, "gcsmooth"),
                    b("Symmetric", "small", 3, "gcsymmetric"),
                    b("Equal", "small", 3, "gcequal"),
                    BAR,
                    b("Show/Hide", "small+label", 1, "constraintbar"),
                    b("Show All", "small+label", 2, "constraintbar.showall"),
                    b("Hide All", "small+label", 3, "constraintbar.hideall"),
                ],
                slideout: &[],
                launcher: Some(("constraintsettings", "Constraint Settings, Geometric")),
            },
            WantPanel {
                label: "Dimensional",
                items: &[
                    sp("Linear", "large", 0, &[("Linear", "dclinear"), ("Horizontal", "dchorizontal"), ("Vertical", "dcvertical")], 0),
                    b("Aligned", "large", 0, "dcaligned"),
                    b("Radius", "small", 1, "dcradius"),
                    b("Diameter", "small", 1, "dcdiameter"),
                    b("Angular", "small", 2, "dcangular"),
                    b("Convert", "small", 2, "dcconvert"),
                    BAR,
                    b("Show/Hide", "small+label", 1, "dcdisplay"),
                    b("Show All", "small+label", 2, "dcdisplay.showall"),
                    b("Hide All", "small+label", 3, "dcdisplay.hideall"),
                ],
                slideout: &[
                    tg("Dynamic Constraint Mode", "small+label", 1, "cconstraintform.dynamic"),
                    tg("Annotational Constraint Mode", "small+label", 2, "cconstraintform.annotational"),
                ],
                launcher: Some(("constraintsettings", "Constraint Settings, Dimensional")),
            },
            WantPanel {
                label: "Manage",
                items: &[b("Delete Constraints", "large", 0, "delconstraint"), tg("Parameters Manager", "large", 0, "parameters")],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn parametric_is_as_wide_as_autocads() {
    check_widths("Parametric", &[("Geometric", 260.0), ("Dimensional", 256.0), ("Manage", 136.0)]);
}

// ---- View ----

#[test]
fn view_matches_autocad() {
    check_tab(
        "View",
        &[
            WantPanel {
                label: "Viewport Tools",
                items: &[
                    tg("UCS Icon", "large", 0, "ui.toggle.ucsicon"),
                    tg("View Cube", "large", 0, "ui.toggle.viewcube"),
                    tg("Navigation Bar", "large", 0, "navbar"),
                ],
                slideout: &[tg("Show Motion", "small+label", 1, "navsmotion")],
                launcher: None,
            },
            WantPanel {
                label: "Named Views",
                items: &[
                    cb("Named Views", "NamedView", 1, 200.0),
                    b("New View", "small+label", 2, "newview"),
                    b("View Manager", "small+label", 3, "view"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Model Viewports",
                items: &[
                    sp(
                        "Viewport Configuration",
                        "large",
                        0,
                        &[
                            ("Single", "vports.1"),
                            ("Two: Vertical", "vports.2v"),
                            ("Two: Horizontal", "vports.2h"),
                            ("Three: Right", "vports.3r"),
                            ("Three: Left", "vports.3l"),
                            ("Three: Above", "vports.3a"),
                            ("Three: Below", "vports.3b"),
                            ("Three: Vertical", "vports.3v"),
                            ("Three: Horizontal", "vports.3h"),
                            ("Four: Equal", "vports.4"),
                            ("Four: Right", "vports.4r"),
                            ("Four: Left", "vports.4l"),
                        ],
                        0,
                    ),
                    b("Named", "small+label", 1, "vports"),
                    b("Join", "small+label", 2, "vports.join"),
                    b("Restore", "small+label", 3, "vports.toggle"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel { label: "Review", items: &[b("DWG Compare", "large", 0, "compare")], slideout: &[], launcher: None },
            WantPanel { label: "History", items: &[tg("Activity Insights", "large", 0, "activityinsights")], slideout: &[], launcher: None },
            WantPanel {
                label: "Palettes",
                items: &[
                    tg("Tool Palettes", "large", 0, "toolpalettes"),
                    tg("Properties", "large", 0, "properties"),
                    tg("Blocks", "large", 0, "ui.dialog.blocks"),
                    tg("Count", "large", 0, "count"),
                    tg("Command Macros", "large", 0, "commandmacros"),
                    tg("Sheet Set Manager", "large", 0, "sheetset"),
                    tg("Command Line", "small", 1, "ui.toggle.cmdline"),
                    tg("Markup Set Manager", "small", 1, "markup"),
                    tg("Layer Properties", "small", 2, "layer"),
                    tg("Quick Calculator", "small", 2, "ui.panel.quickcalc"),
                    tg("Design Center", "small", 3, "adcenter"),
                    tg("External References", "small", 3, "xref"),
                ],
                slideout: &[
                    b("Text Window", "large", 0, "textscr"),
                    tg("Materials Browser", "small+label", 1, "matbrowseropen"),
                    tg("Materials Editor", "small+label", 2, "mateditoropen"),
                    tg("Visual Styles", "small+label", 3, "visualstyles"),
                    tg("Render Settings", "small", 1, "rpref"),
                    tg("Lights in Model", "small", 2, "lightlist"),
                    tg("Sun Properties", "small", 3, "sunproperties"),
                ],
                launcher: None,
            },
            WantPanel {
                label: "Interface",
                items: &[
                    // A drop-down in AutoCAD's screenshot (▾ right of the label), not a plain
                    // button: it lists the open drawings.
                    mn("Switch Windows", "large", 0, &[("Switch Windows", "switchwindows")]),
                    tg("File Tabs", "large", 0, "ui.toggle.filetabs"),
                    tg("Layout Tabs", "large", 0, "ui.toggle.layouttabs"),
                    b("Tile Horizontally", "small+label", 1, "syswindows.tilehorz"),
                    b("Tile Vertically", "small+label", 2, "syswindows.tilevert"),
                    b("Cascade", "small+label", 3, "syswindows.cascade"),
                ],
                slideout: &[],
                launcher: Some(("options", "Options, Display Tab")),
            },
        ],
    );
}

#[test]
fn view_is_as_wide_as_autocads() {
    check_widths(
        "View",
        &[
            ("Viewport Tools", 145.0),
            ("Named Views", 206.0),
            ("Model Viewports", 175.0),
            ("Review", 60.0),
            ("History", 52.0),
            ("Palettes", 356.0),
            ("Interface", 268.0),
        ],
    );
}

// ---- Manage ----

#[test]
fn manage_matches_autocad() {
    check_tab(
        "Manage",
        &[
            WantPanel {
                label: "Action Recorder",
                items: &[
                    b("Record", "large", 0, "actrecord"),
                    b("Insert Message", "small", 1, "actusermessage"),
                    b("Insert Base Point", "small", 2, "actbasepoint"),
                    b("Pause for User Input", "small", 3, "actuserinput"),
                    BAR,
                    b("Play", "small+label", 1, "actplay"),
                    b("Preference", "small", 1, "actpreferences"),
                    b("Manage Action Macros", "small", 1, "actmanager"),
                    cb("Available Action Macro", "ActionMacro", 2, 125.0),
                ],
                // AutoCAD's slide-out is the Action Tree, a 250 px tree view of the macro's steps:
                // here a disabled box of that width until the Action Recorder exists.
                slideout: &[cb("Action Tree", "ActionTree", 1, 250.0)],
                launcher: None,
            },
            WantPanel {
                label: "Customization",
                items: &[
                    b("User Interface", "large", 0, "cui"),
                    b("Tool Palettes", "large", 0, "customize"),
                    b("Import", "small+label", 1, "cuiimport"),
                    b("Export", "small+label", 2, "cuiexport"),
                    // A split in AutoCAD's screenshot (▾ right of the label), not a plain button.
                    sp("Edit Aliases", "small+label", 3, &[("Edit Aliases", "editaliases")], 0),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Applications",
                items: &[
                    b("Load Application", "large", 0, "appload"),
                    b("Run Script", "large", 0, "script"),
                    b("Visual Basic Editor", "small+label", 1, "vbaide"),
                    b("Visual LISP Editor", "small+label", 2, "vlisp"),
                    b("Run VBA Macro", "small+label", 3, "vbarun"),
                ],
                slideout: &[b("Load Project", "small+label", 1, "vbaload"), b("VBA Manager", "small+label", 2, "vbaman")],
                launcher: None,
            },
            WantPanel {
                label: "CAD Standards",
                items: &[
                    b("Layer Translator", "small+label", 1, "laytrans"),
                    b("Check", "small+label", 2, "checkstandards"),
                    b("Configure", "small+label", 3, "standards"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Cleanup",
                items: &[
                    b("Find Non-Purgeable Items", "large", 0, "purge.nonpurgeable"),
                    b("Purge", "small+label", 1, "purge"),
                    b("Overkill", "small+label", 2, "overkill"),
                    b("Audit", "small+label", 3, "audit"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel { label: "Performance", items: &[b("Performance Analyzer", "large", 0, "performanceanalyzer")], slideout: &[], launcher: None },
        ],
    );
}

#[test]
fn manage_is_as_wide_as_autocads() {
    check_widths(
        "Manage",
        &[
            ("Action Recorder", 210.0),
            ("Customization", 205.0),
            ("Applications", 236.0),
            ("CAD Standards", 117.0),
            ("Cleanup", 195.0),
            ("Performance", 79.0),
        ],
    );
}
