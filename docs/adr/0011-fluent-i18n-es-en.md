# 0011. Fluent i18n, Spanish and English

Date: 2026-11-17 · Status: Accepted

## Context
v1 ships in Spanish and English. archi-games already uses Fluent (`locales/en.ftl`), and Fluent handles plurals and variables properly.

## Decision
`crates/panda-launcher/locales/{es,en}.ftl` loaded with the `fluent` crates. Default to the OS locale (Spanish if it starts with `es`, English otherwise), with a selector in Settings that is saved to `settings.json` (serde_json is already a dependency; no TOML crate needed). Repo docs and code stay in English.

## Consequences
- Same tooling as archi-games.
- Every user-facing string needs both files; CI can check that the key sets match.
