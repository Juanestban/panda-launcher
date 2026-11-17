# 0004. Offline play only after verified ownership

Date: 2026-11-17 · Status: Accepted

## Context
Players expect to play without internet, like the official launcher, but the project must stay legal: no cracked accounts.

## Decision
- The refresh token, the Minecraft profile and the last ownership check are stored in the OS keychain (`keyring`, Windows Credential Manager). Never in plain files or logs.
- On start, refresh silently. If the network is unavailable **and** this account was verified online before, allow offline play with the cached profile.
- There is no way to create or use an account that was never verified online.

## Consequences
- Matches official-launcher behaviour; servers with online-mode still reject offline sessions, as expected.
- A keychain failure means the user signs in again, which is acceptable.
