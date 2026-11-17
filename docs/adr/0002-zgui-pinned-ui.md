# 0002. zgui for the UI, pinned to a git revision

Date: 2026-11-17 · Status: Accepted

## Context
The owner chose [zgui](https://github.com/zortax/zgui): a retained, reactive Rust UI with real CSS (Stylo), wgpu + Vello rendering, a shadcn-style widget set (`zgui-ui`) and async support (`background`, `zgui-tokio`). As of 2026-11 it is about 3.5 months old, at 0.1.0, has mostly one maintainer and warns about breaking changes.

## Decision
Use zgui. Depend on a pinned git `rev` (not `^0.1`) and upgrade on purpose. Style it through CSS variables from [design-system.md](../design-system.md).

## Consequences
- CSS-based theming lets the launcher and the website share tokens.
- Breaking upstream changes are absorbed only when we bump the rev.
- If zgui stalls, [0001](0001-rust-workspace-core-ui-split.md) limits the rewrite to `panda-launcher`.
- Risk to verify early: image support (skins, screenshots) beyond `AvatarImage`/`zgui-image`.

## Alternatives
- egui (used in archi-games): mature, but immediate mode and harder to theme to the design system.
- gpui (Horizon's launcher): powerful but tied to Zed's fork.
