# 0003. Microsoft auth with device code and our own Azure client ID

Date: 2026-11-17 · Status: Accepted

## Context
The launcher must only let people who legally own Minecraft: Java Edition play. Mojang requires third-party launchers to use their own Azure app and to have it approved for the Minecraft API. Unapproved IDs get `403 Invalid app registration` from `login_with_xbox`. Reusing another launcher's client ID breaks its terms. [Horizon](https://github.com/horizon-rust/horizon) (`docs/microsoft-account.md`) solves the same problem.

## Decision
- Register our own Azure app (personal accounts, public client flows, no secret) and request access at <https://aka.ms/mce-reviewappid>.
- Inject the client ID at build time: `option_env!("PANDA_MSA_CLIENT_ID")`. It is not committed as a constant; CI reads it from a secret.
- Use the **device code** flow (no local redirect server): Microsoft → Xbox Live → XSTS → `login_with_xbox` → `/entitlements/mcstore` → `/minecraft/profile`. See [architecture.md](../architecture.md#sign-in-adr-0003-0004).
- Hand-written on `reqwest`, like Horizon, with full control of the XSTS `XErr` messages. No auth crate.
- Play requires `game_minecraft` or `product_minecraft` in the entitlements (Game Pass counts).
- One account in v1.

## Consequences
- Nobody can sign in until Mojang approves the app (ROADMAP Phase 0).
- Forks must register their own app.
- About 5 HTTP calls of our own to maintain if Microsoft changes the endpoints.
