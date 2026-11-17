# 0005. Fabric is the only mod loader

Date: 2026-11-17 · Status: Accepted

## Context
A mod built for one loader does not run on another, so supporting several multiplies the testing, metadata and UI work. The owner wants one loader for good.

## Decision
Support only Fabric. The loader profile comes from `meta.fabricmc.net` and is merged over the vanilla version JSON. Every profile is `<version>-fabric`. Forge, NeoForge and Quilt are permanently out of scope.

## Consequences
- Simpler profile model, search filters and launch code.
- Players who need Forge/NeoForge-only mods must use another launcher.
