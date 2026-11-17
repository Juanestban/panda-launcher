# 0007. Modrinth as the only mod source, with client-side dependency resolution

Date: 2026-11-17 · Status: Accepted

## Context
The core feature is one-click install of the right mod for the right version. CurseForge needs an approved API key and many authors block third-party downloads. Modrinth's API v2 is open, filters by game version and loader, and returns hashes and dependencies. It has no server-side recursive resolver.

## Decision
- Search with facets `versions:<v>`, `categories:fabric`, `project_type:mod`.
- Pick the newest version from `/project/<id>/version?loaders=["fabric"]&game_versions=["<v>"]`, primary file.
- Walk `required` dependencies recursively (use the pinned `version_id` when given). Abort, naming the conflict, if an `incompatible` mod is installed. Ignore `optional` and `embedded` dependencies.
- Verify SHA-512, write into the profile's `mods/`, and record the install in `panda-mods.json` (`project_id`, `version_id`, `file`, `sha512`, `required_by`).
- Uninstall warns when `required_by` is not empty.
- User-Agent `panda-launcher/<version> (+repo URL)` with no personal data; respect `X-Ratelimit-*` (300 req/min).
- Updating installed mods is out of v1.

## Consequences
- No API key or partner agreement needed.
- Mods published only on CurseForge are unavailable.
- `panda-mods.json` lets later features (updates, `.mrpack` export) reuse the record.
