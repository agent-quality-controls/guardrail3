# Arch Family Rewrite Implementation

**Date:** 2026-04-01 22:56
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/`

## Summary

Complete rewrite of the RS-ARCH family with 8 new rules across 3 groups. Removed all old rules (01-04) which were artificially scoped to packages/ path and library-only crates. New rules operate on the full crate tree with no path awareness.

## Changes

### New directory structure
- `facade/` — RS-ARCH-01 through 04 (crate facade, lib.rs facade-only, mod.rs required, mod.rs facade-only)
- `dependency/` — RS-ARCH-05, 06 (boundary crossing, shared flag)
- `complexity/` — RS-ARCH-07, 08 (force split, feature-gated exports)
- `facts/` — crate_tree, dependency_edges, facade_surface, module_layout

### Facts collection rewrite
- `crate_tree.rs`: Discovers ALL crates via Cargo.toml walk. Builds parent-child containment tree. Reads shared flag, features, complexity metrics. No packages/ filter.
- `dependency_edges.rs`: Collects all path dependencies, resolves target directories via path normalization.
- `facade_surface.rs`: Parses lib.rs and mod.rs ASTs using syn. Detects body items (implementation violations), broad re-exports, feature gates, pub exports.
- `module_layout.rs`: Scans source files for mod declarations, checks directory+mod.rs existence.

### Rules implemented
1. RS-ARCH-01: Every crate with [package] must have lib.rs or main.rs
2. RS-ARCH-02: lib.rs must be facade-only (migrated from RS-CODE-27, tightened with broad re-export detection)
3. RS-ARCH-03: Module directories must have mod.rs, foo.rs convention forbidden
4. RS-ARCH-04: mod.rs must be facade-only
5. RS-ARCH-05: Dependencies can't cross crate boundaries (path resolution, not name matching)
6. RS-ARCH-06: Non-child dependencies require shared=true in [package.metadata.guardrail3]
7. RS-ARCH-07: Force crate split at thresholds (10 sibling .rs files, 12 deps, 3 depth, 4 dirs)
8. RS-ARCH-08: Facade exports must be feature-gated with named sub-features

### Validation results on this repo
3521 errors across all rules. Key counts:
- RS-ARCH-02 (lib.rs facade-only): 1277
- RS-ARCH-04 (mod.rs facade-only): 1552
- RS-ARCH-06 (shared flag): 383
- RS-ARCH-08 (feature gating): 188
- RS-ARCH-07 (complexity): 53
- RS-ARCH-03 (mod.rs required): 46
- RS-ARCH-05 (boundary crossing): 22

## Key Files
- `facts/mod.rs` — fact collection orchestrator
- `facts/crate_tree.rs` — crate containment tree with boundary_violation() and is_direct_child()
- `facts/facade_surface.rs` — AST-based facade analysis with syn
- `facade/` — rules 01-04
- `dependency/` — rules 05-06
- `complexity/` — rules 07-08
- `lib.rs` — main check() entrypoint

## Next Steps
- Run test-attack skill for adversarial review
- RS-CODE-27 still exists in code family — needs removal or deprecation after arch migration is confirmed
