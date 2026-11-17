# 0012. Astro website on GitHub Pages

Date: 2026-11-17 · Status: Accepted

## Context
The launcher needs a public page to present and download it, similar to [Horizon's site](https://playhorizon.dev). This is the phase after launcher v1.

## Decision
- `web/` is an Astro project in the same repo (not a Cargo crate), built and deployed to GitHub Pages by an Actions workflow on pushes to `main` that touch `web/`.
- Astro i18n routes `/es/` and `/en/`.
- Tabs: **Home** (hero, screenshots, download), **Download** (MSI + portable with version, size, checksums, install steps), **Releases** (changelog from GitHub Releases at build time), **Docs** (guide + FAQ), **Roadmap** (rendered from `docs/ROADMAP.md`).
- The download version is baked in at build time; the release workflow triggers a redeploy, so visitors never hit the GitHub API rate limit.
- Styled with the shared tokens from [design-system.md](../design-system.md).
- Footer disclaimer: "Not an official Minecraft product. Not approved by or associated with Mojang or Microsoft." (also in the launcher's About).

## Consequences
- Free hosting in the same repo; a custom domain can be added later.
- The Astro site is the only non-Rust code in the repo.
