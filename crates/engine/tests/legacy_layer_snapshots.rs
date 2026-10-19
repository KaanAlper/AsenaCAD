//! LAYERP and LAYUNISO snapshots kept in a drawing saved before the rename (`$CADCRAFT_LAYERP`,
//! `$CADCRAFT_LAYISO`) work as before; AsenaCAD keeps them as `ASENACAD_LAYERP` and
//! `ASENACAD_LAYISO`, and neither name is listed as a system variable.

use asenacad_engine::Session;
use asenacad_engine::doc::{Drawing, Layer};
use serde_json::json;

/// A drawing whose layer Walls is off, with legacy snapshots where Walls is on.
fn legacy_session() -> Session {
    let mut d = Drawing::new_metric();
    d.layers.push(Layer::new("Walls"));
    let snapshot = serde_json::to_string(&d.layers).unwrap();
    for l in &mut d.layers {
        l.on = l.name != "Walls";
    }
    // The (code, value) lines of the saved file, with the two variables added to its HEADER.
    let text = asenacad_io::write_dxf(&d);
    let mut lines: Vec<String> = text.lines().map(|l| l.trim().to_string()).collect();
    let end = lines.chunks(2).position(|p| p == ["0", "ENDSEC"]).unwrap() * 2;
    let vars = ["9", "$CADCRAFT_LAYISO", "1", &snapshot, "9", "$CADCRAFT_LAYERP", "1", &snapshot].map(String::from);
    lines.splice(end..end, vars);
    let legacy = lines.join("\n") + "\n";
    let mut s = Session::empty();
    s.open_drawing(asenacad_io::read_dxf(legacy.as_bytes()).unwrap(), "legacy", None);
    s
}

fn walls_on(s: &Session) -> bool {
    s.doc().unwrap().layer("Walls").unwrap().on
}

#[test]
fn layuniso_restores_a_legacy_snapshot() {
    let mut s = legacy_session();
    assert!(!walls_on(&s));
    s.execute("layuniso", &json!({})).unwrap();
    assert!(walls_on(&s));
}

#[test]
fn layerp_restores_a_legacy_snapshot() {
    let mut s = legacy_session();
    s.execute("layerp", &json!({})).unwrap();
    assert!(walls_on(&s));
}

#[test]
fn legacy_snapshots_are_kept_under_the_new_names_and_not_listed() {
    let s = legacy_session();
    let vars = &s.doc().unwrap().header.vars;
    assert!(vars.contains_key("ASENACAD_LAYISO") && vars.contains_key("ASENACAD_LAYERP"));
    assert!(vars.keys().all(|k| !k.starts_with("CADCRAFT")), "{:?}", vars.keys());
    let listed = asenacad_engine::sysvars::list(&s).to_string();
    assert!(!listed.contains("ASENACAD_") && !listed.contains("CADCRAFT_"), "{listed}");
}
