# Releasing

GitHub Actions (`.github/workflows/release.yml`) builds and publishes a release when a `vX.Y.Z` tag is pushed.
Installed launchers find it and offer to update ([ADR 0010](docs/adr/0010-github-releases-updater-signed.md)).

## Setup

Nothing is required: with no secrets the release builds, publishes, and installed copies update from it,
checking each download against `SHA256SUMS.txt`. These settings (**Settings → Secrets and variables → Actions**) are optional:

| Name | Kind | Without it | Value |
|---|---|---|---|
| `PANDA_MSA_CLIENT_ID` | secret | The build can't sign in with Microsoft. | The Azure app's client ID ([ADR 0003](docs/adr/0003-microsoft-auth-device-code-own-client-id.md)). It is not secret, so it may also be a variable. |
| `PANDA_UPDATE_SIGNING_KEY` | secret | No `.sig` files. | 64 hex characters from `openssl rand -hex 32`. Keep a copy in a password manager. |
| `PANDA_UPDATE_PUBLIC_KEY` | variable | Updates are checked by SHA-256 only. | The output of `PANDA_UPDATE_SIGNING_KEY=<key> cargo run -p panda-core --example sign -- --public`. |

Signing is all or nothing: set both update keys, or neither. When the public key is set the job checks it matches the
signing key, because copies built with a key refuse unsigned updates. Turning signing on later is fine; turning it off
later strands the copies built with the key (they must download one release by hand). Never commit the signing key.

## Steps

1. Bump `version` in the root `Cargo.toml` (`[workspace.package]`) following [SemVer](https://semver.org/).
2. Refresh `Cargo.lock` and check everything passes:
   ```
   cargo test --workspace
   cargo clippy --workspace --all-targets -- -D warnings
   ```
3. Commit the version (for example `chore: release 0.2.0`) and push it.
4. Tag and push:
   ```
   git tag v0.2.0
   git push origin v0.2.0
   ```
5. In the **Actions** tab the *Release* workflow checks the tag against `Cargo.toml`, runs the tests, packages, signs (when the keys are set) and publishes the release with notes generated from the commits.

## What it publishes

| File | Purpose |
|---|---|
| `PandaLauncher-<version>-x64.msi` | Per-user installer (no admin). Installed copies update with this. |
| `PandaLauncher-<version>-x64-portable.exe` | Standalone exe. Portable copies update with this. |
| `SHA256SUMS.txt` | SHA-256 of both files. |
| `<file>.sig` | Only with the signing key: ed25519 signature over that file's SHA-256 digest. |

**Don't rename these files**: the updater finds them by name and skips releases missing the installer or `SHA256SUMS.txt` (or the `.sig`, for signed builds).

## Packaging locally (Windows)

```
dotnet tool restore
.\scripts\package.ps1      # -> target\dist
```

## If something goes wrong

- **Tag doesn't match the version:** delete the tag (`git tag -d v0.2.0` and `git push origin :refs/tags/v0.2.0`), fix `Cargo.toml` and tag again.
- **A broken release:** publish a new version with the fix. Deleting a release doesn't roll back anyone who installed it.
- **Lost or leaked signing key:** generate a new pair, update both settings, and publish a release. Copies older than it can't verify the new signatures, so their users must download that release once by hand.
