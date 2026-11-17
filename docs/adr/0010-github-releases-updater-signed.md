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

## Consequences
- No updater framework; about 300 lines we own.
- Losing the private key means shipping a release that rotates the public key. A leaked key needs the same response.
