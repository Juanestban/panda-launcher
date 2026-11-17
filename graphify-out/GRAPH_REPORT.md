# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 63 files · ~47,142 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 13 file(s) not represented in the graph (top: (none) 5, .ttf 2, .ftl 2)

## Summary
- 660 nodes · 1248 edges · 45 communities (41 shown, 4 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 27 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a13d2f2d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Roadmap
- Architecture
- Design system
- 0002. zgui for the UI, pinned to a git revision
- 0005. Fabric is the only mod loader
- 0006. Per-version isolated profiles
- fabric.rs
- Panda Launcher
- java.rs
- auth.rs
- 0009. MSI and portable distribution (archi-games layout)
- update.rs
- 0011. Fluent i18n, Spanish and English
- 0001. Cargo workspace with a UI-free core
- 0003. Microsoft auth with device code and our own Azure client ID
- version.rs
- state.rs
- panda-core
- 0004. Offline play only after verified ownership
- mod.rs
- download.rs
- 0012. Astro website on GitHub Pages
- 0007. Modrinth as the only mod source, with client-side dependency resolution
- 0008. Managed Java runtime
- i18n.rs
- mods.rs
- 0010. Self-update from GitHub Releases, signed with ed25519
- mod_page.rs
- main.rs
- [lang]/index.astro
- tr
- i18n.ts
- Base.astro
- package.json
- play.rs
- icons.rs
- ui/mods.rs
- Releasing
- Icon.astro
- Account
- dependencies
- UpdateState

## God Nodes (most connected - your core abstractions)
1. `AppState` - 45 edges
2. `Dirs` - 40 edges
3. `AuthError` - 23 edges
4. `core()` - 21 edges
5. `tr()` - 14 edges
6. `install()` - 13 edges
7. `Version` - 11 edges
8. `UpdateState` - 11 edges
9. `Roadmap` - 11 edges
10. `install()` - 10 edges

## Surprising Connections (you probably didn't know these)
- `mods_dir()` --calls--> `profile_name()`  [INFERRED]
  crates/panda-core/src/mods.rs → crates/panda-core/src/fabric.rs
- `record_path()` --calls--> `profile_name()`  [INFERRED]
  crates/panda-core/src/mods.rs → crates/panda-core/src/fabric.rs
- `expand()` --calls--> `allowed()`  [INFERRED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs
- `Browse()` --calls--> `search()`  [INFERRED]
  crates/panda-launcher/src/ui/mods.rs → crates/panda-core/src/mods.rs
- `Details()` --calls--> `t()`  [INFERRED]
  crates/panda-launcher/src/ui/mod_page.rs → crates/panda-launcher/src/i18n.rs

## Import Cycles
- None detected.

## Communities (45 total, 4 thin omitted)

### Community 1 - "Roadmap"
Cohesion: 0.18
Nodes (11): Later (v2+), Out of scope, Phase 0: Prerequisites (blocks login), Phase 1: Workspace and CI, Phase 2: Core: auth and vanilla launch, Phase 3: Fabric profiles, Phase 4: Modrinth mods, Phase 5: UI (zgui) (+3 more)

### Community 2 - "Architecture"
Cohesion: 0.22
Nodes (9): Architecture, Flows, Install a mod ([ADR 0007](adr/0007-modrinth-only-dependency-resolution.md)), Launch, Network etiquette, On-disk layout, Sign in ([ADR 0003](adr/0003-microsoft-auth-device-code-own-client-id.md), [0004](adr/0004-offline-play-after-verified-ownership.md)), Update the launcher ([ADR 0010](adr/0010-github-releases-updater-signed.md)) (+1 more)

### Community 3 - "Design system"
Cohesion: 0.20
Nodes (10): Accessibility, Brand, Color, Design system, Icons, Motion, Principles, Spacing and radius (+2 more)

### Community 4 - "0002. zgui for the UI, pinned to a git revision"
Cohesion: 0.40
Nodes (5): 0002. zgui for the UI, pinned to a git revision, Alternatives, Consequences, Context, Decision

### Community 5 - "0005. Fabric is the only mod loader"
Cohesion: 0.40
Nodes (4): 0005. Fabric is the only mod loader, Consequences, Context, Decision

### Community 6 - "0006. Per-version isolated profiles"
Cohesion: 0.40
Nodes (4): 0006. Per-version isolated profiles, Consequences, Context, Decision

### Community 7 - "fabric.rs"
Cohesion: 0.17
Nodes (20): fabric_json(), FabricArguments, FabricProfile, game_versions(), GameVersion, install(), installs_real_fabric_profiles(), latest_loader() (+12 more)

### Community 8 - "Panda Launcher"
Cohesion: 0.50
Nodes (3): graphify, Panda Launcher, Rules

### Community 9 - "java.rs"
Cohesion: 0.08
Nodes (31): ALL, DONE, ensure(), Entry, EntryDownloads, falls_back_to_same_major_version(), java_exe(), pick() (+23 more)

### Community 10 - "auth.rs"
Cohesion: 0.06
Nodes (55): Account, entry(), load(), Resume, resume(), Offline, Online, SignedOut (+47 more)

### Community 11 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.40
Nodes (4): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision

### Community 12 - "update.rs"
Cohesion: 0.08
Nodes (21): AFTER_UPDATE_ARG, apply(), check(), download(), enabled(), expected_sum(), FULL, GhAsset (+13 more)

### Community 13 - "0011. Fluent i18n, Spanish and English"
Cohesion: 0.40
Nodes (4): 0011. Fluent i18n, Spanish and English, Consequences, Context, Decision

### Community 14 - "0001. Cargo workspace with a UI-free core"
Cohesion: 0.50
Nodes (4): 0001. Cargo workspace with a UI-free core, Consequences, Context, Decision

### Community 15 - "0003. Microsoft auth with device code and our own Azure client ID"
Cohesion: 0.50
Nodes (4): 0003. Microsoft auth with device code and our own Azure client ID, Consequences, Context, Decision

### Community 16 - "version.rs"
Cohesion: 0.10
Nodes (31): Action, Allow, Disallow, allowed(), Arg, Plain, Ruled, Arguments (+23 more)

### Community 17 - "state.rs"
Cohesion: 0.14
Nodes (20): ModPage(), check_updates(), Core, core(), install_mod(), install_update(), installed_versions(), logo_path() (+12 more)

### Community 19 - "0004. Offline play only after verified ownership"
Cohesion: 0.40
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 20 - "mod.rs"
Cohesion: 0.16
Nodes (12): AccountBox(), App(), NavItem(), Shell(), Sidebar(), UpdateBanner(), AppState, Page (+4 more)

### Community 21 - "download.rs"
Cohesion: 0.09
Nodes (18): download_verified(), fetch(), fetch_all(), FileSpec, get_json(), hex(), json(), PARALLEL (+10 more)

### Community 22 - "0012. Astro website on GitHub Pages"
Cohesion: 0.40
Nodes (5): 0012. Astro website on GitHub Pages, Consequences, Context, Decision, Implementation notes (2026-11-17)

### Community 23 - "0007. Modrinth as the only mod source, with client-side dependency resolution"
Cohesion: 0.40
Nodes (4): 0007. Modrinth as the only mod source, with client-side dependency resolution, Consequences, Context, Decision

### Community 24 - "0008. Managed Java runtime"
Cohesion: 0.40
Nodes (4): 0008. Managed Java runtime, Consequences, Context, Decision

### Community 25 - "i18n.rs"
Cohesion: 0.20
Nodes (8): both_languages_have_the_same_keys(), bundle(), keys(), Lang, En, Es, placeholders_are_filled(), set_lang()

### Community 26 - "mods.rs"
Cohesion: 0.07
Nodes (37): API, Dependency, find_conflict(), get(), Hashes, icon(), incompatible_of(), install() (+29 more)

### Community 27 - "0010. Self-update from GitHub Releases, signed with ed25519"
Cohesion: 0.40
Nodes (5): 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision, Implementation notes (Phase 6)

### Community 28 - "mod_page.rs"
Cohesion: 0.09
Nodes (20): compact(), date(), trim(), Block, Code, Heading, Item, Paragraph (+12 more)

### Community 29 - "main.rs"
Cohesion: 0.15
Nodes (6): USER_AGENT, GEIST, GEIST_MONO, lock_instance(), main(), window_icon()

### Community 30 - "[lang]/index.astro"
Cohesion: 0.16
Nodes (12): { lang, release, size = "lg" }, s, lang, s, shots, small, words, Asset (+4 more)

### Community 31 - "tr"
Cohesion: 0.18
Nodes (6): t(), tr(), SHEET, AboutPage(), MEMORY_GB, SettingsPage()

### Community 32 - "i18n.ts"
Cohesion: 0.22
Nodes (9): en, es, href(), Lang, langs, strings, t(), section() (+1 more)

### Community 33 - "Base.astro"
Cohesion: 0.15
Nodes (8): canonical, links, navLinks, og, Props, reveal, s, spy

### Community 34 - "package.json"
Cohesion: 0.15
Nodes (11): astro, @fontsource-variable/geist, @fontsource-variable/geist-mono, @phosphor-icons/core, name, private, scripts, build (+3 more)

### Community 35 - "play.rs"
Cohesion: 0.24
Nodes (9): PlayPage(), PlayPanel(), PlayStatus(), SignInPanel(), PlayState, Failed, Idle, Running (+1 more)

### Community 36 - "icons.rs"
Cohesion: 0.38
Nodes (8): DOWNLOAD, GEAR, INFO, PLAY, PUZZLE, SEARCH, SIGN_OUT, TRASH

### Community 37 - "ui/mods.rs"
Cohesion: 0.33
Nodes (7): Browse(), InstallButton(), InstalledRow(), ModCard(), ModIcon(), ModsPage(), Notice()

### Community 38 - "Releasing"
Cohesion: 0.33
Nodes (6): If something goes wrong, One-time setup, Packaging locally (Windows), Releasing, Steps, What it publishes

### Community 39 - "Icon.astro"
Cohesion: 0.47
Nodes (4): { name, size = 20 }, svg, IconName, icons

### Community 40 - "Account"
Cohesion: 0.40
Nodes (5): Account, Checking, Code, SignedIn, SignedOut

### Community 41 - "dependencies"
Cohesion: 0.40
Nodes (5): dependencies, astro, @fontsource-variable/geist, @fontsource-variable/geist-mono, @phosphor-icons/core

### Community 42 - "UpdateState"
Cohesion: 0.50
Nodes (4): UpdateState, Checking, Idle, UpToDate

## Knowledge Gaps
- **57 isolated node(s):** `panda-core`, `SignedOut`, `NoClientId`, `Expired`, `Declined` (+52 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 277 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Dirs` connect `mods.rs` to `fabric.rs`, `java.rs`, `version.rs`, `state.rs`, `download.rs`?**
  _High betweenness centrality (0.056) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `core()` (e.g. with `ModPage()` and `Browse()`) actually correct?**
  _`core()` has 4 INFERRED edges - model-reasoned connections that need verification._
- **What connects `panda-core`, `SignedOut`, `NoClientId` to the rest of the system?**
  _57 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `java.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08067226890756303 - nodes in this community are weakly interconnected._
- **Why does `Error` connect `java.rs` to `fabric.rs`, `update.rs`, `version.rs`, `download.rs`, `mods.rs`?**
  _High betweenness centrality (0.046) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.055900621118012424 - nodes in this community are weakly interconnected._
- **Why does `AppState` connect `mod.rs` to `play.rs`, `ui/mods.rs`, `Account`, `UpdateState`, `state.rs`, `download.rs`, `mods.rs`, `mod_page.rs`, `tr`?**
  _High betweenness centrality (0.042) - this node is a cross-community bridge._