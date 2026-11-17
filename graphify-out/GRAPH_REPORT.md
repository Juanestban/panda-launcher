# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 30 files · ~8,522 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 5 file(s) not represented in the graph (top: (none) 4, .toml 1)

## Summary
- 297 nodes · 484 edges · 17 communities (15 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS · INFERRED: 1 edges (avg confidence: 0.85)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6ee8ad36`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ROADMAP.md
- Roadmap
- Architecture
- Design system
- 0002. zgui for the UI, pinned to a git revision
- 0005. Fabric is the only mod loader
- 0006. Per-version isolated profiles
- Panda Launcher
- auth.rs
- account.rs
- version.rs
- net.rs
- panda-core
- Dirs
- download.rs
- java.rs

## God Nodes (most connected - your core abstractions)
1. `AuthError` - 23 edges
2. `Dirs` - 20 edges
3. `Version` - 14 edges
4. `Roadmap` - 11 edges
5. `install()` - 10 edges
6. `Rule` - 9 edges
7. `sign_in()` - 9 edges
8. `resume()` - 9 edges
9. `Design system` - 9 edges
10. `Arg` - 8 edges

## Surprising Connections (you probably didn't know these)
- `expand()` --calls--> `allowed()`  [INFERRED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs
- `ensure()` --references--> `Dirs`  [EXTRACTED]
  crates/panda-core/src/java.rs → crates/panda-core/src/paths.rs
- `Launch` --references--> `Version`  [EXTRACTED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs
- `expand()` --references--> `Item`  [EXTRACTED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/auth.rs
- `expand()` --references--> `Arg`  [EXTRACTED]
  crates/panda-core/src/launch.rs → crates/panda-core/src/version.rs

## Import Cycles
- None detected.

## Communities (17 total, 2 thin omitted)

### Community 0 - "ROADMAP.md"
Cohesion: 0.05
Nodes (36): 0001. Cargo workspace with a UI-free core, Consequences, Context, Decision, 0003. Microsoft auth with device code and our own Azure client ID, Consequences, Context, Decision (+28 more)

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

### Community 8 - "Panda Launcher"
Cohesion: 0.50
Nodes (3): graphify, Panda Launcher, Rules

### Community 10 - "auth.rs"
Cohesion: 0.11
Nodes (26): Oauth, Claims, CLIENT_ID, client_id(), device_code(), DEVICE_GRANT, DeviceCode, Entitlements (+18 more)

### Community 12 - "account.rs"
Cohesion: 0.09
Nodes (28): Account, entry(), load(), Resume, resume(), Offline, Online, SignedOut (+20 more)

### Community 16 - "version.rs"
Cohesion: 0.09
Nodes (31): Action, Allow, Disallow, allowed(), Arg, Plain, Ruled, Arguments (+23 more)

### Community 17 - "net.rs"
Cohesion: 0.18
Nodes (5): Profile, USER_AGENT, main(), player(), run()

### Community 20 - "Dirs"
Cohesion: 0.11
Nodes (6): args_put_main_class_between_jvm_and_game_args(), expand(), fill(), Launch, Player, Dirs

### Community 21 - "download.rs"
Cohesion: 0.10
Nodes (16): fetch(), fetch_all(), FileSpec, get_json(), hex(), json(), PARALLEL, Error (+8 more)

### Community 22 - "java.rs"
Cohesion: 0.19
Nodes (15): ALL, DONE, ensure(), Entry, EntryDownloads, falls_back_to_same_major_version(), java_exe(), pick() (+7 more)

## Knowledge Gaps
- **29 isolated node(s):** `panda-core`, `Http`, `Io`, `Json`, `Checksum` (+24 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 143 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AuthError` connect `account.rs` to `auth.rs`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **What connects `panda-core`, `Http`, `Io` to the rest of the system?**
  _29 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ROADMAP.md` be split into smaller, more focused modules?**
  _Cohesion score 0.050170068027210885 - nodes in this community are weakly interconnected._
- **Why does `Dirs` connect `Dirs` to `version.rs`, `java.rs`?**
  _High betweenness centrality (0.052) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11260504201680673 - nodes in this community are weakly interconnected._
- **Why does `Error` connect `download.rs` to `version.rs`, `java.rs`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Should `account.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09243697478991597 - nodes in this community are weakly interconnected._