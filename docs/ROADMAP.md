# Roadmap

Panda Launcher: a Minecraft Java Edition launcher in pure Rust. Microsoft login with ownership check, Fabric-only per-version profiles, one-click Modrinth mods, self-updating MSI and portable builds.

Decisions behind each item live in [`adr/`](adr/). Architecture is in [`architecture.md`](architecture.md), visual rules in [`design-system.md`](design-system.md).

Status keys: `[ ]` todo, `[~]` in progress, `[x]` done.

---

## Phase 0: Prerequisites (blocks login)

- [ ] **Register the Azure app**: personal Microsoft accounts only, "Allow public client flows: Yes", no secret, no redirect URI. ([ADR 0003](adr/0003-microsoft-auth-device-code-own-client-id.md))
- [ ] **Request Minecraft API access** for that client ID at <https://aka.ms/mce-reviewappid>. Approval takes days to weeks; until then `login_with_xbox` returns `403 Invalid app registration`.
- [ ] Add `PANDA_MSA_CLIENT_ID` as a GitHub Actions secret.
- [ ] Generate the ed25519 release key pair; private key → `PANDA_UPDATE_SIGNING_KEY` secret, public key → committed in `panda-core`. ([ADR 0010](adr/0010-github-releases-updater-signed.md))
- [~] Make the repo public; add `LICENSE-MIT` and `LICENSE-APACHE` (licences added).

## Phase 1: Workspace and CI

- [x] Convert to a Cargo workspace: `crates/panda-core`, `crates/panda-launcher`. ([ADR 0001](adr/0001-rust-workspace-core-ui-split.md))
- [x] `ci.yml` on `windows-latest`: `cargo fmt --check`, `clippy -D warnings`, `cargo test --workspace --locked`.
- [x] Shared HTTP client with the project User-Agent `panda-launcher/<version> (+https://github.com/juanestban/panda-launcher)`, with no personal data.

## Phase 2: Core: auth and vanilla launch

- [~] Microsoft device-code login → Xbox Live → XSTS → Minecraft token → entitlements → profile. ([ADR 0003](adr/0003-microsoft-auth-device-code-own-client-id.md))
- [x] Map XSTS `XErr` codes to user-facing messages (no Xbox account, child account, region blocked).
- [~] Store the refresh token and cached ownership in the OS keychain (`keyring`). ([ADR 0004](adr/0004-offline-play-after-verified-ownership.md))
- [~] Silent refresh on start; offline play only for a previously verified account.
- [x] Version manifest, version JSON, libraries (OS rules), natives, asset index and objects; SHA-1 verified and resumable.
- [x] Managed Java runtime from Mojang's runtime manifest. ([ADR 0008](adr/0008-managed-java-runtime.md))
- [~] Build launch arguments and spawn the game; stream logs to a file.

## Phase 3: Fabric profiles

- [x] Fabric loader profile from `meta.fabricmc.net`, merged over the vanilla version JSON. ([ADR 0005](adr/0005-fabric-only.md))
- [x] Isolated profile directories `profiles/<version>-fabric/`, launched with `--gameDir`. ([ADR 0006](adr/0006-per-version-isolated-profiles.md))
- [x] Create a profile on first play of a version; one profile per version.

## Phase 4: Modrinth mods

- [x] Search filtered by game version and `fabric`. ([ADR 0007](adr/0007-modrinth-only-dependency-resolution.md))
- [x] Install the newest compatible version into the profile's `mods/`, verified with SHA-512.
- [x] Resolve `required` dependencies recursively; refuse `incompatible` ones.
- [x] Per-profile `panda-mods.json` that records what was installed and why.
- [x] Uninstall, with a warning when other installed mods depend on it.
- [x] Respect the rate limit (300 req/min) and the `X-Ratelimit-*` headers.

## Phase 5: UI (zgui)

- [x] Pin zgui to a git revision. ([ADR 0002](adr/0002-zgui-pinned-ui.md))
- [x] Theme from [`design-system.md`](design-system.md).
- [x] Screens: sign-in (device code), Play (version picker + Play button), Mods (search, installed), About (versions, licences, disclaimer).
- [ ] Settings screen (language, memory, auto-update).
- [x] Download and launch progress; skeleton loading; inline errors.
- [x] Spanish and English with Fluent, defaulting to the system locale. ([ADR 0011](adr/0011-fluent-i18n-es-en.md))

## Phase 6: Distribution and updates → **v1.0**

- [ ] `wix/main.wxs` per-user MSI, plus `scripts/package.ps1`, following the archi-games layout. ([ADR 0009](adr/0009-msi-and-portable-distribution.md))
- [ ] `release.yml` on tag `v*.*.*`: checks the tag against the Cargo version, then tests, packages, signs and uploads the MSI, the portable exe, `SHA256SUMS.txt` and the `.sig` files.
- [ ] Updater: check on start and hourly, download on click, verify SHA-256 and ed25519, install, relaunch with `--after-update`. ([ADR 0010](adr/0010-github-releases-updater-signed.md))
- [ ] Single-instance lock with retry while the old process exits.
- [ ] `RELEASING.md`.

## Phase 7: Website

- [ ] `web/` Astro site on GitHub Pages, with `/es` and `/en`. ([ADR 0012](adr/0012-astro-website-github-pages.md))
- [ ] Tabs: Home, Download, Releases, Docs (guide + FAQ), Roadmap (from this file).
- [ ] Design tokens shared with the launcher.
- [ ] The release workflow redeploys the site so the download version is baked in.
- [ ] "Not an official Minecraft product" disclaimer in the footer.

## Later (v2+)

- Several named profiles per version (e.g. two modpacks on 1.21.10).
- Multiple Microsoft accounts.
- Update installed mods (Modrinth `version_files/update`).
- Import `.mrpack` modpacks.
- macOS and Linux builds.

## Out of scope

- Other mod loaders (Forge, NeoForge, Quilt). See [ADR 0005](adr/0005-fabric-only.md).
- CurseForge.
- Offline/cracked accounts.
