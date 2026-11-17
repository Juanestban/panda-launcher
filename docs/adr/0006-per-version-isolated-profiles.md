# 0006. Per-version isolated profiles

Date: 2026-11-17 · Status: Accepted

## Context
Minecraft only loads mods from `<gameDir>/mods`. Players should switch versions without moving jars around. Sharing worlds across versions risks corrupting them when an older version opens a newer world.

## Decision
- Each version gets its own game directory: `%LOCALAPPDATA%\PandaLauncher\profiles/<version>-fabric/` with its own `mods/`, `config/`, `saves/`, `resourcepacks/` and `options.txt`. The launcher passes it as `--gameDir`.
- Assets, libraries, version jars and Java runtimes are shared under `shared/` and downloaded once.
- v1 has one profile per version, created on first play. Several named profiles per version come in v2.

## Consequences
- Switching versions is just choosing another profile; nothing is copied or moved.
- Worlds and options are not shared between versions (an intended safety trade-off).
