//! Home (through the harness), Insert and Annotate.

use super::*;

/// The block gallery's stand-in list (both Home and Insert).
const INSERT_GALLERY: &[(&str, &str)] = &[("More Options...", "insert")];

#[test]
fn the_harness_accepts_home_panels() {
    // Data M1a verified, through the same harness.
    let home = tab("Home");
    let panel = |l: &str| home.panels.iter().find(|p| p.label == l).unwrap_or_else(|| panic!("{l}"));
    let paste = &[
        ("Paste", "pasteclip"),
        ("Paste as Block", "pasteblock"),
        ("Paste as Hyperlink", "pastehlnk"),
        ("Paste to Original Coordinates", "pasteorig"),
        ("Paste Special", "pastespec"),
    ];
    check_items(
        "Home ▸ Clipboard",
        panel("Clipboard").items,
        &[sp("Paste", "large", 0, paste, 0), b("Cut", "small", 1, "cutclip"), b("Copy Clip", "small", 2, "copyclip")],
    );
    let props = panel("Properties");
    assert_eq!(launcher(props), Some(("properties", "Properties")));
    check_items(
        "Home ▸ Properties (slide-out)",
        props.slideout,
        &[
            cb("Plot Style", "PlotStyle", 1, 200.0),
            cb("Transparency Mode", "Transparency", 2, 35.0),
            sl("Transparency", "Transparency", 2, 158.0),
            b("List", "small+label", 3, "list"),
        ],
    );
    check_items(
        "Home ▸ Groups (slide-out)",
        panel("Groups").slideout,
        &[b("Group Manager", "small+label", 1, "classicgroup"), tg("Group Bounding Box", "small+label", 2, "groupdisplaymode")],
    );
    assert_eq!(launcher(panel("View")), Some(("viewstd", "Drafting Standard")));
    let block = panel("Block");
    let insert = block.items.first().map(describe);
    assert_eq!(insert, Some(expected(&mn("Insert", "large", 0, INSERT_GALLERY))));
}

#[test]
fn home_is_as_wide_as_autocads() {
    // The panel widths of AutoCAD's Home tab at 1906 px. M1a tuned the whole tab; its three
    // narrowest panels (Groups, Utilities, Clipboard) are 8–14 px off on their own.
    check_total_width(
        "Home",
        &[
            ("Draw", 224.0),
            ("Modify", 250.0),
            ("Annotation", 189.0),
            ("Layers", 273.0),
            ("Block", 240.0),
            ("Properties", 265.0),
            ("Groups", 69.0),
            ("Utilities", 97.0),
            ("Clipboard", 91.0),
            ("View", 64.0),
        ],
    );
}

// ---- Insert ----

#[test]
fn insert_matches_autocad() {
    check_tab(
        "Insert",
        &[
            WantPanel {
                label: "Block",
                items: &[
                    // AutoCAD's Insert is a gallery of the drawing's blocks (a drop-down button).
                    mn("Insert", "large", 0, INSERT_GALLERY),
                    sp("Detect and Convert", "large", 0, &[("Detect and Convert", "bdetect"), ("Search and Convert", "bconvert")], 0),
                    sp("Edit Attribute", "small+label", 1, &[("Single", "attedit"), ("Multiple", "-attedit")], 0),
                    cb("Retain Attribute Display", "AttributeDisplay", 2, 165.0),
                    b("Replace", "small+label", 3, "breplace"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Block Definition",
                items: &[
                    sp("Create Block", "large", 0, &[("Create Block", "block"), ("Write Block", "wblock")], 0),
                    b("Define Attributes", "large", 0, "attdef"),
                    b("Manage Attributes", "large", 0, "battman"),
                    b("Block Editor", "large", 0, "bedit"),
                ],
                slideout: &[b("Set Base Point", "small+label", 1, "base"), b("Synchronize", "small+label", 2, "attsync")],
                launcher: None,
            },
            WantPanel {
                label: "Reference",
                items: &[
                    b("Attach", "large", 0, "attach"),
                    b("Clip", "large", 0, "clip"),
                    b("Adjust", "large", 0, "adjust"),
                    b("Underlay Layers", "small+label", 1, "ulayers"),
                    cb("*Frames vary*", "XrefFrames", 2, 114.0),
                    cb("Snap to Underlays ON", "UnderlaySnap", 3, 157.0),
                ],
                slideout: &[
                    b("Edit Reference", "small+label", 1, "refedit"),
                    tg("Xref Fading Toggle Button", "small", 2, "xdwgfadectl"),
                    sl("Xref fading", "XrefFade", 2, 200.0),
                ],
                launcher: Some(("xref", "External References")),
            },
            WantPanel {
                label: "Import",
                items: &[
                    sp("PDF Import", "large", 0, &[("PDF Import", "pdfimport"), ("DGN Import", "dgnimport"), ("Import", "import")], 0),
                    BAR,
                    b("Recognize SHX Text", "small+label", 1, "pdfshxtext"),
                    b("Recognition Settings", "small+label", 2, "pdfshxtext.settings"),
                    b("Combine Text", "large", 0, "txt2mtxt"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Data",
                items: &[
                    b("Field", "large", 0, "field"),
                    b("Update Fields", "small+label", 1, "updatefield"),
                    b("OLE Object", "small+label", 2, "insertobj"),
                    b("Hyperlink", "small+label", 3, "hyperlink"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Linking & Extraction",
                items: &[
                    b("Data Link", "large", 0, "datalink"),
                    b("Download from Source", "small+label", 1, "datalinkupdate"),
                    b("Upload to Source", "small+label", 2, "datalinkupdate.write"),
                    // AutoCAD's label has two spaces.
                    b("Extract  Data", "small+label", 3, "dataextraction"),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Location",
                // A drop-down button (inventory §6, observed after the first pass).
                items: &[mn(
                    "Set Location",
                    "large",
                    0,
                    &[
                        ("From Esri Maps", "geographiclocation.esri"),
                        ("From File", "geographiclocation.file"),
                        SEP,
                        ("From Bing Maps", "geographiclocation.bing"),
                    ],
                )],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn insert_is_as_wide_as_autocads() {
    check_widths(
        "Insert",
        &[
            ("Block", 288.0),
            ("Block Definition", 215.0),
            ("Reference", 282.0),
            ("Import", 270.0),
            ("Data", 145.0),
            ("Linking & Extraction", 196.0),
            ("Location", 69.0),
        ],
    );
}

// ---- Annotate ----

const DIM_LINEAR: &[(&str, &str)] = &[
    ("Linear", "dimlinear"),
    ("Aligned", "dimaligned"),
    ("Angular", "dimangular"),
    SEP,
    ("Arc Length", "dimarc"),
    ("Radius", "dimradius"),
    ("Diameter", "dimdiameter"),
    SEP,
    ("Jogged", "dimjogged"),
    ("Ordinate", "dimordinate"),
];

#[test]
fn annotate_matches_autocad() {
    check_tab(
        "Annotate",
        &[
            WantPanel {
                label: "Text",
                items: &[
                    sp("Multiline Text", "large", 0, &[("Multiline Text", "mtext"), ("Single Line", "text")], 0),
                    b("Check Spelling", "small", 1, "spell"),
                    b("Text Align", "small", 2, "textalign"),
                    b("Justify", "small", 3, "justifytext"),
                    cb("Text Style", "TextStyle", 1, 200.0),
                    cb("Find Text", "FindText", 2, 200.0),
                    cb("Annotation Text Height", "TextHeight", 3, 200.0),
                ],
                slideout: &[b("Scale", "small+label", 1, "scaletext"), cb("Text Layer Gallery", "TextLayer", 2, 200.0)],
                launcher: Some(("style", "Text Style...")),
            },
            WantPanel {
                label: "Dimensions",
                items: &[
                    b("Dimension", "large", 0, "dim"),
                    cb("Dimension Style", "DimStyle", 1, 200.0),
                    b("Break", "small", 1, "dimbreak"),
                    b("Adjust Space", "small", 1, "dimspace"),
                    b("Jog Line", "small", 1, "dimjogline"),
                    cb("Dim Layer Gallery", "DimLayer", 2, 200.0),
                    b("Inspect", "small", 2, "diminspect"),
                    b("Update", "small", 2, "dimstyle.update"),
                    b("Reassociate", "small", 2, "dimreassociate"),
                    sp("Linear", "small+label", 3, DIM_LINEAR, 0),
                    b("Quick", "small+label", 3, "qdim"),
                    sp("Continue", "small+label", 3, &[("Continue", "dimcontinue"), ("Baseline", "dimbaseline")], 0),
                ],
                slideout: &[
                    b("Tolerance", "small", 1, "tolerance"),
                    b("Oblique", "small", 1, "dimedit.oblique"),
                    b("Text Angle", "small", 1, "dimtedit.angle"),
                    b("Left Justify", "small", 1, "dimtedit.left"),
                    b("Center Justify", "small", 1, "dimtedit.center"),
                    b("Right Justify", "small", 1, "dimtedit.right"),
                    b("Override", "small", 1, "dimoverride"),
                ],
                launcher: Some(("dimstyle", "Dimension, Dimension Style...")),
            },
            WantPanel {
                label: "Centerlines",
                items: &[b("Center Mark", "large", 0, "centermark"), b("Centerline", "large", 0, "centerline")],
                slideout: &[cb("Center Layer Gallery", "CenterLayer", 1, 200.0)],
                launcher: None,
            },
            WantPanel {
                label: "Leaders",
                items: &[
                    b("Multileader", "large", 0, "mleader"),
                    cb("Multileader Style", "MLeaderStyle", 1, 200.0),
                    cb("Mleader Layer Gallery", "MLeaderLayer", 2, 200.0),
                    b("Align", "small", 3, "mleaderalign"),
                    b("Collect", "small", 3, "mleadercollect"),
                    b("Add Leader", "small", 3, "mleaderedit.add"),
                    b("Remove Leader", "small", 3, "mleaderedit.remove"),
                ],
                slideout: &[],
                launcher: Some(("mleaderstyle", "Multileader Style Manager")),
            },
            WantPanel {
                label: "Tables",
                items: &[
                    b("Table", "large", 0, "table"),
                    cb("Table Style", "TableStyle", 1, 200.0),
                    b("Download from Source", "small", 2, "datalinkupdate"),
                    b("Extract Data", "small+label", 2, "dataextraction"),
                    b("Upload to Source", "small", 3, "datalinkupdate.write"),
                    b("Link Data", "small+label", 3, "datalink"),
                ],
                slideout: &[],
                launcher: Some(("tablestyle", "Table Style...")),
            },
            WantPanel {
                label: "Markup",
                items: &[
                    b("Wipeout", "large", 0, "wipeout"),
                    sp(
                        "Revision Cloud",
                        "large",
                        0,
                        &[("Rectangular", "revcloud"), ("Polygonal", "revcloud.polygonal"), ("Freehand", "revcloud.freehand")],
                        0,
                    ),
                ],
                slideout: &[],
                launcher: None,
            },
            WantPanel {
                label: "Annotation Scaling",
                items: &[
                    sp(
                        "Add Current Scale",
                        "large",
                        0,
                        &[("Add Current Scale", "aiobjectscaleadd"), ("Delete Current Scale", "aiobjectscaleremove")],
                        0,
                    ),
                    b("Scale List", "small+label", 1, "scalelistedit"),
                    b("Add/Delete Scales", "small+label", 2, "objectscale"),
                    b("Sync Scale Positions", "small+label", 3, "annoreset"),
                ],
                slideout: &[],
                launcher: None,
            },
        ],
    );
}

#[test]
fn annotate_is_as_wide_as_autocads() {
    check_widths(
        "Annotate",
        &[
            ("Text", 300.0),
            ("Dimensions", 341.0),
            ("Centerlines", 106.0),
            ("Leaders", 272.0),
            ("Tables", 244.0),
            ("Markup", 122.0),
            ("Annotation Scaling", 234.0),
        ],
    );
}
