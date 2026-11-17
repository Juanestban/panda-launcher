# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 32 files · ~10,806 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 4, .toml 1)

## Summary
- 363 nodes · 654 edges · 28 communities (26 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `edec3403`
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
- Error
- auth.rs
- 0009. MSI and portable distribution (archi-games layout)
- account.rs
- 0011. Fluent i18n, Spanish and English
- 0001. Cargo workspace with a UI-free core
- 0003. Microsoft auth with device code and our own Azure client ID
- version.rs
- net.rs
- panda-core
- 0004. Offline play only after verified ownership
- Dirs
- download.rs
- java.rs
- 0007. Modrinth as the only mod source, with client-side dependency resolution
- 0008. Managed Java runtime
- 0012. Astro website on GitHub Pages
- mods.rs
- 0010. Self-update from GitHub Releases, signed with ed25519

## God Nodes (most connected - your core abstractions)
1. `Dirs` - 32 edges
2. `AuthError` - 23 edges
3. `Version` - 17 edges
4. `install()` - 11 edges
5. `Roadmap` - 11 edges
6. `install()` - 10 edges
7. `install()` - 10 edges
8. `resume()` - 9 edges
9. `sign_in()` - 9 edges
10. `Arg` - 9 edges

## Surprising Connections (you probably didn't know these)
- `expand()` --calls--> `allowed()`  [INFERRED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs
- `player()` --references--> `Profile`  [EXTRACTED]
  crates/panda-launcher/src/main.rs → crates/panda-core/src/auth.rs
- `expand()` --references--> `Item`  [EXTRACTED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/auth.rs
- `mods_dir()` --calls--> `profile_name()`  [INFERRED]
  crates/panda-core/src/mods.rs → crates/panda-core/src/fabric.rs
- `record_path()` --calls--> `profile_name()`  [INFERRED]
  crates/panda-core/src/mods.rs → crates/panda-core/src/fabric.rs

## Import Cycles
- None detected.

## Communities (28 total, 2 thin omitted)

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

### Community 9 - "Error"
Cohesion: 0.12
Nodes (15): Error, Checksum, Http, IncompatibleMods, Io, Json, ModNotInstalled, ModStillRequired (+7 more)

### Community 10 - "auth.rs"
Cohesion: 0.11
Nodes (26): Oauth, Claims, CLIENT_ID, client_id(), device_code(), DEVICE_GRANT, DeviceCode, Entitlements (+18 more)

### Community 11 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.50
Nodes (4): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision

### Community 12 - "account.rs"
Cohesion: 0.10
Nodes (29): Account, entry(), load(), Resume, resume(), Offline, Online, SignedOut (+21 more)

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
Cohesion: 0.09
Nodes (31): Action, Allow, Disallow, allowed(), Arg, Plain, Ruled, Arguments (+23 more)

### Community 17 - "net.rs"
Cohesion: 0.19
Nodes (5): USER_AGENT, main(), mods_command(), player(), run()

### Community 19 - "0004. Offline play only after verified ownership"
Cohesion: 0.50
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 20 - "Dirs"
Cohesion: 0.19
Nodes (5): args_put_main_class_between_jvm_and_game_args(), expand(), fill(), Launch, Player

### Community 21 - "download.rs"
Cohesion: 0.18
Nodes (8): download_verified(), fetch(), fetch_all(), FileSpec, get_json(), hex(), json(), PARALLEL

### Community 22 - "java.rs"
Cohesion: 0.22
Nodes (15): ALL, DONE, ensure(), Entry, EntryDownloads, falls_back_to_same_major_version(), java_exe(), pick() (+7 more)

### Community 23 - "0007. Modrinth as the only mod source, with client-side dependency resolution"
Cohesion: 0.50
Nodes (4): 0007. Modrinth as the only mod source, with client-side dependency resolution, Consequences, Context, Decision

### Community 24 - "0008. Managed Java runtime"
Cohesion: 0.50
Nodes (4): 0008. Managed Java runtime, Consequences, Context, Decision

### Community 25 - "0012. Astro website on GitHub Pages"
Cohesion: 0.50
Nodes (4): 0012. Astro website on GitHub Pages, Consequences, Context, Decision

### Community 26 - "mods.rs"
Cohesion: 0.09
Nodes (30): profile_name(), API, Dependency, find_conflict(), get(), Hashes, incompatible_of(), install() (+22 more)

### Community 27 - "0010. Self-update from GitHub Releases, signed with ed25519"
Cohesion: 0.50
Nodes (4): 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision

## Knowledge Gaps
- **35 isolated node(s):** `panda-core`, `SignedOut`, `NoClientId`, `Expired`, `Declined` (+30 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 161 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Dirs` connect `mods.rs` to `version.rs`, `Dirs`, `java.rs`, `fabric.rs`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **What connects `panda-core`, `SignedOut`, `NoClientId` to the rest of the system?**
  _35 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `fabric.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14855072463768115 - nodes in this community are weakly interconnected._
- **Why does `Error` connect `Error` to `fabric.rs`, `version.rs`, `download.rs`, `java.rs`, `mods.rs`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._
- **Should `Error` be split into smaller, more focused modules?**
  _Cohesion score 0.11764705882352941 - nodes in this community are weakly interconnected._
- **Why does `AuthError` connect `account.rs` to `auth.rs`?**
  _High betweenness centrality (0.057) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11260504201680673 - nodes in this community are weakly interconnected._