<div align="center">

<img src="docs/assets/logo.png" alt="Panda Launcher logo" width="96">

# Panda Launcher

**A Minecraft Java launcher where every version keeps its own mods.
Microsoft sign in, Fabric and Modrinth, written in pure Rust.**

[![CI](https://github.com/juanestban/panda-launcher/actions/workflows/ci.yml/badge.svg)](https://github.com/juanestban/panda-launcher/actions/workflows/ci.yml)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
![Rust](https://img.shields.io/badge/rust-edition%202024-orange.svg)
![Platform](https://img.shields.io/badge/platform-Windows-0078D6.svg)

</div>

Pick a Minecraft version, install mods from Modrinth in one click, press Play. Each version gets its own
Fabric profile and its own `mods` folder, so switching between 1.21.10 and 26.3 never means moving files around.
The launcher installs the game, the Fabric loader and the right Java for you.

> [!NOTE]
> Panda Launcher is built and released for **Windows**. It also runs on macOS from source, untested beyond
> development. Signing in needs this project's Microsoft app to be approved for the Minecraft API
> (Phase 0 of the roadmap). Where each part stands is in [docs/ROADMAP.md](docs/ROADMAP.md).

## Ground rules

- **Legal only.** You sign in with your Microsoft account and the launcher checks that you own Minecraft Java Edition.
  There is no cracked or offline account mode; offline play works only for an account that was verified online before
  ([ADR 0004](docs/adr/0004-offline-play-after-verified-ownership.md)).
- **Your session stays on your machine.** Tokens go to the OS keychain (Windows Credential Manager), never to files or logs.
- **No personal data in requests.** Requests identify themselves only as `panda-launcher/<version>` plus the repository URL.
- **Fabric and Modrinth only** ([ADR 0005](docs/adr/0005-fabric-only.md), [ADR 0007](docs/adr/0007-modrinth-only-dependency-resolution.md)).
- **Not an official Minecraft product.** Not approved by or associated with Mojang or Microsoft.

## Platforms

| Platform | Status |
|---|---|
| Windows 10/11, x64 | Supported. MSI installer and portable exe, both self-updating. |
| macOS | Builds and runs from source. No releases, no self-update. |
| Linux | Not tried yet. The code is cross-platform Rust, so it may build; no releases. |

## Install

Every [release](https://github.com/juanestban/panda-launcher/releases) has two downloads for Windows:

- **`PandaLauncher-<version>-x64.msi`** installs for your user only, in `%LOCALAPPDATA%\Programs\Panda Launcher`,
  without admin rights. Uninstall it from *Settings → Apps*.
- **`PandaLauncher-<version>-x64-portable.exe`** is a single file. Put it anywhere and open it.

Both check for new versions and update themselves when you press **Update**
([ADR 0010](docs/adr/0010-github-releases-updater-signed.md)).

The files are not code signed, so Windows SmartScreen may say *"Windows protected your PC"*. To open them anyway:
**More info → Run anyway**. To check a file is the one we published, compare its SHA-256 (`Get-FileHash <file>`)
with `SHA256SUMS.txt` from the same release.

## Using the app

1. **Sign in.** On **Play**, press *Sign in with Microsoft*. The launcher shows a code and a button that opens
   Microsoft's sign in page; type the code there and approve. The launcher checks that you own the game.
2. **Pick a version.** The version picker on **Play** (also in **Mods**) lists the stable Minecraft versions Fabric supports.
3. **Add mods.** In **Mods**, browse what is popular for that version or search Modrinth. Press **Install** for the
   newest build, or open a mod to read its description and choose an exact build in **Versions**. Required
   dependencies install with it.
4. **Play.** Press **Play**. The first launch of a version downloads the game, Fabric and Java, with progress shown.
5. **Settings.** Language (Spanish or English, live), memory for the game, and automatic updates.

Everything lives in the launcher's data folder: `%LOCALAPPDATA%\PandaLauncher` on Windows,
`~/Library/Application Support/PandaLauncher` on macOS. Each version's mods are in
`profiles/<version>-fabric/mods`.

## Getting started (from source)

### Requirements

- [Rust](https://rustup.rs/) stable (edition 2024, Rust 1.85 or newer)
- Git

### Windows

1. Install the **Visual Studio Build Tools** with the *Desktop development with C++* workload. Rust needs them to
   link, and their Windows SDK includes `rc.exe`, which embeds the icon in the exe.
2. Install Rust from <https://rustup.rs/> (default toolchain, `x86_64-pc-windows-msvc`).
3. Clone and run:

   ```powershell
   git clone https://github.com/juanestban/panda-launcher.git
   cd panda-launcher
   cargo run -p panda-launcher
   ```

### macOS

```sh
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/juanestban/panda-launcher.git
cd panda-launcher
cargo run -p panda-launcher
```

### Signing in from a local build

Microsoft sign in needs an Azure app client ID at build time. Without it the app runs, but **Sign in** reports that
the build has no client ID. Never reuse another launcher's client ID
([ADR 0003](docs/adr/0003-microsoft-auth-device-code-own-client-id.md)).

```sh
PANDA_MSA_CLIENT_ID=<your-azure-client-id> cargo run -p panda-launcher
```

## Project layout

```
crates/panda-core       no UI: Microsoft auth, game and Fabric installs, Java, Modrinth mods, launch, updates
crates/panda-launcher   the app: zgui interface, styles, Spanish and English texts (locales/)
web/                    the website (Astro, GitHub Pages), the only non-Rust code
wix/, scripts/          MSI definition and the packaging script
docs/                   architecture, design system, roadmap and ADRs (docs/adr/)
```

## Development

```
cargo run -p panda-launcher                             # open the app
cargo test --workspace                                  # tests
cargo fmt --all --check                                 # format
cargo clippy --workspace --all-targets -- -D warnings   # lint
```

Tests that talk to Mojang, Fabric or Modrinth are ignored by default:
`cargo test --workspace -- --ignored`.

The website: `cd web && npm install && npm run dev`.

To publish a version, see [RELEASING.md](RELEASING.md). To package locally on Windows:

```
dotnet tool restore              # WiX 5, local to the repository
.\scripts\package.ps1            # -> target\dist (MSI, portable, SHA256SUMS.txt)
```

## Contributing

Issues and pull requests are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) first: it covers the setup, the
checks CI runs, and the few rules every change has to keep.

## License

At your option, [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE). Unless you say otherwise, any contribution is
published under those same two licenses.

## Credits

Interface built with [zgui](https://github.com/zortax/zgui). Geist and Geist Mono fonts (OFL), Phosphor icons (MIT).
Visual rules from [elayadesign/ai-design-skills](https://github.com/elayadesign/ai-design-skills).
Distribution and updater modelled on [archi-games](https://github.com/Juanestban/archi-games).
