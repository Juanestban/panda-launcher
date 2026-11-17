# 0010. Self-update from GitHub Releases, signed with ed25519

Date: 2026-11-17 · Status: Accepted

## Context
The launcher must find new releases in `juanestban/panda-launcher` and update itself with one click, for both MSI and portable installs. archi-games has a working hand-written updater (`crates/app/src/update.rs`), but it only checks SHA-256, which does not protect against someone who takes over the GitHub account.

## Decision
Reuse the archi-games design in `panda-core::update`, and add a signature check:
- `GET https://api.github.com/repos/juanestban/panda-launcher/releases/latest`, unauthenticated; a 404 means no release.
- `Version(u64,u64,u64)` compared to `CARGO_PKG_VERSION`; pre-releases are ignored.
- Offer the update only if the matching asset, `SHA256SUMS.txt` and `<asset>.sig` all exist.
- Check 3 s after start, then hourly, retrying after 5 min on failure. Automatic checks can be turned off in Settings, with a manual "Check for updates" in About. Automatic failures stay silent.
- **Never silent:** a banner offers Update and What's new; the download starts only on click, with progress.
- Verify the SHA-256 **and** the ed25519 signature (`ed25519-dalek`, public key compiled in, private key in the `PANDA_UPDATE_SIGNING_KEY` CI secret). Any mismatch deletes the file and shows an integrity error.
- Apply: MSI → `msiexec /i <msi> /passive LAUNCHAPP=1` and quit; portable → rename to `.exe.old`, move `.exe.new` into place, spawn with `--after-update`, quit; delete `.exe.old` on the next start.
- A single-instance lock retries for ~15 s on `--after-update` while the old process exits.

### Implementation notes (Phase 6)
- The `.sig` is a raw 64-byte ed25519 signature over the installer's **SHA-256 digest** (not the whole file). The signature is checked against the hash listed in `SHA256SUMS.txt` before downloading, then the download must match that hash, so the file is authenticated without reading it twice.
- The public key is injected at build time from `PANDA_UPDATE_PUBLIC_KEY` (a repo variable), like the MSA client ID. Builds without it, and non-Windows builds, have updates turned off and say so in About.
- Signing is a pure-Rust example, `cargo run -p panda-core --example sign`, so CI needs no extra tools. The release job fails when the public key doesn't match the signing key.
- The lock is `launcher.lock` in the data root, held with `std::fs::File::try_lock`. A second start exits quietly instead of focusing the open window (shortcut: no IPC yet).

## Consequences
- No updater framework; about 300 lines we own.
- Losing the private key means shipping a release that rotates the public key. A leaked key needs the same response.
