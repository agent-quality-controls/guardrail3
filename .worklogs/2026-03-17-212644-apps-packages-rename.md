# Rename [rust.crates.*] to [rust.apps.*] + [rust.packages]

**Date:** 2026-03-17 21:26
**Scope:** 10 files changed

## Summary
Config naming now matches directory structure: apps are apps, packages are packages.
- `[rust.crates.X]` → `[rust.apps.X]` for services in apps/
- New `[rust.packages]` single entry for all packages/ (one config, not per-package)
- Init only creates guardrail3.toml — local/*.toml and release files moved to generate
- Smart rs init discovers workspace members, auto-detects profiles/layers
- Merged packages config into architecture checks via clone + inject
