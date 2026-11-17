# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 31 files · ~9,336 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 4, .toml 1)

## Summary
- 320 nodes · 547 edges · 26 communities (24 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6ee8ad36`
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

## God Nodes (most connected - your core abstractions)
1. `Dirs` - 24 edges
2. `AuthError` - 23 edges
3. `Version` - 17 edges
4. `Roadmap` - 11 edges
5. `install()` - 10 edges
6. `install()` - 10 edges
7. `resume()` - 9 edges
8. `sign_in()` - 9 edges
9. `Arg` - 9 edges
10. `Rule` - 9 edges

## Surprising Connections (you probably didn't know these)
- `expand()` --calls--> `allowed()`  [INFERRED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs
- `player()` --references--> `Profile`  [EXTRACTED]
  crates/panda-launcher/src/main.rs → crates/panda-core/src/auth.rs
- `expand()` --references--> `Item`  [EXTRACTED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/auth.rs
- `fabric_json()` --references--> `Dirs`  [EXTRACTED]
  crates/panda-core/src/fabric.rs → crates/panda-core/src/paths.rs
- `merge()` --references--> `Version`  [EXTRACTED]
  crates/panda-core/src/fabric.rs → crates/panda-core/src/version.rs

## Import Cycles
- None detected.

## Communities (26 total, 2 thin omitted)

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
Nodes (20): fabric_json(), FabricArguments, FabricProfile, game_versions(), GameVersion, install(), installs_real_fabric_profiles(), latest_loader() (+12 more)

### Community 8 - "Panda Launcher"
Cohesion: 0.50
Nodes (3): graphify, Panda Launcher, Rules

### Community 9 - "Error"
Cohesion: 0.17
Nodes (10): Error, Checksum, Http, Io, Json, NoFabric, NoJavaRuntime, UnknownVersion (+2 more)

### Community 10 - "auth.rs"
Cohesion: 0.11
Nodes (26): Oauth, Claims, CLIENT_ID, client_id(), device_code(), DEVICE_GRANT, DeviceCode, Entitlements (+18 more)

### Community 11 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.20
Nodes (8): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision, 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision

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
Cohesion: 0.20
Nodes (4): USER_AGENT, main(), player(), run()

### Community 19 - "0004. Offline play only after verified ownership"
Cohesion: 0.50
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 20 - "Dirs"
Cohesion: 0.11
Nodes (6): args_put_main_class_between_jvm_and_game_args(), expand(), fill(), Launch, Player, Dirs

### Community 21 - "download.rs"
Cohesion: 0.16
Nodes (7): fetch(), fetch_all(), FileSpec, get_json(), hex(), json(), PARALLEL

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

## Knowledge Gaps
- **30 isolated node(s):** `panda-core`, `SignedOut`, `NoClientId`, `Expired`, `Declined` (+25 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 148 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AuthError` connect `account.rs` to `auth.rs`?**
  _High betweenness centrality (0.061) - this node is a cross-community bridge._
- **What connects `panda-core`, `SignedOut`, `NoClientId` to the rest of the system?**
  _30 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `fabric.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14666666666666667 - nodes in this community are weakly interconnected._
- **Why does `Dirs` connect `Dirs` to `version.rs`, `download.rs`, `java.rs`, `fabric.rs`?**
  _High betweenness centrality (0.061) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11260504201680673 - nodes in this community are weakly interconnected._
- **Why does `Error` connect `Error` to `version.rs`, `download.rs`, `java.rs`, `fabric.rs`?**
  _High betweenness centrality (0.048) - this node is a cross-community bridge._
- **Should `account.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09982174688057041 - nodes in this community are weakly interconnected._