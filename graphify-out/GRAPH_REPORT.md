# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 25 files · ~5,819 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 4 file(s) not represented in the graph (top: (none) 4)

## Summary
- 181 nodes · 257 edges · 20 communities (18 shown, 2 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `41b42dc2`
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
- 0011. Fluent i18n, Spanish and English
- Panda Launcher
- 0001. Cargo workspace with a UI-free core
- auth.rs
- 0004. Offline play only after verified ownership
- account.rs
- 0008. Managed Java runtime
- 0009. MSI and portable distribution (archi-games layout)
- 0010. Self-update from GitHub Releases, signed with ed25519
- AuthError
- net.rs
- panda-core

## God Nodes (most connected - your core abstractions)
1. `AuthError` - 24 edges
2. `Roadmap` - 11 edges
3. `resume()` - 9 edges
4. `sign_in()` - 9 edges
5. `Design system` - 9 edges
6. `Resume` - 7 edges
7. `save()` - 6 edges
8. `Session` - 6 edges
9. `token()` - 6 edges
10. `oauth()` - 6 edges

## Surprising Connections (you probably didn't know these)
- `entry()` --references--> `AuthError`  [EXTRACTED]
  crates/panda-core/src/account.rs → crates/panda-core/src/auth.rs
- `load()` --references--> `AuthError`  [EXTRACTED]
  crates/panda-core/src/account.rs → crates/panda-core/src/auth.rs
- `save()` --references--> `AuthError`  [EXTRACTED]
  crates/panda-core/src/account.rs → crates/panda-core/src/auth.rs
- `sign_out()` --references--> `AuthError`  [EXTRACTED]
  crates/panda-core/src/account.rs → crates/panda-core/src/auth.rs
- `resume()` --references--> `AuthError`  [EXTRACTED]
  crates/panda-core/src/account.rs → crates/panda-core/src/auth.rs

## Import Cycles
- None detected.

## Communities (20 total, 2 thin omitted)

### Community 0 - "ROADMAP.md"
Cohesion: 0.12
Nodes (12): 0003. Microsoft auth with device code and our own Azure client ID, Consequences, Context, Decision, 0007. Modrinth as the only mod source, with client-side dependency resolution, Consequences, Context, Decision (+4 more)

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

### Community 7 - "0011. Fluent i18n, Spanish and English"
Cohesion: 0.40
Nodes (4): 0011. Fluent i18n, Spanish and English, Consequences, Context, Decision

### Community 8 - "Panda Launcher"
Cohesion: 0.50
Nodes (3): graphify, Panda Launcher, Rules

### Community 9 - "0001. Cargo workspace with a UI-free core"
Cohesion: 0.50
Nodes (4): 0001. Cargo workspace with a UI-free core, Consequences, Context, Decision

### Community 10 - "auth.rs"
Cohesion: 0.10
Nodes (26): Oauth, Claims, CLIENT_ID, client_id(), device_code(), DEVICE_GRANT, DeviceCode, Entitlements (+18 more)

### Community 11 - "0004. Offline play only after verified ownership"
Cohesion: 0.50
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 12 - "account.rs"
Cohesion: 0.19
Nodes (14): Account, entry(), load(), Resume, resume(), Offline, Online, SignedOut (+6 more)

### Community 13 - "0008. Managed Java runtime"
Cohesion: 0.50
Nodes (4): 0008. Managed Java runtime, Consequences, Context, Decision

### Community 14 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.50
Nodes (4): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision

### Community 15 - "0010. Self-update from GitHub Releases, signed with ed25519"
Cohesion: 0.50
Nodes (4): 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision

### Community 16 - "AuthError"
Cohesion: 0.13
Nodes (15): AuthError, AdultVerification, AppNotApproved, ChildAccount, Declined, Expired, Http, Keychain (+7 more)

### Community 17 - "net.rs"
Cohesion: 0.29
Nodes (3): USER_AGENT, main(), run()

## Knowledge Gaps
- **15 isolated node(s):** `panda-core`, `SignedOut`, `NoClientId`, `Expired`, `Declined` (+10 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 102 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `AuthError` connect `AuthError` to `net.rs`, `auth.rs`, `account.rs`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._
- **What connects `panda-core`, `SignedOut`, `NoClientId` to the rest of the system?**
  _15 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `ROADMAP.md` be split into smaller, more focused modules?**
  _Cohesion score 0.12318840579710146 - nodes in this community are weakly interconnected._
- **Why does `Roadmap` connect `Roadmap` to `ROADMAP.md`?**
  _High betweenness centrality (0.054) - this node is a cross-community bridge._
- **Should `auth.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1036036036036036 - nodes in this community are weakly interconnected._
- **Why does `Design system` connect `Design system` to `ROADMAP.md`?**
  _High betweenness centrality (0.043) - this node is a cross-community bridge._
- **Should `AuthError` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._