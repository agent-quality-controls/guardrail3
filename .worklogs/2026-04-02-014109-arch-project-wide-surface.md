# Arch project-wide surface + discovery analysis

**Date:** 2026-04-02 01:41

## Summary

Changed arch from per-workspace dispatch to single project-wide run. Discovered
test fixtures leaking into arch output (37 fixture crates). Investigated root
cause: families walking the surface with dirs_with_file/all_dir_rels instead of
using route roots from structure/legality.

## Changes

### Runner dispatch
- `runners.rs`: arch now runs once with project-wide `arch_surface()` instead of
  per-workspace iteration. Same pattern as topology.
- Added `arch_surface()` function that builds from all route roots + guardrail3.toml.
- Removed `family-arch` from `workspace_surface` feature gate.

### Discovery analysis
Audited all families for tree-walking method usage:
- **Full surface walkers** (the problem): arch, hexarch, release use
  `dirs_with_file("Cargo.toml")` or `all_dir_rels()` to discover structure.
  This bypasses structure/legality classification and finds test fixtures.
- **Glob expanders** (manifest-anchored): cargo, code, hexarch, release use
  `matching_dir_rels()` to expand workspace member globs. Anchored to manifest
  content, different pattern.
- **Known-path checkers** (fine): all other families use file_exists/dir_contents
  on specific known paths.

### Identified fix
Remove `dirs_with_file` and `all_dir_rels` from the surface API. This breaks
arch, hexarch, and release — forcing them to consume root inventories from the
route (structure/legality output) instead of rediscovering.

## Key Files
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — arch_surface + dispatch
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — surface API
- `.plans/2026-03-31-rs-family-audit-fix-plan.md` — systemic analysis
- `.plans/todo/2026-03-30-project-tree-root-scope-followup.md` — placement classification plan
