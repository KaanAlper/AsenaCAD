# Releasing AsenaCAD

The canonical recipe is craftrules' [`release/playbook.md`](https://github.com/storytold/craftrules/blob/main/release/playbook.md); this file covers only what's AsenaCAD-specific.

- **Version:** `[workspace.package] version` in the root `Cargo.toml` is the single source. `cargo xtask version` prints it; `cargo xtask version set X.Y.Z[-pre]` changes it. The app shows it in About, `asenacad --version` and `asenacad-cli --version`; release builds also embed the commit (`ASENACAD_BUILD_SHA`).
- **Release workflow: not built yet** ([gaps.md](gaps.md) S7). There is no workflow that builds, signs, notarizes or publishes release packages; the shared one AsenaCAD inherited was removed. Until it exists, packages come only from the packaging scripts run by hand (below), unsigned.
- **Packages the scripts make:** macOS `.dmg` (`packaging/macos/package.sh`), Windows `.msi` + portable `.zip` per architecture (`packaging/windows/package.ps1`), Linux `.AppImage`, `.deb`, `.rpm`, `.tar.gz` and a `.flatpak` bundle (`packaging/linux/package.sh`, `packaging/linux/flatpak-bundle.sh`; the from-source Flathub manifest is in `packaging/linux/flatpak/`), web `.zip` (`packaging/web/package.sh`, see `packaging/web/README.md`).
- **Windows installer:** `packaging/windows/asenacad.wxs` (WiX v5 + `WixToolset.UI.wixext`). Its wizard has a Shortcuts page: the Start Menu shortcut is always created, the desktop shortcut is opt-in (`DESKTOPSHORTCUT=1` for silent installs, `DESKTOPSHORTCUT=0` drops it on an upgrade; otherwise upgrades keep the previous choice).
- **File types:** `.dxf` and `.dwg` (macOS document types, Windows "Open with", Linux MIME types).
- **Local packaging test:** `packaging/macos/package.sh` (ad-hoc signed DMG), `packaging/linux/package.sh`, `packaging/web/package.sh`.
