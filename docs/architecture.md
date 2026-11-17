# Architecture

## Workspace

```
panda-launcher/
  Cargo.toml              [workspace], resolver 3, edition 2024
  crates/
    panda-core/           all logic; no UI dependency
      auth/               Microsoft → Xbox → XSTS → Minecraft, ownership
      net/                shared reqwest client, User-Agent, verified downloads
      meta/               Mojang version manifest, Fabric meta, Java runtimes
      profile/            per-version directories, launch arguments, process spawn
      mods/               Modrinth search/install/uninstall, panda-mods.json
      update/             GitHub Releases check, download, verify, apply
    panda-launcher/       binary: zgui UI, Fluent locales, wiring to core
      locales/{es,en}.ftl
  wix/main.wxs            MSI definition
  scripts/package.ps1     builds MSI + portable + SHA256SUMS + .sig
  web/                    Astro site (Phase 7)
  docs/
```

`panda-core` exposes async functions and progress events. `panda-launcher` runs them on a tokio worker (`zgui-tokio`/`background`) and renders the state. Swapping the UI framework touches only `panda-launcher` ([ADR 0001](adr/0001-rust-workspace-core-ui-split.md)).

## On-disk layout

Install (MSI, per-user): `%LOCALAPPDATA%\Programs\Panda Launcher\panda-launcher.exe`.
The portable exe can live anywhere.

Data (both builds): `%LOCALAPPDATA%\PandaLauncher\`

```
PandaLauncher/
  settings.toml           language, memory, auto-update toggle
  logs/
  shared/
    versions/<id>/        version JSON + client jar
    libraries/
    assets/{indexes,objects}/
    natives/<id>/
    java/<component>/     managed runtimes
  profiles/
    1.21.10-fabric/       passed as --gameDir
      mods/               only this version's mods
      panda-mods.json     installed-mod record
      config/ saves/ resourcepacks/ options.txt
```

Secrets (refresh token, cached ownership) live in the OS keychain, never under this folder.

## Flows

### Sign in ([ADR 0003](adr/0003-microsoft-auth-device-code-own-client-id.md), [0004](adr/0004-offline-play-after-verified-ownership.md))

1. `POST login.microsoftonline.com/consumers/oauth2/v2.0/devicecode`, scope `XboxLive.signin offline_access`. Show `user_code` and the `microsoft.com/link` URL.
2. Poll `/token` (handle `authorization_pending`, `slow_down`, `expired_token`).
3. `POST user.auth.xboxlive.com/user/authenticate` with `RpsTicket=d=<access_token>`.
4. `POST xsts.auth.xboxlive.com/xsts/authorize`, RelyingParty `rp://api.minecraftservices.com/`. Map `XErr` codes to messages.
5. `POST api.minecraftservices.com/authentication/login_with_xbox` → Minecraft access token (~24 h).
6. `GET /entitlements/mcstore` must contain `game_minecraft` or `product_minecraft`. Otherwise Play stays disabled.
7. `GET /minecraft/profile` → name + UUID. A 404 means the account owns the game but has no profile yet.
8. Save the refresh token, profile and ownership timestamp to the keychain.

On start: refresh silently. If the network is down and ownership was verified before, allow offline play with the cached profile.

### Launch

1. `piston-meta.mojang.com/mc/game/version_manifest_v2.json` → version JSON.
2. `meta.fabricmc.net/v2/versions/loader/<game>/<loader>/profile/json`, merged over the vanilla JSON (libraries appended, `mainClass` replaced).
3. Download the missing client jar, libraries (filtered by `rules`), natives and assets. Each file is SHA-1 verified, with parallel downloads.
4. Resolve the Java component from `javaVersion.component` in Mojang's runtime manifest; download it if missing ([ADR 0008](adr/0008-managed-java-runtime.md)).
5. Substitute the argument placeholders (`${auth_player_name}`, `${auth_uuid}`, `${auth_access_token}`, `${game_directory}`, `${assets_root}`, `${version_name}`, …). `game_directory` is the profile folder.
6. Spawn `java`, pipe stdout/stderr to `logs/`. The access token is never logged.

### Install a mod ([ADR 0007](adr/0007-modrinth-only-dependency-resolution.md))

1. Search: `GET api.modrinth.com/v2/search?facets=[["versions:<v>"],["categories:fabric"],["project_type:mod"]]`.
2. Pick: `GET /v2/project/<id>/version?loaders=["fabric"]&game_versions=["<v>"]`; take the newest, primary file.
3. Resolve: for each `required` dependency not yet installed, repeat step 2 (or use its pinned `version_id`). If an installed mod is listed as `incompatible`, abort with the conflict named.
4. Download, verify SHA-512, write into `profiles/<v>-fabric/mods/`, record in `panda-mods.json` (`project_id`, `version_id`, `file`, `sha512`, `required_by`).
5. Uninstall removes the file and the record; warn if `required_by` is not empty.

### Update the launcher ([ADR 0010](adr/0010-github-releases-updater-signed.md))

1. 3 s after start, then hourly (retry after 5 min on failure): `GET api.github.com/repos/juanestban/panda-launcher/releases/latest`.
2. If the version is newer and the matching asset, `SHA256SUMS.txt` and `.sig` exist → show the banner.
3. When the user clicks **Update**: download with progress, verify the SHA-256 and the ed25519 signature, then:
   - MSI install (exe under `%LOCALAPPDATA%\Programs\Panda Launcher`): `msiexec /i <msi> /passive LAUNCHAPP=1`, then quit. The MSI relaunches with `--after-update`.
   - Portable: rename the running exe → `.exe.old`, move `.exe.new` into place, spawn with `--after-update`, quit. `.exe.old` is deleted on the next start.

## Network etiquette

- One shared `reqwest` client. User-Agent: `panda-launcher/<version> (+https://github.com/juanestban/panda-launcher)`. **No email or personal data in headers, URLs or bodies.**
- Respect the Modrinth `X-Ratelimit-*` headers.
- All downloads are hash-verified before use, written to a temporary name, then renamed.
