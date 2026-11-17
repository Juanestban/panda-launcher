# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 40 files · ~14,526 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 4, .ttf 2, .ftl 2)

## Summary
- 457 nodes · 841 edges · 29 communities (26 shown, 3 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 15 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8cb2b2d6`
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
- account.rs
- 0011. Fluent i18n, Spanish and English
- 0001-rust-workspace-core-ui-split.md
- 0003. Microsoft auth with device code and our own Azure client ID
- version.rs
- state.rs
- panda-core
- 0004. Offline play only after verified ownership
- mod.rs
- download.rs
- prelude
- 0007. Modrinth as the only mod source, with client-side dependency resolution
- 0008. Managed Java runtime
- i18n.rs
- mods.rs
- 0010. Self-update from GitHub Releases, signed with ed25519

## God Nodes (most connected - your core abstractions)
1. `Dirs` - 34 edges
2. `AuthError` - 23 edges
3. `AppState` - 22 edges
4. `Version` - 17 edges
5. `core()` - 11 edges
6. `Roadmap` - 11 edges
7. `install()` - 11 edges
8. `install()` - 10 edges
9. `install()` - 10 edges
10. `Lang` - 9 edges

## Surprising Connections (you probably didn't know these)
- `play()` --calls--> `tr()`  [INFERRED]
  crates/panda-launcher/src/ui/state.rs → crates/panda-launcher/src/i18n.rs
- `Sidebar()` --references--> `AppState`  [EXTRACTED]
  crates/panda-launcher/src/ui/mod.rs → crates/panda-launcher/src/ui/state.rs
- `NavItem()` --references--> `AppState`  [EXTRACTED]
  crates/panda-launcher/src/ui/mod.rs → crates/panda-launcher/src/ui/state.rs
- `AccountBox()` --references--> `AppState`  [EXTRACTED]
  crates/panda-launcher/src/ui/mod.rs → crates/panda-launcher/src/ui/state.rs
- `ModsPage()` --calls--> `core()`  [INFERRED]
  crates/panda-launcher/src/ui/mods.rs → crates/panda-launcher/src/ui/state.rs

## Import Cycles
- None detected.

## Communities (29 total, 3 thin omitted)

### Community 1 - "Roadmap"
Cohesion: 0.18
Nodes (11): Later (v2+), Out of scope, Phase 0: Prerequisites (blocks login), Phase 1: Workspace and CI, Phase 2: Core: auth and vanilla launch, Phase 3: Fabric profiles, Phase 4: Modrinth mods, Phase 5: UI (zgui) (+3 more)

### Community 2 - "Architecture"
Cohesion: 0.22
Nodes (9): Architecture, Flows, Install a mod ([ADR 0007](adr/0007-modrinth-only-dependency-resolution.md)), Launch, Network etiquette, On-disk layout, Sign in ([ADR 0003](adr/0003-microsoft-auth-device-code-own-client-id.md), [0004](adr/0004-offline-play-after-verified-ownership.md)), Update the launcher ([ADR 0010](adr/0010-github-releases-updater-signed.md)) (+1 more)

### Community 3 - "Design system"
Cohesion: 0.22
Nodes (9): Accessibility, Color, Design system, Icons, Motion, Principles, Spacing and radius, States (+1 more)

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
Cohesion: 0.15
Nodes (19): fabric_json(), FabricArguments, FabricProfile, game_versions(), GameVersion, install(), installs_real_fabric_profiles(), latest_loader() (+11 more)

### Community 8 - "Panda Launcher"
Cohesion: 0.50
Nodes (3): graphify, Panda Launcher, Rules

### Community 9 - "java.rs"
Cohesion: 0.08
Nodes (30): ALL, DONE, ensure(), Entry, EntryDownloads, falls_back_to_same_major_version(), java_exe(), pick() (+22 more)

### Community 10 - "auth.rs"
Cohesion: 0.09
Nodes (27): Oauth, Claims, CLIENT_ID, client_id(), device_code(), DEVICE_GRANT, DeviceCode, Entitlements (+19 more)

### Community 11 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.50
Nodes (4): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision

### Community 12 - "account.rs"
Cohesion: 0.10
Nodes (29): Account, entry(), load(), Resume, resume(), Offline, Online, SignedOut (+21 more)

### Community 13 - "0011. Fluent i18n, Spanish and English"
Cohesion: 0.40
Nodes (4): 0011. Fluent i18n, Spanish and English, Consequences, Context, Decision

### Community 14 - "0001-rust-workspace-core-ui-split.md"
Cohesion: 0.18
Nodes (8): 0001. Cargo workspace with a UI-free core, Consequences, Context, Decision, 0012. Astro website on GitHub Pages, Consequences, Context, Decision

### Community 15 - "0003. Microsoft auth with device code and our own Azure client ID"
Cohesion: 0.50
Nodes (4): 0003. Microsoft auth with device code and our own Azure client ID, Consequences, Context, Decision

### Community 16 - "version.rs"
Cohesion: 0.07
Nodes (36): args_put_main_class_between_jvm_and_game_args(), expand(), fill(), Launch, Player, Action, Allow, Disallow (+28 more)

### Community 17 - "state.rs"
Cohesion: 0.08
Nodes (31): SearchHit, t(), tr(), InstalledRow(), Mods, ModsPage(), ResultRow(), PlayPage() (+23 more)

### Community 19 - "0004. Offline play only after verified ownership"
Cohesion: 0.50
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 20 - "mod.rs"
Cohesion: 0.13
Nodes (15): DOWNLOAD, INFO, PLAY, PUZZLE, SEARCH, SIGN_OUT, TRASH, AccountBox() (+7 more)

### Community 21 - "download.rs"
Cohesion: 0.18
Nodes (8): download_verified(), fetch(), fetch_all(), FileSpec, get_json(), hex(), json(), PARALLEL

### Community 22 - "prelude"
Cohesion: 0.17
Nodes (4): GEIST, GEIST_MONO, SHEET, AboutPage()

### Community 23 - "0007. Modrinth as the only mod source, with client-side dependency resolution"
Cohesion: 0.50
Nodes (4): 0007. Modrinth as the only mod source, with client-side dependency resolution, Consequences, Context, Decision

### Community 24 - "0008. Managed Java runtime"
Cohesion: 0.50
Nodes (4): 0008. Managed Java runtime, Consequences, Context, Decision

### Community 25 - "i18n.rs"
Cohesion: 0.19
Nodes (8): both_languages_have_the_same_keys(), bundle(), keys(), Lang, En, Es, placeholders_are_filled(), set_lang()

### Community 26 - "mods.rs"
Cohesion: 0.09
Nodes (29): profile_name(), API, Dependency, find_conflict(), get(), Hashes, incompatible_of(), install() (+21 more)

### Community 27 - "0010. Self-update from GitHub Releases, signed with ed25519"
Cohesion: 0.50
Nodes (4): 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision

## Knowledge Gaps
- **47 isolated node(s):** `panda-launcher`, `Es`, `En`, `Play`, `Mods` (+42 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 196 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Dirs` connect `mods.rs` to `version.rs`, `java.rs`, `state.rs`, `fabric.rs`?**
  _High betweenness centrality (0.086) - this node is a cross-community bridge._
- **What connects `panda-launcher`, `Es`, `En` to the rest of the system?**
  _47 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `fabric.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14855072463768115 - nodes in this community are weakly interconnected._
- **Why does `Error` connect `java.rs` to `version.rs`, `mods.rs`, `download.rs`, `fabric.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `java.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08377896613190731 - nodes in this community are weakly interconnected._
- **Why does `AuthError` connect `account.rs` to `auth.rs`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08902439024390243 - nodes in this community are weakly interconnected._