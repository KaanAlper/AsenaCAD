//! Windows only: embed the app icon and version info into `asenacad.exe`. Elsewhere a no-op.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../assets/app-icon/asenacad.ico");
    println!("cargo:rerun-if-env-changed=ASENACAD_REQUIRE_WINRES");
    println!("cargo:rerun-if-env-changed=ASENACAD_BUILD_SHA");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let mut res = winresource::WindowsResource::new();
    if std::path::Path::new("../../assets/app-icon/asenacad.ico").exists() {
        res.set_icon("../../assets/app-icon/asenacad.ico");
    }
    res.set("ProductName", "AsenaCAD")
        .set("FileDescription", "AsenaCAD computer-aided design")
        .set("CompanyName", "AsenaCAD contributors")
        .set(
            "LegalCopyright",
            "Copyright (c) 2026 AsenaCAD contributors; based on CADCraft (c) 2026 ArtCraft Team and the CADCraft contributors. MIT OR Apache-2.0.",
        )
        .set("OriginalFilename", "asenacad.exe")
        .set("InternalName", "asenacad");
    if let Err(e) = res.compile() {
        if std::env::var_os("ASENACAD_REQUIRE_WINRES").is_some() {
            eprintln!("embedding Windows resources failed: {e}");
            std::process::exit(1);
        }
        println!("cargo:warning=asenacad.exe built without icon/version resources: {e}");
    }
}
