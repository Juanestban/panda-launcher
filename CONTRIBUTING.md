# Contributing to Panda Launcher

Thanks for helping. This page covers how to set up, what CI checks, and the rules every change keeps.

## Before you start

- **Bugs:** open an issue with what you did, what you expected, what happened, your OS and the launcher version
  (shown in **About**). Never paste tokens, your email or log lines that contain them.
- **Features:** open an issue first. Some ideas are out of scope on purpose (see [Rules](#rules)), and a short
  discussion saves a pull request that can't be merged.
- **Small fixes** (typos, docs, obvious bugs): send a pull request directly.

## Setup

Follow [Getting started](README.md#getting-started-from-source) in the README. You need Rust stable (edition 2024)
and, on Windows, the Visual Studio Build Tools. A local build runs without any keys; Microsoft sign in needs
`PANDA_MSA_CLIENT_ID` at build time, from your own Azure app.

## Making a change

1. Fork, then branch from `main` (`fix/mod-search-empty-state`, `feat/update-installed-mods`).
2. Keep the change focused: one fix or feature per pull request.
3. Run what CI runs, and make it pass:

   ```
   cargo fmt --all --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```

   Network tests are ignored by default; run them when you touch downloads, Fabric or Modrinth:
   `cargo test --workspace -- --ignored`.
4. For UI changes, run the app (`cargo run -p panda-launcher`) and attach a screenshot to the pull request.
   Check both languages.
5. For website changes: `cd web && npm install && npm run build`.

## Rules

These come from the project's decisions in [docs/adr/](docs/adr/). A pull request that breaks one won't be merged.

- **English in the repo.** Code, comments, docs, ADRs and commit messages are in English. The app ships in
  Spanish and English: add every new text to both `crates/panda-launcher/locales/es.ftl` and `en.ftl`.
- **Legal only.** No cracked or offline account mode. Offline play is only for an account verified online before.
  Never use another launcher's Azure client ID.
- **Privacy.** Never put an email or personal data in a User-Agent, header, URL or payload. The User-Agent is
  `panda-launcher/<version> (+https://github.com/juanestban/panda-launcher)`.
- **Secrets.** Tokens go to the OS keychain (`keyring`), never to files or logs.
- **Pure Rust.** Everything that ships is Rust: no Electron, webviews or other runtimes. `web/` is the only non-Rust
  code. Prefer crates already in `Cargo.toml`, and explain any new dependency in the pull request.
- **Fabric and Modrinth only.** No Forge, NeoForge, Quilt or CurseForge.
- **Core without UI.** `panda-core` (auth, downloads, mods, launch, updates) must not depend on the UI crate.
- **Design.** UI follows [docs/design-system.md](docs/design-system.md): black and red, Geist, Phosphor icons,
  named text styles, the spacing scale, and every state (hover, focus, disabled, loading, empty, error).
- **Architecture decisions** get an ADR: a new `docs/adr/NNNN-title.md` in the existing format.

## Commits and pull requests

- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  `feat(ui): …`, `fix(core): …`, `docs: …`, `chore: …`.
- The pull request says what changed and why, how you tested it, and links the issue.
- If your change finishes a roadmap item, tick it in [docs/ROADMAP.md](docs/ROADMAP.md).

## Code style

- `rustfmt.toml` sets two spaces and a 150 column width; `cargo fmt` applies it.
- Prefer the smallest change that solves the problem. No speculative options or abstractions.
- Comments explain *why*, not *what*. New logic with branches or parsing gets a test.

## License

By contributing you agree that your work is published under MIT OR Apache-2.0, like the rest of the project.
