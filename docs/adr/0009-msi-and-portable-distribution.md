# 0009. MSI and portable distribution (archi-games layout)

Date: 2026-11-17 · Status: Accepted

## Context
The owner wants an MSI or a portable exe and already ships a working setup in [archi-games](https://github.com/Juanestban/archi-games). Shipped code must be pure Rust. Packaging tools are a separate matter.

## Decision
Copy the archi-games setup:
- **WiX 5 as a .NET local tool** (`dotnet-tools.json`) building `wix/main.wxs`. WiX is allowed only for packaging, never at runtime.
- **Per-user MSI**: `Scope="perUser"`, installs to `%LOCALAPPDATA%\Programs\Panda Launcher\`, no admin, Start menu shortcut, a fixed `UpgradeCode` and `MajorUpgrade`. The `LaunchAfterUpdate` custom action runs the exe with `--after-update` when `LAUNCHAPP=1`.
- `scripts/package.ps1` builds the release exe, the MSI, the portable copy, `SHA256SUMS.txt` and the `.sig` files ([0010](0010-github-releases-updater-signed.md)) into `target/dist`.
- `release.yml` on tag `v*.*.*` (windows-latest) fails if the tag ≠ `v` + the Cargo version, runs the tests, packages, and uploads with `softprops/action-gh-release`.
- Asset names: `PandaLauncher-<v>-x64.msi`, `PandaLauncher-<v>-x64-portable.exe`, `SHA256SUMS.txt`, `<asset>.sig`.

## Consequences
- Proven layout; no admin prompts.
- CI needs .NET 8 for WiX.
- No Authenticode code signing yet, so SmartScreen may warn on first runs (deferred).
