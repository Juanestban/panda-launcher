# Panda Launcher

Minecraft Java Edition launcher written in pure Rust. Microsoft account login with ownership check, Fabric-only per-version profiles, Modrinth mod installs, self-updating MSI and portable builds from GitHub Releases. Design decisions live in `docs/adr/`, plan in `docs/ROADMAP.md`.

## Rules

- **Language:** all repo content (code, comments, docs, ADRs, commit messages) is in English. Chat with the user is in Spanish. The launcher UI ships in Spanish and English.
- **Privacy:** never put the user's email address or any personal data in a User-Agent, header, URL or payload sent to any external service (Modrinth, GitHub, Mojang, Microsoft, etc.). User-Agent format: `panda-launcher/<version> (+https://github.com/<repo>)`, repo URL only, no contact email.
- **Pure Rust:** all shipped code is Rust; no non-Rust runtimes or frameworks (no Velopack, Electron, webviews). WiX 5 (`dotnet tool`) is allowed only for packaging the MSI in CI. The `web/` Astro site is the only non-Rust code. Prefer crates already in `Cargo.toml`; justify any new dependency.
- **Legal:** never reuse another launcher's Azure client ID; ours is injected at build time via `PANDA_MSA_CLIENT_ID`. Never add a cracked/offline-account mode. Offline play is allowed only for an account whose ownership was verified online before.
- **Mod loader:** Fabric only. Mod source: Modrinth only.
- **Secrets:** tokens go to the OS keychain (`keyring`), never to plain files or logs.
- **UI:** zgui (pinned git rev), dark minimalist theme defined in `docs/design-system.md`. Core logic (auth, downloads, mods, launch) must not depend on the UI crate.
- **ADRs:** any architectural decision gets an MADR-style file in `docs/adr/NNNN-title.md`.
- **Graph:** after any change to code or docs, run `graphify update .` (code) or `/graphify --update` (docs) so `graphify-out/` stays current.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

Rules:
- When graphify-out/graph.json exists, run `graphify query "<question>"` before your first grep, search or source-file read for a codebase question, and put this rule in every subagent prompt that explores code. Read raw files only for lines the graph or an earlier read already located. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
- graphify-out/ is local and git-ignored (the post-commit hook rebuilds it after every commit). Never commit it; on a fresh clone run `graphify update .` to create it.
