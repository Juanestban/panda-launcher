# Graph Report - panda-launcher  (2026-11-17)

## Corpus Check
- 20 files · ~4,231 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 2 file(s) not represented in the graph (top: (none) 2)

## Summary
- 100 nodes · 107 edges · 19 communities (16 shown, 3 thin omitted)
- Extraction: 100% EXTRACTED · 0% INFERRED · 0% AMBIGUOUS
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Roadmap
- Architecture
- Design system
- 0002. zgui for the UI, pinned to a git revision
- 0005. Fabric is the only mod loader
- 0006. Per-version isolated profiles
- 0011. Fluent i18n, Spanish and English
- Panda Launcher
- 0001. Cargo workspace with a UI-free core
- 0003. Microsoft auth with device code and our own Azure client ID
- 0004. Offline play only after verified ownership
- 0007. Modrinth as the only mod source, with client-side dependency resolution
- 0008. Managed Java runtime
- 0009. MSI and portable distribution (archi-games layout)
- 0010. Self-update from GitHub Releases, signed with ed25519
- 0012. Astro website on GitHub Pages
- panda-launcher

## God Nodes (most connected - your core abstractions)
1. `Roadmap` - 11 edges
2. `Design system` - 9 edges
3. `0002. zgui for the UI, pinned to a git revision` - 5 edges
4. `Architecture` - 5 edges
5. `Flows` - 5 edges
6. `0001. Cargo workspace with a UI-free core` - 4 edges
7. `0003. Microsoft auth with device code and our own Azure client ID` - 4 edges
8. `0004. Offline play only after verified ownership` - 4 edges
9. `0005. Fabric is the only mod loader` - 4 edges
10. `0006. Per-version isolated profiles` - 4 edges

## Surprising Connections (you probably didn't know these)
- None detected - all connections are within the same source files.

## Import Cycles
- None detected.

## Communities (19 total, 3 thin omitted)

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

### Community 10 - "0003. Microsoft auth with device code and our own Azure client ID"
Cohesion: 0.50
Nodes (4): 0003. Microsoft auth with device code and our own Azure client ID, Consequences, Context, Decision

### Community 11 - "0004. Offline play only after verified ownership"
Cohesion: 0.50
Nodes (4): 0004. Offline play only after verified ownership, Consequences, Context, Decision

### Community 12 - "0007. Modrinth as the only mod source, with client-side dependency resolution"
Cohesion: 0.50
Nodes (4): 0007. Modrinth as the only mod source, with client-side dependency resolution, Consequences, Context, Decision

### Community 13 - "0008. Managed Java runtime"
Cohesion: 0.50
Nodes (4): 0008. Managed Java runtime, Consequences, Context, Decision

### Community 14 - "0009. MSI and portable distribution (archi-games layout)"
Cohesion: 0.50
Nodes (4): 0009. MSI and portable distribution (archi-games layout), Consequences, Context, Decision

### Community 15 - "0010. Self-update from GitHub Releases, signed with ed25519"
Cohesion: 0.50
Nodes (4): 0010. Self-update from GitHub Releases, signed with ed25519, Consequences, Context, Decision

### Community 16 - "0012. Astro website on GitHub Pages"
Cohesion: 0.50
Nodes (4): 0012. Astro website on GitHub Pages, Consequences, Context, Decision

## Knowledge Gaps
- **1 isolated node(s):** `panda-launcher`
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 68 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Roadmap` connect `Roadmap` to `ROADMAP.md`?**
  _High betweenness centrality (0.178) - this node is a cross-community bridge._
- **What connects `panda-launcher` to the rest of the system?**
  _1 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Why does `Design system` connect `Design system` to `ROADMAP.md`?**
  _High betweenness centrality (0.144) - this node is a cross-community bridge._
- **Why does `Architecture` connect `Architecture` to `ROADMAP.md`?**
  _High betweenness centrality (0.142) - this node is a cross-community bridge._