# 0008. Managed Java runtime

Date: 2026-11-17 · Status: Accepted

## Context
Each Minecraft version needs a specific Java major version. Asking players to install Java is the most common launcher support problem.

## Decision
Read `javaVersion.component` from the version JSON and download that runtime from Mojang's runtime manifest into `shared/java/<component>/`, verified by hash. Never use a system Java.

## Consequences
- Zero setup for players.
- Several hundred MB on disk per runtime component, shared across profiles.
