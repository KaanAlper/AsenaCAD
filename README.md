# AsenaCAD

AsenaCAD is an open-source, clean-room CAD application written in pure Rust whose interface,
commands and workflow match Autodesk AutoCAD one to one. It runs natively on Linux, Windows,
macOS and FreeBSD, and in the browser (WebAssembly).

AsenaCAD is a fork of [CADCraft](https://github.com/storytold/cadcraft) by the ArtCraft team and
the CADCraft contributors (see `NOTICE`).

## Build and run

    cargo run --release -p asenacad -- --sample
    cargo run --release -p asenacad-cli -- --help

Before every commit: `cargo xtask ci`.

## Status

Pre-alpha. Goals and milestones: [docs/specs/2026-10-18-asenacad-design.md](docs/specs/2026-10-18-asenacad-design.md).
Parity with AutoCAD is tracked in `docs/*-parity.md`.

## License

MIT OR Apache-2.0. AsenaCAD is not affiliated with Autodesk, Inc. AutoCAD and DWG are
trademarks of Autodesk, Inc.
