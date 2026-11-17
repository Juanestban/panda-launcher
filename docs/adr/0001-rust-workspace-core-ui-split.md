# 0001. Cargo workspace with a UI-free core

Date: 2026-11-17 · Status: Accepted

## Context
The UI framework (zgui, [0002](0002-zgui-pinned-ui.md)) is young and may break or need replacing. Auth, downloads, mods and updates must be testable without a window.

## Decision
A Cargo workspace (resolver 3, edition 2024) with:
- `crates/panda-core`: all logic, no UI dependency, async API plus progress events.
- `crates/panda-launcher`: the binary, UI and locales only.
- `web/`: the Astro site, outside Cargo ([0012](0012-astro-website-github-pages.md)).

## Consequences
- Replacing zgui touches one crate.
- Core logic is unit-testable in CI.
- One extra crate boundary to maintain. Shared types live in `panda-core`.
