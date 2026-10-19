# UI parity

> **Last reviewed:** 2026-10-18 · **Last updated:** 2026-10-18 · **Change:** major (Windows AutoCAD 2026 baseline for the shell and the Home tab) · **Target:** Autodesk AutoCAD 2027 for Mac (26.0); the Windows baseline section: Autodesk AutoCAD 2026 (Windows)

How AsenaCAD's interaction compares with AutoCAD's: point entry, object snaps, tracking, dynamic
input, grips, selection, the command line, chrome and navigation. A drafter's speed in AutoCAD
comes from these, so "the command exists" isn't enough: it has to accept the same input, snap to
the same points and respond to the same keys. Work items: [gaps.md](gaps.md#uiux).

**Headline (estimated): UI/UX fidelity ≈ 48%.** The command line, point syntax, the common
running snaps, polar/ortho, object snap tracking and the palettes are real, and so are the
Extension, Parallel and Apparent Intersection snaps (#379); dynamic input is a read-only
tooltip; FROM, M2P, TK and point filters don't exist; hot-grip options are advertised but
ignored; there are no context menus. Remaining: **≈ 110–170 Opus 5.5 hours**.

## Windows AutoCAD 2026 baseline

> **Last reviewed:** 2026-10-19 · **Last updated:** 2026-10-19 · **Change:** minor (the Insert, Annotate, Parametric, View, Manage, Output, Collaborate and Express Tools tabs) · **Target:** Autodesk AutoCAD 2026 (Windows)

The rest of this doc still measures against AutoCAD 2027 for Mac; this section is the first part
re-measured against the Windows target (milestone M1a: title row, ribbon, document tabs and the
shell around the drawing; M1b: the other ribbon tabs).

**How this was measured.** AutoCAD 2026 for Windows was observed on the reference PC on
2026-10-18: an empty Drawing1 in the default workspace with the Home tab open, in a 1906 × 1026
window and in a 1280 px wide one, plus a UI Automation read of every Home panel, slide-out and
split-button list (local notes, never committed). AsenaCAD (branch `m1a-ribbon`) was rendered by
the headless test harness at the same window sizes with the sample drawing. Slide-outs and
drop-down lists were compared against the observed item lists (structure, order, labels, control
types), not against screenshots. "Matches" means what the [parity method](../.claude/rules/parity-method.md)
says: position and order, label text, control type and when an item is enabled. Pixels, icons
and colours are ours by design.
The other tabs (M1b) were compared the same way, against a UI Automation read of each tab's
panels and a screenshot of each tab at 1906 px (layout and labels only).

**Measured: 69 rows, 16 ✅, 52 ◐, 1 ✗.**

| Element | Status | AutoCAD 2026 (Windows) | AsenaCAD and the differences |
|---|---|---|---|
| Title row: Application button | ◐ | Leftmost in the window's own caption row, with a ▾; opens the Application menu | Same place and ▾, but it opens the File menu's entries until the Application menu lands (row below). On Windows and Linux the title row sits under the system's title bar, so the window buttons are the system's, not on the title row's right end; on macOS it starts after the traffic lights |
| Title row: Quick Access Toolbar | ✅ | New, Open, Save, Save As, two Web & Mobile buttons, Plot, Undo and Redo split buttons, Customize ▾, Share; the Customize list has 17 commands, More Commands, Show Menu Bar, Show Below the Ribbon | Same buttons in the same order without the Autodesk online ones ([below](#intentionally-absent-autodesk-online-services)); same Customize list and options |
| Title row: title | ✅ | Product and drawing name, centred | "AsenaCAD  <drawing>.dwg", centred; it slides or is cut short to stay between the toolbar and the InfoCenter |
| Title row: InfoCenter | ✅ | Hide-search ▸, search box, Search, Sign In ▾, App Store, Stay Connected ▾, Help split | Hide-search ▸, search box with the same placeholder, Search, Help split; the online items are left out on purpose. Search starts a command whose name or alias is typed, else opens our Command Reference (AutoCAD searches its online help) |
| Ribbon tab row | ◐ | Home, Insert, Annotate, Parametric, View, Manage, Output, Collaborate, Express Tools; then the minimize button (cycles Full, Panel Buttons, Panel Titles, Tabs) and its ▾ menu | Same nine tabs in order and the same minimize button and menu. Every tab has its panels |
| Home ▸ Draw | ✅ | Line, Polyline, Circle ▾, Arc ▾ (large); Rectangle ▾, Ellipse ▾ (its face reads "Center"), Hatch ▾ (small, icons only); 13-item slide-out ending with Revision Cloud ▾ | Same items, sizes, rows, list entries and separators; every item runs its command |
| Home ▸ Modify | ◐ | Move/Copy/Stretch, Rotate/Mirror/Scale, Trim ▾/Fillet ▾/Array ▾ with labels, Erase/Explode/Offset as icons; 15-item slide-out ending with the draw-order split | Same layout and lists; Set to ByLayer has no command yet, so it is disabled |
| Home ▸ Annotation | ◐ | Text ▾ and Dimension (large); Linear ▾, Leader ▾, Table (small, labelled); slide-out of four style buttons, each with its style list | Same items, lists and four-row slide-out; the style lists set the current style. Dimension starts our DIM, which makes only linear dimensions (AutoCAD's DIM picks the kind from the object under the cursor) |
| Home ▸ Layers | ◐ | Layer Properties (large toggle), the layer list, two rows of layer tools ending with Make Current and Match Layer; slide-out with Layer State, eight tools and the locked-layer fading toggle and slider | Same layout. The layer list shows only the layer's name (AutoCAD's also shows its on, freeze, lock and colour states); Copy Objects to New Layer, Layer Walk, VP Freeze in All Viewports except Current and Locked Layer fading have no command yet; the fading slider shows its value but is disabled |
| Home ▸ Block | ◐ | Insert ▾ (large drop-down, a gallery of the drawing's blocks), Detect and Convert ▾ (large), Create, Edit, Edit Attributes ▾; slide-out of six | Same layout and lists. Insert is a drop-down whose list has only More Options..., which starts INSERT (no gallery of blocks yet); Detect and Convert, Search and Convert and Replace have no command yet |
| Home ▸ Properties | ◐ | Match Properties; Object Color, Lineweight and Linetype lists, each with a small icon on its left and a colour swatch or line preview in the box; slide-out with Plot Style, transparency mode and slider, List; dialog launcher | Same layout, lists and launcher. The lists have no row icons, swatches or line previews; the transparency mode and slider are disabled (new objects' transparency is not modelled yet). Plot Style is disabled with ByColor in a colour-dependent drawing, as in AutoCAD |
| Home ▸ Groups | ◐ | Group (large), Ungroup, Group Edit, Group Selection toggle; slide-out with Group Manager and the Group Bounding Box toggle | Same layout; Group Manager and Group Bounding Box have no command yet |
| Home ▸ Utilities | ◐ | Measure ▾ (Quick, Distance, Radius, Angle, Area, Volume), Quick Select, Select All, Quick Calculator toggle; slide-out with ID Point and Point Style | Same layout and list. The Measure face runs Quick, which has no command yet, so the face is disabled (its ▾ list works); Volume has no command yet |
| Home ▸ Clipboard | ◐ | Paste ▾ (five entries), Cut and Copy Clip as icons, each drawn with a small ▾ mark | Same layout and Paste list; Paste as Hyperlink and Paste Special have no command yet. Cut and Copy Clip are plain icon buttons: what AutoCAD's ▾ marks open wasn't captured. Paste is disabled while there is nothing to paste |
| Home ▸ View | ◐ | Base (large); slide-out with Section and Detail View Style buttons and lists; Drafting Standard launcher | Same layout, but model documentation isn't built: Base, both styles and the launcher are disabled |
| Insert ▸ Block | ◐ | Insert ▾ (large drop-down, a gallery of the drawing's blocks), Detect and Convert ▾ (large), Edit Attribute ▾, the attribute display list, Replace | Same layout and lists. Insert is a drop-down whose list has only More Options..., which starts INSERT (no gallery of blocks yet); Detect and Convert, Search and Convert and Replace have no command yet. The attribute display drop-down is a plain list box (AutoCAD's has an icon in front) |
| Insert ▸ Block Definition | ✅ | Create Block ▾ (Create Block, Write Block), Define Attributes, Manage Attributes, Block Editor (large); slide-out with Set Base Point and Synchronize | Same items, sizes, list and slide-out; every item runs its command |
| Insert ▸ Reference | ◐ | Attach, Clip, Adjust (large); Underlay Layers, the frame display list, the underlay snap list; slide-out with Edit Reference and the xref fading toggle and slider; External References launcher | Same layout and launcher (it opens the External References palette). Attach is a placeholder; Clip, Adjust, Underlay Layers and the fading toggle have no command yet. The frame and underlay snap lists show AutoCAD's defaults but are disabled (frame display and underlay snapping are not modelled); the fading slider shows its value but is disabled. Both lists are plain list boxes (AutoCAD's have an icon in front) |
| Insert ▸ Import | ◐ | PDF Import ▾ (PDF Import, DGN Import, Import), Recognize SHX Text, Recognition Settings, Combine Text (large) | Same layout and list. Only Import has a command: the face (PDF Import), DGN Import, both SHX text tools and Combine Text are disabled |
| Insert ▸ Data | ◐ | Field (large), Update Fields, OLE Object, Hyperlink | Same layout; none of the four has a command yet, so all are disabled |
| Insert ▸ Linking & Extraction | ◐ | Data Link (large), Download from Source, Upload to Source, Extract Data | Same layout; Upload to Source has no command yet |
| Insert ▸ Location | ◐ | Set Location ▾ (large drop-down button: the whole button opens From Esri Maps, From File, —, From Bing Maps) | Same drop-down button and list. None of the entries has a command yet (Esri and Bing are online map services), so it is disabled |
| Annotate ▸ Text | ◐ | Multiline Text ▾ (Multiline Text, Single Line) (large); Check Spelling, Text Align, Justify (icons); the text style list, the Find Text box and the Annotation Text Height box; slide-out with Scale and the text layer list; Text Style launcher | Same layout and launcher. The style list sets the current style; Enter in Find Text finds the text and selects the objects it is in; Annotation Text Height is an editable list: Enter or an entry sets TEXTSIZE; its entries are the drawing's text heights and a few common sizes (our own choice, not AutoCAD's list). Check Spelling, Text Align, Justify and Scale have no command yet; the text layer list shows "Use Current" but is disabled (a layer for new text isn't modelled yet) |
| Annotate ▸ Dimensions | ◐ | Dimension (large); the dimension style list with Break, Adjust Space and Jog Line; the dimension layer list with Inspect, Update and Reassociate; Linear ▾, Quick, Continue ▾; slide-out of seven text and edit tools; Dimension Style launcher | Same layout, lists and launcher; every button runs its command. Dimension starts our DIM, which makes only linear dimensions (AutoCAD's DIM picks the kind from the object). The dimension layer list shows "Use Current" but is disabled |
| Annotate ▸ Centerlines | ◐ | Center Mark, Centerline (large); slide-out with the centerline layer list | Same items; the layer list shows "Use Current" but is disabled |
| Annotate ▸ Leaders | ◐ | Multileader (large); the multileader style and layer lists; Align, Collect, Add Leader, Remove Leader (icons); Multileader Style Manager launcher | Same layout and launcher; the style list sets the current style; the layer list shows "Use Current" but is disabled |
| Annotate ▸ Tables | ◐ | Table (large); the table style list; Download from Source, Extract Data, Upload to Source, Link Data; Table Style launcher | Same layout and launcher; Upload to Source has no command yet |
| Annotate ▸ Markup | ✅ | Wipeout, Revision Cloud ▾ (Rectangular, Polygonal, Freehand) (large) | Same items and list; both run their commands |
| Annotate ▸ Annotation Scaling | ✅ | Add Current Scale ▾ (Add Current Scale, Delete Current Scale) (large); Scale List, Add/Delete Scales, Sync Scale Positions | Same items and list; every item runs its command |
| Parametric ▸ Geometric | ✅ | Auto Constrain (large); the twelve geometric constraints as icons in three rows of four; Show/Hide, Show All, Hide All; Constraint Settings launcher | Same layout and launcher; every item runs its command. The launcher opens our Constraint Settings dialog, which shows its geometric, dimensional and AutoConstrain settings on one page (AutoCAD's dialog has a tab for each) |
| Parametric ▸ Dimensional | ◐ | Linear ▾ (Linear, Horizontal, Vertical), Aligned (large); Radius, Diameter, Angular, Convert (icons); Show/Hide, Show All, Hide All; slide-out with the Dynamic and Annotational Constraint Mode toggles; Constraint Settings launcher | Same layout, list, slide-out and launcher. Convert has no command yet. Both mode toggles are disabled: annotational constraints aren't modelled (CCONSTRAINTFORM), every constraint is dynamic, so Dynamic Constraint Mode shows pressed as in AutoCAD's default |
| Parametric ▸ Manage | ✅ | Delete Constraints (large), Parameters Manager (large toggle) | Same items; both run their commands. The toggle shows pressed while the Parameters Manager is open and closes it when clicked again (PARAMETERSCLOSE) |
| View ▸ Viewport Tools | ◐ | UCS Icon, View Cube, Navigation Bar (large toggles); slide-out with the Show Motion toggle | Same layout. The three toggles show and hide the UCS icon, the ViewCube and the navigation bar and show pressed while they are on; Show Motion is disabled (no ShowMotion) |
| View ▸ Named Views | ◐ | The named views list ("Unsaved View"); New View, View Manager | Same layout. The list shows "Unsaved View" and lists the views of the current space; choosing one restores it (VIEW Restore); with no views it holds only "Unsaved View". New View has no command yet. View Manager runs VIEW, which works like -VIEW here: its options are asked on the command line (AutoCAD opens the View Manager dialog) |
| View ▸ Model Viewports | ◐ | Viewport Configuration ▾ (large; Single, two, three and four-viewport arrangements); Named, Join, Restore | Same layout and list. Single, the two and three-viewport arrangements and Four: Equal tile the model space (in a layout they make layout viewports), and Join joins two tiles; Four: Right, Four: Left and Restore (-VPORTS Toggle) have no command yet. Named runs VPORTS, which asks on the command line (AutoCAD opens the Viewports dialog's Named Viewports tab) |
| View ▸ Review | ✅ | DWG Compare (large) | Same button; it runs COMPARE |
| View ▸ History | ◐ | Activity Insights (large toggle) | Same toggle, disabled: there is no Activity Insights palette |
| View ▸ Palettes | ◐ | Tool Palettes, Properties, Blocks, Count, Command Macros, Sheet Set Manager (large toggles); Command Line, Markup Set Manager, Layer Properties, Quick Calculator, Design Center, External References (icon toggles); slide-out with Text Window and six rendering palette toggles | Same layout and slide-out. Properties, Blocks, Command Line, Layer Properties, Quick Calculator and External References open their palettes and show pressed while open; Text Window opens the command history (TEXTSCR); Count starts COUNT (pick an object, its kind is counted; there is no Count palette, so it never shows pressed); Blocks opens a plain list of the drawing's blocks. Tool Palettes, Command Macros, Sheet Set Manager, Markup Set Manager, Design Center and the six rendering palettes aren't built. Clicking a pressed palette toggle closes its palette (PROPERTIESCLOSE, LAYERCLOSE, QCCLOSE, EXTERNALREFERENCESCLOSE, BLOCKSPALETTECLOSE), as in AutoCAD |
| View ▸ Interface | ◐ | Switch Windows ▾ (large drop-down listing the open drawings), File Tabs, Layout Tabs (large toggles); Tile Horizontally, Tile Vertically, Cascade; Options launcher (Display tab) | Same layout and launcher. File Tabs and Layout Tabs show and hide the drawing tabs and the layout tabs. Switch Windows is a drop-down without the list of drawings, so it is disabled; the three window arrangements are disabled (AsenaCAD shows one drawing at a time). The launcher opens Options on its Display tab |
| Manage ▸ Action Recorder | ◐ | Record (large); Insert Message, Insert Base Point, Pause for User Input (icons, disabled while not recording); Play, Preference, Manage Action Macros and the action macro list; a slide-out with the Action Tree | Same layout, the separator after the first three icons and the slide-out; everything is disabled (no Action Recorder yet). The slide-out's Action Tree (a tree of the macro's steps) is an empty, disabled box of its width |
| Manage ▸ Customization | ◐ | User Interface, Tool Palettes (large); Import, Export, Edit Aliases ▾ | Same layout; Edit Aliases is a split as in AutoCAD, but only its face entry is listed (the ▾ list wasn't captured). None of the five has a command yet |
| Manage ▸ Applications | ◐ | Load Application, Run Script (large); Visual Basic Editor, Visual LISP Editor, Run VBA Macro; slide-out with Load Project and VBA Manager | Same layout and slide-out. Load Application (APPLOAD, LISP files) and Run Script (SCRIPT) run; the VBA and Visual LISP tools have no command yet |
| Manage ▸ CAD Standards | ◐ | Layer Translator, Check, Configure | Same layout; none of the three has a command yet |
| Manage ▸ Cleanup | ◐ | Find Non-Purgeable Items (large); Purge, Overkill, Audit | Same layout; Purge, Overkill and Audit run their commands, Find Non-Purgeable Items has no command yet |
| Manage ▸ Performance | ◐ | Performance Analyzer (large) | Same button, disabled: no performance analyzer yet |
| Output ▸ Plot | ◐ | Plot, Batch Plot, Preview (large); Page Setup Manager, View Details, Plotter Manager; the Plot launcher | Same layout and launcher, but View Details is disabled. Plot and the launcher open the Plot dialog, Batch Plot opens Publish, Preview, Page Setup Manager and Plotter Manager run their commands; View Details (the last plot's report) has no command yet |
| Output ▸ Export to DWF/PDF | ◐ | Export ▾ (DWFx, DWF, PDF) (large); the export area list ("Export: Display") with Export Window; the page setup list ("Page Setup: Current") with Page Setup; Preview, DWF options and PDF options icons | Same layout and list. The PDF entry runs EXPORTPDF; DWFx and DWF export, the two lists and the five small tools have no command yet, so the lists show AutoCAD's defaults disabled |
| Collaborate ▸ Share | ◐ | Share Drawing (large), Shared Views (large toggle) | Same items, disabled: both use Autodesk's online services ([below](#intentionally-absent-autodesk-online-services)) |
| Collaborate ▸ Autodesk Docs | ◐ | Push to Autodesk Docs (large toggle) | Same toggle, disabled: it needs Autodesk Docs |
| Collaborate ▸ Traces | ◐ | Traces Palette (large toggle), Markup Import (large) | Same items, disabled: both need Autodesk's online services |
| Collaborate ▸ Compare | ✅ | DWG Compare (large) | Same button; it runs COMPARE |
| Express Tools ▸ Blocks | ◐ | Explode Attributes, Replace Block (large); List Properties, Import Attributes, Export Attributes; slide-out with Convert block to xref, Copy Nested Objects, Extend to Nested Objects, Trim to Nested Objects | Same layout and slide-out; Copy Nested Objects runs NCOPY, the other eight have no command yet |
| Express Tools ▸ Text | ◐ | Arc Aligned, Modify Text ▾ (Explode, Change Case, Rotate, Fit, Justify) (large); Convert to Mtext, Auto Number, Enclose in Object; slide-out with Remote Text, Text Mask, Unmask Text | Same layout, list and slide-out; none of the tools has a command yet, so Modify Text and its ▾ are disabled |
| Express Tools ▸ Modify | ◐ | Move/Copy/Rotate, Stretch Multiple, centred in the panel's height; slide-out with Flatten objects, Extended Clip, Extended Offset, Convert Shape to Block | Same layout (the two rows centred) and slide-out; Flatten objects runs FLATTEN, the other five have no command yet |
| Express Tools ▸ Layout | ◐ | Align Space, Synchronize Viewports, Merge Layout; slide-out with List Viewport Scale | Same layout and slide-out; none of the four has a command yet |
| Express Tools ▸ Draw | ◐ | Break-line Symbol, Super Hatch (large) | Same buttons, disabled: neither has a command yet |
| Express Tools ▸ Dimension | ◐ | Annotation Attachment ▾ (Attach Leader to Annotation, Global Attach Leader to Annotation, Detach Leaders from Annotation) (large); Reset Text, Import Style, Export Style | Same layout and list; none of the tools has a command yet |
| Express Tools ▸ Tools | ◐ | Command Aliases (large); Attach Xdata, List Xdata, System Variables; slide-out with Extended Plan, Make Linetype, Make Shape | Same layout and slide-out; Command Aliases runs ALIASEDIT, the other six have no command yet |
| Express Tools ▸ Web | ◐ | URL Options ▾ (large drop-down button: the whole button opens Show URLs, Change URLs, Find and Replace URLs) | Same drop-down button and list; none of the entries has a command yet, so it is disabled |
| Ribbon collapse | ✅ | At 1906 px every Home panel is full. At 1280 px Modify, Annotation, Layers and Block drop their small items' labels, and Properties, Groups, Utilities, Clipboard and View become panel buttons (icon, name on one line, ▾) that open the panel below them | Same panels shrink and collapse at both widths, at any display scale. Narrower windows collapse more panels from the right. A panel button never breaks a word of its name: it gets wider when a word doesn't fit (Annotation in a 900-point window) |
| Document tab bar | ✅ | File-tabs menu ≡, Start (no close button), drawing tabs with × and a * while unsaved, + | Same controls in the same order |
| Viewport controls | ✅ | [−][Top][2D Wireframe] menus at the top left; minimize, restore and close at the top right | Same controls and places |
| ViewCube | ◐ | Compass ring with N, E, S, W, the TOP face, WCS ▾ below; the Home button appears while the pointer is over the cube | Same parts and places, Home shown only on hover. Top and Home zoom to the extents; the cube can't be dragged or rotated (2D only, #7) |
| Navigation bar | ◐ | Under the ViewCube: Full Navigation Wheel, Pan, Zoom ▾, Orbit ▾, ShowMotion; a close button | Same tools in the same order (NAVBAR, ⊗ closes it). Pan and Zoom with its eleven variants work (the face shows the last one); the wheel, Orbit and its variants, ShowMotion and the bar's Customize are disabled (no SteeringWheels or 3D views) |
| Command line | ◐ | A floating bar over the drawing's bottom: grip, close, Customize, recent commands, input box ("Type a command"), history toggle; the history shows each prompt with the value typed after it | Same bar and controls (COMMANDLINEHIDE, COMMANDLINE or Ctrl+9 bring it back; history lines setting). The history shows "Command: LINE" but not the prompts with the values typed; the bar can't be dragged or docked and has no transparency |
| Layout bar | ✅ | Layouts menu ≡, Model, Layout1, Layout2, + at the status bar's left | Same controls in the same order; in a narrow window ≡, Model, the current layout and + always stay |
| Status bar | ◐ | 16 default items from MODEL to Clean Screen, then Customization ≡; some with ▾ menus (snap type, polar angles, isoplanes, object snaps, annotation scale, workspaces) | Same items, order and ▾ menus; in a narrow window the items that don't fit go from the left end. Workspace Switching, Annotation Monitor and Clean Screen are disabled until their commands exist |
| Status bar Customization | ◐ | A check list of 29 items with separators, 14 hidden by default | Same list, order, separators and defaults, saved with the workspace. 3D Object Snap, Dynamic UCS, Selection Filtering, Gizmo, Lock UI and Graphics Performance are disabled |
| Application menu | ✗ | Search box; New, Open, Save, Save As, Import, Export, Publish, Print, Drawing Utilities, Close with sub-panes; Recent and Open Documents; Options and Exit | Not built (M1c): the Application button lists the File menu's entries |

The ZOOM prompt now has AutoCAD's comma: "Specify corner of window, enter a scale factor (nX or
nXP), or [All/Center/…] <real time>:".

### Intentionally absent (Autodesk online services)

These need an Autodesk account or Autodesk's servers. AsenaCAD uses no Autodesk service, so they
are left out of the shell rather than shown disabled:

- **Share** (Quick Access Toolbar): sends a link to the drawing through Autodesk's cloud.
- **Sign In** (InfoCenter): signs in to an Autodesk account.
- **Autodesk App Store** (InfoCenter): Autodesk's add-on store.
- **Stay Connected** (InfoCenter): links to Autodesk's online channels and feedback.
- **Open from Web & Mobile** and **Save to Web & Mobile** (Quick Access Toolbar): drawings stored
  in Autodesk's online storage.

The Collaborate tab's tools are such services too. Unlike the items above they stay in the
ribbon, disabled, so the tab has AutoCAD's panels and buttons; only DWG Compare works:

- **Share Drawing**: hands out a link to the drawing kept in Autodesk's cloud.
- **Shared Views**: publishes views of the drawing for others to look at and comment on in a web
  browser.
- **Push to Autodesk Docs**: sends layouts as PDF sheets to an Autodesk Docs project.
- **Traces Palette**: overlays for reviewing a drawing, kept with Autodesk's online services.
- **Markup Import**: brings marks made on a printed or PDF sheet into the drawing through
  Autodesk's markup service.

### Ribbon items without a command

Ribbon items that name an AutoCAD command AsenaCAD doesn't have yet. They are shown disabled with
"Not available yet". This list is `ribbon::model::tests::UNBUILT`; a test fails when the two
differ.

| Command | Ribbon item |
|---|---|
| `setbylayer` | Modify ▸ Set to ByLayer |
| `copytolayer` | Layers ▸ Copy Objects to New Layer |
| `laywalk` | Layers ▸ Layer Walk |
| `layvpi` | Layers ▸ VP Freeze in All Viewports except Current |
| `laylockfadectl` | Layers ▸ Locked Layer fading (toggle) |
| `bdetect` | Block ▸ Detect and Convert (also Insert ▸ Block) |
| `bconvert` | Block ▸ Search and Convert (also Insert ▸ Block) |
| `breplace` | Block ▸ Replace (also Insert ▸ Block) |
| `classicgroup` | Groups ▸ Group Manager |
| `groupdisplaymode` | Groups ▸ Group Bounding Box (toggle) |
| `measuregeom.quick` | Utilities ▸ Measure ▸ Quick |
| `measuregeom.volume` | Utilities ▸ Measure ▸ Volume |
| `pastehlnk` | Clipboard ▸ Paste ▸ Paste as Hyperlink |
| `pastespec` | Clipboard ▸ Paste ▸ Paste Special |
| `viewbase` | View ▸ Base |
| `viewsectionstyle` | View ▸ Section View Style |
| `viewdetailstyle` | View ▸ Detail View Style |
| `viewstd` | View ▸ Drafting Standard (dialog launcher) |
| `clip` | Insert ▸ Reference ▸ Clip |
| `adjust` | Insert ▸ Reference ▸ Adjust |
| `ulayers` | Insert ▸ Reference ▸ Underlay Layers |
| `xdwgfadectl` | Insert ▸ Reference ▸ Xref Fading (toggle) |
| `pdfimport` | Insert ▸ Import ▸ PDF Import |
| `dgnimport` | Insert ▸ Import ▸ PDF Import ▸ DGN Import |
| `pdfshxtext` | Insert ▸ Import ▸ Recognize SHX Text |
| `pdfshxtext.settings` | Insert ▸ Import ▸ Recognition Settings |
| `txt2mtxt` | Insert ▸ Import ▸ Combine Text; Express Tools ▸ Text ▸ Convert to Mtext |
| `field` | Insert ▸ Data ▸ Field |
| `updatefield` | Insert ▸ Data ▸ Update Fields |
| `insertobj` | Insert ▸ Data ▸ OLE Object |
| `hyperlink` | Insert ▸ Data ▸ Hyperlink |
| `datalinkupdate.write` | Insert ▸ Linking & Extraction ▸ Upload to Source (also Annotate ▸ Tables) |
| `geographiclocation.esri` | Insert ▸ Location ▸ Set Location ▸ From Esri Maps (an online map service) |
| `geographiclocation.file` | Insert ▸ Location ▸ Set Location ▸ From File |
| `geographiclocation.bing` | Insert ▸ Location ▸ Set Location ▸ From Bing Maps (an online map service) |
| `spell` | Annotate ▸ Text ▸ Check Spelling |
| `textalign` | Annotate ▸ Text ▸ Text Align |
| `justifytext` | Annotate ▸ Text ▸ Justify; Express Tools ▸ Text ▸ Modify Text ▸ Justify |
| `scaletext` | Annotate ▸ Text ▸ Scale (slide-out) |
| `dcconvert` | Parametric ▸ Dimensional ▸ Convert |
| `cconstraintform.dynamic` | Parametric ▸ Dimensional ▸ Dynamic Constraint Mode (toggle, slide-out; sets CCONSTRAINTFORM to 0) |
| `cconstraintform.annotational` | Parametric ▸ Dimensional ▸ Annotational Constraint Mode (toggle, slide-out; sets CCONSTRAINTFORM to 1) |
| `navsmotion` | View ▸ Viewport Tools ▸ Show Motion (toggle, slide-out; also the navigation bar's ShowMotion) |
| `newview` | View ▸ Named Views ▸ New View |
| `vports.4r` | View ▸ Model Viewports ▸ Viewport Configuration ▸ Four: Right (an arrangement -VPORTS doesn't have yet) |
| `vports.4l` | View ▸ Model Viewports ▸ Viewport Configuration ▸ Four: Left (an arrangement -VPORTS doesn't have yet) |
| `vports.toggle` | View ▸ Model Viewports ▸ Restore (-VPORTS Toggle) |
| `activityinsights` | View ▸ History ▸ Activity Insights (toggle) |
| `toolpalettes` | View ▸ Palettes ▸ Tool Palettes (toggle) |
| `commandmacros` | View ▸ Palettes ▸ Command Macros (toggle) |
| `sheetset` | View ▸ Palettes ▸ Sheet Set Manager (toggle) |
| `markup` | View ▸ Palettes ▸ Markup Set Manager (toggle) |
| `adcenter` | View ▸ Palettes ▸ Design Center (toggle) |
| `matbrowseropen` | View ▸ Palettes ▸ Materials Browser (toggle, slide-out) |
| `mateditoropen` | View ▸ Palettes ▸ Materials Editor (toggle, slide-out) |
| `visualstyles` | View ▸ Palettes ▸ Visual Styles (toggle, slide-out) |
| `rpref` | View ▸ Palettes ▸ Render Settings (toggle, slide-out) |
| `lightlist` | View ▸ Palettes ▸ Lights in Model (toggle, slide-out) |
| `sunproperties` | View ▸ Palettes ▸ Sun Properties (toggle, slide-out) |
| `switchwindows` | View ▸ Interface ▸ Switch Windows (the list of open drawings) |
| `syswindows.tilehorz` | View ▸ Interface ▸ Tile Horizontally |
| `syswindows.tilevert` | View ▸ Interface ▸ Tile Vertically |
| `syswindows.cascade` | View ▸ Interface ▸ Cascade |
| `actrecord` | Manage ▸ Action Recorder ▸ Record |
| `actusermessage` | Manage ▸ Action Recorder ▸ Insert Message |
| `actbasepoint` | Manage ▸ Action Recorder ▸ Insert Base Point |
| `actuserinput` | Manage ▸ Action Recorder ▸ Pause for User Input |
| `actplay` | Manage ▸ Action Recorder ▸ Play (plays the macro chosen in the list) |
| `actpreferences` | Manage ▸ Action Recorder ▸ Preference (the Action Recorder's preferences) |
| `actmanager` | Manage ▸ Action Recorder ▸ Manage Action Macros |
| `cui` | Manage ▸ Customization ▸ User Interface |
| `customize` | Manage ▸ Customization ▸ Tool Palettes (customizes the tool palettes) |
| `cuiimport` | Manage ▸ Customization ▸ Import |
| `cuiexport` | Manage ▸ Customization ▸ Export |
| `editaliases` | Manage ▸ Customization ▸ Edit Aliases (edits the command alias file) |
| `vbaide` | Manage ▸ Applications ▸ Visual Basic Editor |
| `vlisp` | Manage ▸ Applications ▸ Visual LISP Editor |
| `vbarun` | Manage ▸ Applications ▸ Run VBA Macro |
| `vbaload` | Manage ▸ Applications ▸ Load Project (slide-out) |
| `vbaman` | Manage ▸ Applications ▸ VBA Manager (slide-out) |
| `laytrans` | Manage ▸ CAD Standards ▸ Layer Translator |
| `checkstandards` | Manage ▸ CAD Standards ▸ Check |
| `standards` | Manage ▸ CAD Standards ▸ Configure |
| `purge.nonpurgeable` | Manage ▸ Cleanup ▸ Find Non-Purgeable Items (PURGE on its non-purgeable items view) |
| `performanceanalyzer` | Manage ▸ Performance ▸ Performance Analyzer |
| `viewplotdetails` | Output ▸ Plot ▸ View Details (the report of the last plots) |
| `exportdwfx` | Output ▸ Export to DWF/PDF ▸ Export ▸ DWFx |
| `exportdwf` | Output ▸ Export to DWF/PDF ▸ Export ▸ DWF |
| `eplotexport.window` | Output ▸ Export to DWF/PDF ▸ Export Window (picks the area to export) |
| `eplotexport.pagesetup` | Output ▸ Export to DWF/PDF ▸ Page Setup (overrides the page setup for the export) |
| `eplotexport.preview` | Output ▸ Export to DWF/PDF ▸ Preview (of the export) |
| `exportsettings.dwf` | Output ▸ Export to DWF/PDF ▸ Export to DWF Options |
| `exportsettings.pdf` | Output ▸ Export to DWF/PDF ▸ Export to PDF Options |
| `sharedviews` | Collaborate ▸ Share ▸ Shared Views (Autodesk online service) |
| `pushtodocsopen` | Collaborate ▸ Autodesk Docs ▸ Push to Autodesk Docs (Autodesk online service) |
| `tracepaletteopen` | Collaborate ▸ Traces ▸ Traces Palette (Autodesk online service) |
| `markupimport` | Collaborate ▸ Traces ▸ Markup Import (Autodesk online service) |
| `burst` | Express Tools ▸ Blocks ▸ Explode Attributes (explodes blocks, keeping attribute values as text) |
| `blockreplace` | Express Tools ▸ Blocks ▸ Replace Block |
| `xlist` | Express Tools ▸ Blocks ▸ List Properties (of a nested object) |
| `attin` | Express Tools ▸ Blocks ▸ Import Attributes |
| `attout` | Express Tools ▸ Blocks ▸ Export Attributes |
| `blocktoxref` | Express Tools ▸ Blocks ▸ Convert block to xref (slide-out) |
| `bextend` | Express Tools ▸ Blocks ▸ Extend to Nested Objects (slide-out) |
| `btrim` | Express Tools ▸ Blocks ▸ Trim to Nested Objects (slide-out) |
| `arctext` | Express Tools ▸ Text ▸ Arc Aligned |
| `txtexp` | Express Tools ▸ Text ▸ Modify Text ▸ Explode |
| `tcase` | Express Tools ▸ Text ▸ Modify Text ▸ Change Case |
| `torient` | Express Tools ▸ Text ▸ Modify Text ▸ Rotate |
| `textfit` | Express Tools ▸ Text ▸ Modify Text ▸ Fit |
| `tcount` | Express Tools ▸ Text ▸ Auto Number |
| `tcircle` | Express Tools ▸ Text ▸ Enclose in Object |
| `rtext` | Express Tools ▸ Text ▸ Remote Text (slide-out) |
| `textmask` | Express Tools ▸ Text ▸ Text Mask (slide-out) |
| `textunmask` | Express Tools ▸ Text ▸ Unmask Text (slide-out) |
| `mocoro` | Express Tools ▸ Modify ▸ Move/Copy/Rotate |
| `mstretch` | Express Tools ▸ Modify ▸ Stretch Multiple |
| `clipit` | Express Tools ▸ Modify ▸ Extended Clip (slide-out) |
| `exoffset` | Express Tools ▸ Modify ▸ Extended Offset (slide-out) |
| `shp2blk` | Express Tools ▸ Modify ▸ Convert Shape to Block (slide-out) |
| `alignspace` | Express Tools ▸ Layout ▸ Align Space |
| `vpsync` | Express Tools ▸ Layout ▸ Synchronize Viewports |
| `layoutmerge` | Express Tools ▸ Layout ▸ Merge Layout |
| `vpscale` | Express Tools ▸ Layout ▸ List Viewport Scale (slide-out) |
| `breakline` | Express Tools ▸ Draw ▸ Break-line Symbol |
| `superhatch` | Express Tools ▸ Draw ▸ Super Hatch |
| `qlattach` | Express Tools ▸ Dimension ▸ Annotation Attachment ▸ Attach Leader to Annotation |
| `qlattachset` | Express Tools ▸ Dimension ▸ Annotation Attachment ▸ Global Attach Leader to Annotation |
| `qldetachset` | Express Tools ▸ Dimension ▸ Annotation Attachment ▸ Detach Leaders from Annotation |
| `dimreassoc` | Express Tools ▸ Dimension ▸ Reset Text (puts back the measured value of overridden dimension text) |
| `dimim` | Express Tools ▸ Dimension ▸ Import Style |
| `dimex` | Express Tools ▸ Dimension ▸ Export Style |
| `xdata` | Express Tools ▸ Tools ▸ Attach Xdata |
| `xdlist` | Express Tools ▸ Tools ▸ List Xdata |
| `sysvdlg` | Express Tools ▸ Tools ▸ System Variables (the system variable editor) |
| `explan` | Express Tools ▸ Tools ▸ Extended Plan (slide-out) |
| `mkltype` | Express Tools ▸ Tools ▸ Make Linetype (slide-out) |
| `mkshape` | Express Tools ▸ Tools ▸ Make Shape (slide-out) |
| `showurls` | Express Tools ▸ Web ▸ URL Options ▸ Show URLs |
| `churls` | Express Tools ▸ Web ▸ URL Options ▸ Change URLs |
| `repurls` | Express Tools ▸ Web ▸ URL Options ▸ Find and Replace URLs |

## How this was measured

- AsenaCAD: read from the source on 2026-10-10 (main at bd2412f): `crates/engine/src/{prompt,
  snap, select, grips, lib, units}.rs`, `cmd/*`, `crates/ui-egui/src/*`. Not run in this pass.
- AutoCAD: behaviour observed black-box in the running AutoCAD 2027 for Mac
  (`plan/autocad/01-observed-ui.md`, local only, with reference screenshots that are never
  committed), its menu tree (`xtask/data/menu-catalog.txt`) and Autodesk's documentation.
- Weights (how often a drafter hits each area): point entry 15%, object snaps 15%, tracking and
  polar 10%, dynamic input 10%, grips 10%, selection 10%, command line 10%, chrome and palettes
  10%, navigation 5%, keyboard and context menus 5%.

| Area | Weight | AsenaCAD | Weighted |
|---|---:|---:|---:|
| Point entry | 15% | 45% | 6.8 |
| Object snaps | 15% | 45% | 6.8 |
| Tracking, polar, ortho, grid/snap | 10% | 70% | 7.0 |
| Dynamic input | 10% | 45% | 4.5 |
| Grips | 10% | 40% | 4.0 |
| Selection | 10% | 40% | 4.0 |
| Command line | 10% | 60% | 6.0 |
| Chrome and palettes | 10% | 45% | 4.5 |
| Navigation | 5% | 55% | 2.8 |
| Keyboard shortcuts, context menus | 5% | 25% | 1.3 |
| **Total** | 100% | | **≈ 48%** |

## Point entry (≈ 45%)

| Input | AutoCAD | AsenaCAD | Evidence |
|---|---|---|---|
| `x,y[,z]`, `@dx,dy`, `@d<a`, `d<a`, `#x,y`, bare `@` | yes | yes; `#` equals absolute because there is no UCS; Z parsed then discarded | `prompt.rs:147-180` |
| `*x,y` (WCS override) | yes | yes (`*x,y`, `*@dx,dy`, `@*dx,dy`); the same as the plain forms because there is no UCS | `prompt.rs` |
| Direct distance entry | yes, honours ortho, polar and tracking paths | yes, ortho; polar only through the UI's snapped cursor | `lib.rs:782-793` |
| Angle forms: `45d30'`, radians, grads, surveyor `N45dE` | yes | yes | `units.rs:141-197` |
| Architectural/engineering distances `5'6-1/2"` | yes | yes | `units.rs:93-140` |
| `FROM` (base point + offset) | yes | yes: "Base point:", "<Offset>:" with `@` measured from the base; nests with M2P and filters | `pointmod.rs` |
| `M2P` / `MTP` (midpoint between two points) | yes | yes | `pointmod.rs` |
| `TK` (tracking) | yes | **no** | |
| Point filters `.x`, `.y`, `.xy`, `.z` | yes | yes (`.x` … `.yz`, "(need YZ):"); Z is asked for and dropped (no 3D) | `pointmod.rs` |
| Angle override `<45` | yes | yes: `<a`, `<<a`, `<<<a`; a typed distance or a pick follows the angle, typed coordinates win | `pointmod.rs` |
| Object snap overrides (`END`, `MID`, `INT` … `NON`) | yes | yes, every mode and `NON`, with long names and `_`/`'` prefixes, for the next pick only; a typed point is snapped within the aperture; a command's own keyword wins (`_cen` always snaps); also from the snap menu | `pointmod.rs` |
| Typed calculator `'CAL` | yes | no | |

## Object snaps (≈ 45%)

| Mode | AutoCAD | AsenaCAD (`snap.rs:135-334`) |
|---|---|---|
| Endpoint | every curve, solid/3D vertices, dimension ends | lines, arcs, polyline segments, open ellipses and splines, ray base, solid/trace corners |
| Midpoint | every curve incl. ellipse/spline arcs, polyline segments | segments only; **none on circles, ellipses, splines** |
| Center | arcs, circles, ellipses, polyarcs | arcs, circles, ellipses |
| Geometric center | any closed polyline, spline | closed LWPOLYLINE only |
| Node | points, dimension definition points, text alignment points | POINT only |
| Quadrant | circles, arcs, ellipses | yes |
| Intersection | every pair, extended intersection | among up to 200 nearby primitives; ellipse/spline pairs approximate ([geometry-parity.md](geometry-parity.md)) |
| Extension | yes, with dashed extension path | yes: rest on a line or arc end, then the line runs on (a ray) and the arc round its circle; dotted path and "Extension: d < a°" (`snap/tracking.rs`); not from ellipse, spline or polyline-arc ends |
| Insertion | text, mtext, blocks, attributes, attdefs | TEXT, MTEXT, INSERT; no ATTDEF or MLEADER |
| Perpendicular | every curve, deferred | lines, arcs, circles, xlines; deferred for the first point (#168) |
| Tangent | circles, arcs, ellipses, splines, deferred | arcs and circles; deferred for the first point; **no ellipse or spline** |
| Nearest | every curve | yes |
| Apparent intersection | projected intersection | yes in 2D: where objects near the cursor would meet if extended; an object rested on sends its whole extension, so its crossing with another object snaps too (`snap.rs` `osnap_with`, `snap/tracking.rs`) |
| Parallel | yes, with parallel path | yes: rest on a line, then a path parallel to it runs through the base point (`snap/tracking.rs`) |
| Temporary overrides typed at a prompt (`END`, `MID`, `INT` …) | yes | **no**: the text arrives as plain input |
| Shift/Ctrl+right-click snap menu | yes | yes: From, Mid Between 2 Points, Point Filters, the snap modes, None, Osnap Settings; no Temporary track point yet (`context_menu.rs`) |
| Tab to cycle candidate snaps | yes | yes, through the closest snap of each running mode under the cursor (`pointmod::snap_candidates`) |
| AutoSnap marker, tooltip, magnet, aperture box | yes | marker per mode and name label; aperture from APERTURE; no magnet, no aperture box |
| OSNAPZ, 3D object snaps | yes | no |

## Tracking, polar, ortho, grid and snap (≈ 70%)

| Feature | AutoCAD | AsenaCAD |
|---|---|---|
| Ortho (F8) | yes | yes (`snap.rs:503`) |
| Polar tracking increment | yes | yes, measured from ANGBASE; dotted path and "Polar: d < a°" tooltip (`snap/tracking.rs`, `canvas.rs` `draw_tracking`) |
| Additional polar angles, relative to last segment | yes | additional angles yes (POLARADDANG with POLARMODE 4); **relative to the last segment no** |
| Polar distance snap (PolarSnap) | yes | **no** |
| **Object snap tracking** (acquire points, alignment paths, intersections of paths) | yes, F11 | yes (#379): resting on a snap point acquires it (`+`), resting again releases it, seven at most; horizontal/vertical paths, or every polar angle with POLARMODE 2; Shift to acquire with POLARMODE 8; crossings of two paths, a path and an object, and a path and a polar path snap; tooltip names each path; a typed distance (direct distance entry) runs along the path the cursor is on, measured from the point the path comes from. Engine: `Session::snap_cursor`, `snap/tracking.rs`; AUTOSNAP bits 8 and 16 |
| Grid (rectangular, major lines, adaptive, limits) | yes | rectangular and isometric (30°, 90° and 150° lines), major lines, adaptive display |
| Snap (rectangular, isometric, polar snap) | yes | rectangular and isometric (SNAPSTYL 1: the snap Y spacing along the isometric axes), from SNAPBASE turned by SNAPANG |
| Isometric drafting (ISODRAFT, isoplanes F5) | yes | yes (#556): ISODRAFT and the status-bar toggle set SNAPSTYL and the plane; ISOPLANE / F5 / Ctrl+E cycle Left, Top, Right (SNAPISOPAIR); ortho, direct distance entry and the crosshair follow the plane's axes; ELLIPSE Isocircle (radius or Diameter, also for elliptical arcs); Drafting Settings snap style radio. Engine: `snap/iso.rs`, `cmd/settings/iso.rs` |
| Drafting Settings dialog | yes | yes (`dialogs.rs:36-80`) |
| Hatch and Gradient / Hatch Edit dialogs | yes | yes (`hatchdlg.rs`): pattern swatches, gradient colours and shapes, origin, pick points or objects, associative, separate, island detection; opened by HATCH, GRADIENT (HPDLGMODE), HATCHEDIT, right-click or double-clicking a hatch; no Preview or Inherit Properties |

## Dynamic input (≈ 45%)

AutoCAD's heads-up input (F12) has editable pointer-input fields (x, y), dimensional input
fields (length and angle) with Tab between them and lock icons, relative-by-default second
points, dimensional input on grips, a prompt with clickable options and a down-arrow menu.
AsenaCAD has pointer input boxes since #60 (`crates/ui-egui/src/dyninput.rs`): value boxes
beside the cursor while a command asks for a point; second and later points default to polar
distance and angle (DYNPIFORMAT = 1 for Cartesian); `,` and `<` lock a value and move to the next
box; Tab types the separator; Enter sends a relative entry (DYNPICOORDS = 1 for absolute) and
fills an empty box from the cursor; a plain number stays direct distance entry. Missing:
dimensional input on grips (length/angle fields while stretching), lock icons, the prompt with a
down-arrow options menu (DYNPROMPT), and DYNDIM.

## Grips (≈ 40%)

| Feature | AutoCAD | AsenaCAD |
|---|---|---|
| Stretch grips per object type | yes | lines (midpoint moves), arcs (start/mid/end), ellipses (axes), polylines (vertices; segment midpoints move or reshape the bulge), dimensions (text and definition points), MLEADER, viewport, wipeout, spline (`grips.rs:79-270`) |
| Text/MText grips | position, width, column grips | move the whole object |
| Hot-grip modes with Space/Enter cycling | Stretch, Move, Rotate, Scale, Mirror | yes (`canvas.rs:58-67`) |
| Hot-grip options: Base point, Copy, Undo, Reference, eXit | yes | **advertised in the prompt but not parsed**; the engine's JSON grip commands accept `copy`, the UI never passes it |
| Several hot grips at once (Shift-click) | yes | **no** (one handle) |
| Multifunctional grip hover menus (Add Vertex, Remove Vertex, Convert to Arc, Lengthen …) | yes | **no** |
| Grip dimensional input (length/angle fields) | yes | no |
| Dynamic block grips | yes | no (no dynamic blocks) |
| Grip appearance (GRIPSIZE, colours, hover) | yes | GRIPSIZE, GRIPS and GRIPOBJLIMIT honoured (Options ▸ Selection); no grip colours |

## Selection (≈ 40%)

| Feature | AutoCAD | AsenaCAD |
|---|---|---|
| Pick, implied window (left→right) and crossing (right→left) | yes | yes, coloured and dashed rectangles (`lib.rs:576-589`, `canvas.rs:823-836`), filled as SELECTIONAREA, WINDOWAREACOLOR, CROSSINGAREACOLOR and SELECTIONAREAOPACITY say (Visual Effect Settings) |
| Keywords W, C, WP, CP, F, BOX, AU, SI, M, G (group), CL (class) | yes | yes except CLass: W, C, F, WP, CP (#226); BOX, AUto, SIngle, Multiple, Group, Add, Remove, Undo (#428); ALL, Last, Previous |
| Press-drag selection windows (PICKDRAG) | yes | yes (#63, PICKDRAG = 2) |
| Lasso | yes | **no** |
| Selection cycling (SELECTIONCYCLING) | yes | toggle nothing reads |
| Selection preview, hover highlight | yes | selected and hovered objects redrawn in the selection colour, fills half see-through (SELECTIONEFFECT 1; 0 dashed), selection effect color; HIGHLIGHT (not saved), SELECTIONPREVIEW, PREVIEWEFFECT, PREVIEWFILTER (Visual Effect Settings) |
| PICKADD, PICKFIRST, PICKAUTO | yes | PICKADD (Shift), PICKFIRST |
| QSELECT, SELECTSIMILAR, FILTER | yes | QSELECT dialog; SELECTSIMILAR with SELECTSIMILARMODE (#428, SEttings asks for the bit code; no dialog); no FILTER |
| Object isolate/hide (ISOLATEOBJECTS, HIDEOBJECTS) | yes | yes, temporary (OBJECTISOLATIONMODE 0; #428): Tools > Isolate, UNISOLATEOBJECTS; no status-bar button |
| Groups selectable as one (PICKSTYLE) | yes | yes (PICKSTYLE 1, #417), Object grouping on Options ▸ Selection; associative hatch selection (bit 2) is stored, not used |

## Command line (≈ 60%)

| Feature | AutoCAD | AsenaCAD |
|---|---|---|
| AutoComplete (commands, sysvars, mid-string, synonyms) | yes | command list with Tab to accept (`cmdline.rs:30,176,317`); no sysvar or mid-string search confirmed |
| Aliases | about 250 in `acad.pgp` | 144 aliases across the commands (counted from source) |
| History (F2), Up/Down recall | yes | yes |
| Transparent commands (`'zoom`, `'pan`, `'osnap`) | yes | yes (`lib.rs:496-503`) |
| Enter/Space repeats last command | yes | yes |
| Clickable keywords in the prompt | yes | yes (`cmdline.rs:253-267`) |
| UNDO with Mark, Back, BEgin, End, Auto, Control | yes | count only, whole-document snapshots capped at 2,000 (`lib.rs:477`, `edit.rs:56`) |
| SETVAR for every system variable | ~1,000 sysvars | about 60 session/header variables by name (`sysvars.rs`) |
| Scripts (SCRIPT, `.scr`) | yes | script text via CLI/MCP/control; no `.scr` file command |
| Floating/docked command window, transparency, lines of history | yes | the Windows command line bar: see [Windows baseline](#windows-autocad-2026-baseline) |

## Chrome and palettes (≈ 45%)

| Feature | AutoCAD for Mac | AsenaCAD |
|---|---|---|
| Menu bar | 11 menus, 491 leaf items | same 11 menus; 245/491 items covered (233 by `cargo xtask parity`, which counts engine commands only, plus 12 UI-only items: see [parity-checklist.md](parity-checklist.md)) |
| Tool Bar, File tabs, Start tab, layout tabs | yes | yes |
| Tool Sets palette (Drafting, Modeling, … with flyouts) | yes | yes for 2D groups |
| Properties Inspector, Quick Properties | yes | yes (every control acts or is visibly unavailable, #182); block references show read-only Position X/Y/Z from their stored insertion point; coordinate editing is still missing |
| Layer Properties Manager (filters, states, VP overrides) | yes | yes |
| Blocks palette, Content | yes | Blocks window (click to insert, #28) |
| Status bar toggles | ~25 | the Windows status bar, its ▾ menus and Customization list: see [Windows baseline](#windows-autocad-2026-baseline) |
| ViewCube | interactive 3D | clickable for 2D (Top/Home → extents); dragging does nothing (#7) |
| Navigation bar, SteeringWheels | yes | the Windows navigation bar: see [Windows baseline](#windows-autocad-2026-baseline); no SteeringWheels |
| Workspaces | Drafting & Annotation, 3D Modeling | no |
| Options / Preferences dialog | many tabs | **no** (issue #23) |
| Customisation (CUI, aliases editor, tool palettes) | yes | no |
| Themes | Dark, Light | Dark, Light, System (#179) |
| Accessibility | VoiceOver basics | AccessKit through eframe; screen-reader coverage untested; magnifier issue #39; animation effects follow the OS setting (View ▸ Accessibility ▸ Animation Effects: System, On, Off) |

## Navigation (≈ 55%)

| Feature | AutoCAD | AsenaCAD |
|---|---|---|
| ZOOM Extents, All, Window, Previous, In/Out, Center, Scale, Object | yes | yes, 50-entry history (`cmd/view.rs:107-149`) |
| ZOOM Realtime (drag), Dynamic | yes | **no** |
| Wheel zoom about cursor, ZOOMFACTOR, ZOOMWHEEL | yes | fixed factor (no ZOOMFACTOR) |
| Middle-drag pan, double-middle-click extents | yes | yes (`canvas.rs:538-583`) |
| Trackpad two-finger pan, pinch zoom (macOS) | yes | yes (#58) |
| Named views (VIEW) | yes | **no** (VIEW table not read or written) |
| Tiled model-space viewports (VPORTS) | yes | not confirmed: VPORTS maps to paper-space MVIEW |
| 3D orbit, ViewCube drag, SteeringWheels | yes | no (M11) |

## Keyboard and context menus (≈ 25%)

- Shortcut table: 28 entries (Cmd shortcuts, F1, F2, F3, F5, F7–F12) plus 23 command `.key()`
  shortcuts. AutoCAD for Mac's defaults are similar in number; F4 (3D osnap) and F6 (dynamic UCS)
  are missing.
- Delete erases the selection on macOS (#64); Escape cancels.
- Right-click on the canvas is Enter while a command runs; holding it (250 ms, AutoCAD's
  time-sensitive right-click) opens the command menu: Enter, Cancel, Recent Input, the prompt's
  options, Snap Overrides, Pan/Zoom. Otherwise it opens a shortcut menu (Repeat, Recent Commands,
  Clipboard ▸, Isolate ▸, the selection's editing commands, Draw Order ▸, Group, the selected
  object type's edit commands, Select Similar, Deselect All, Undo/Redo, Pan/Zoom; #325, #512).
  The command line, drawing tabs, layout tabs, Layers palette rows and the Properties palette
  have menus too (#512). Still missing: in-place layout tab rename, Move or Copy, Select All
  Layouts.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-19 | minor | Output ▸ Plot drops to ◐: View Details has no command |
| 2026-10-19 | minor | Express Tools ▸ Web ▸ URL Options is a drop-down button with Show URLs, Change URLs and Find and Replace URLs (observed) |
| 2026-10-19 | minor | Express Tools ▸ Modify's two rows are centred in the panel as in AutoCAD; adjacent-large-button rule no longer treats separators as large buttons |
| 2026-10-19 | trivial | Only large toggles are at least 38 px wide (AutoCAD's View Cube); other large buttons keep the 42 px minimum, which keeps Home, Insert and Annotate at their measured widths |
| 2026-10-19 | minor | Manage ▸ Action Recorder has its slide-out (the Action Tree, disabled); the Named Views list stays enabled with no views; View Manager's row notes that VIEW asks on the command line |
| 2026-10-19 | minor | Ribbon palette toggles (Home and View, Parametric's Parameters Manager) close their palette when clicked while pressed; View ▸ Palettes ▸ Count starts COUNT; Parametric ▸ Manage becomes ✅ |
| 2026-10-19 | minor | View ▸ Model Viewports: the two and three-viewport arrangements and Join are commands of their own (`vports.2v` … `vports.3h`, `vports.join`); nine fewer ribbon items without a command |
| 2026-10-19 | minor | Windows baseline: the Express Tools tab's eight panels (8 rows, all ◐); every ribbon tab is built; 44 more ribbon items without a command |
| 2026-10-19 | minor | Windows baseline: the Output tab's two panels and the Collaborate tab's four (6 rows: 2 ✅, 4 ◐); Collaborate's online tools listed as intentionally absent but shown disabled; 12 more ribbon items without a command; adjacent large buttons stand with no gap between them, as in AutoCAD |
| 2026-10-19 | minor | Windows baseline: the Manage tab's six panels (6 rows, all ◐); 22 more ribbon items without a command |
| 2026-10-19 | minor | Windows baseline: the View tab's seven panels (7 rows: 1 ✅, 6 ◐); Layout Tabs shows and hides the layout tabs; TEXTSCR is a known command; large buttons are at least 38 px wide and panels can draw AutoCAD's thin separators between groups (Parametric's Geometric and Dimensional panels) |
| 2026-10-19 | minor | Windows baseline: the Parametric tab's three panels (3 rows: 1 ✅, 2 ◐); three more ribbon items without a command |
| 2026-10-19 | minor | Review fixes: Set Location and the block gallery (Home and Insert) are drop-down buttons; launchers carry AutoCAD's names; Home ▸ Annotation drops to ◐ (DIM makes only linear dimensions); Annotation Text Height drops down a list of heights; TEXTSIZE is capped at 1,000,000 |
| 2026-10-19 | minor | Windows baseline: the Annotate tab's seven panels (7 rows: 2 ✅, 5 ◐); Find Text finds and selects text, Annotation Text Height sets TEXTSIZE (now checked by SETVAR) |
| 2026-10-18 | minor | Windows baseline: the Insert tab's seven panels (7 rows: 1 ✅, 6 ◐); large labels of two or more words always take two lines, and a large split's ▾ sits right of the second line |
| 2026-10-18 | major | Windows AutoCAD 2026 baseline: side-by-side rows for the title row, tab row, each Home panel, ribbon collapse, document tabs, viewport controls, ViewCube, navigation bar, command line, layout bar, status bar and Application menu (9 ✅, 15 ◐, 1 ✗); Autodesk online items listed as intentionally absent; ribbon items without a command listed (checked by a test); collapsed panel buttons keep whole words; ViewCube Home on hover; ZOOM prompt comma. The Mac table's command line, status bar and navigation bar rows point to it |
| 2026-10-18 | minor | Status bar fits narrow windows; placeholder commands (ATTACH, SHARE, 3DORBIT) show disabled; 2D Object Snap with no modes opens Drafting Settings |
| 2026-10-18 | minor | Navigation bar below the ViewCube (Pan, Zoom and its variants, Orbit; NAVBAR) and the drawing window's buttons at the top-right corner |
| 2026-10-18 | minor | Command line bar as AutoCAD 2026 for Windows: grip, close (COMMANDLINEHIDE), Customize, recent commands, history toggle |
| 2026-10-18 | minor | Status bar row as AutoCAD 2026 for Windows: layout bar (≡ · Model · layouts · +), separator band, default items in order, ▾ menus, Customization list |
| 2026-10-12 | minor | Selected and hovered objects redrawn in the selection colour (text and arrowheads included); Options ▸ Selection laid out as AutoCAD's: Object grouping, Show grips, grip object limit, selection preview, Visual Effect Settings (selection area, preview filter) |
| 2026-10-12 | minor | Animation effects (dialog and menu fades, smoothed wheel scrolling and ⌘/Ctrl+wheel zoom) follow the operating system's setting; View ▸ Accessibility ▸ Animation Effects chooses System, On or Off |
| 2026-10-12 | minor | Hatch and Gradient / Hatch Edit dialogs (HATCH, GRADIENT, HATCHEDIT, double-click a hatch; HPDLGMODE) |
| 2026-10-12 | minor | Block references now show their stored insertion X/Y/Z in the Properties Inspector, including nonzero Z; fields are read-only |
| 2026-10-11 | minor | Isometric drafting (#556): isometric snap and grid, ISODRAFT, ISOPLANE (F5), ortho along isoplane axes, ELLIPSE Isocircle |
| 2026-10-11 | minor | Object snap tracking, Extension, Parallel, Apparent Intersection, additional polar angles (#379): tracking 30% → 70%, object snaps 40% → 45%; total 43% → 48% |
| 2026-10-10 | minor | Dynamic input 15% → 45% (pointer boxes, #60); press-drag windows (#63); total 40% → 43% |
| 2026-10-10 | major | First UI-parity audit: point entry, snaps, tracking, dynamic input, grips, selection, command line, chrome, navigation, keys |
