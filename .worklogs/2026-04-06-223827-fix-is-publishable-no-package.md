# Fix is_publishable for workspace-only roots

**Date:** 2026-04-06 22:38
**Scope:** g3rs-release-config-checks support.rs

## Summary
`is_publishable` returned `true` for Cargo.toml files with no `[package]` section (workspace-only roots). This caused all 9 per-crate release checks to fire on workspace roots like `apps/guardrail3` and `packages/reason-policy`, producing false errors. Fixed by returning `false` when `[package]` is absent.

## Verified
Ran full chain on real workspaces:
- Publishable packages (cargo-toml-parser): all checks pass correctly
- Workspace-only roots (reason-policy, apps/guardrail3): correctly produce no results
- Unpublishable packages (fuzz with publish=false): correctly skipped
