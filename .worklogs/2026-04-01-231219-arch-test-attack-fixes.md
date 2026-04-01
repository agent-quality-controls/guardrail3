# Arch Rewrite — Test Attack Fixes

**Date:** 2026-04-01 23:12
**Scope:** `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/`

## Summary

Three iterations of adversarial review converged. Fixed 4 bugs found across iterations 1 and 2. Iteration 3 found zero remaining issues.

## Fixes Applied

### Iteration 1 fixes
1. **boundary_violation false positive on parent→child deps**: Added `boundary.rel_dir != source_rel` check so a crate's own boundary doesn't flag its children as violations. Reduced RS-ARCH-05 from 22 false positives to 0.
2. **Unused route parameter**: Removed `_route: &RsArchRoute` from `crate_tree::collect()` and fact orchestrator. Arch collects from the project tree, not from route info.
3. **RS-ARCH-08 missing all/default validation**: Added checks that `all` feature must be non-empty (enables sub-features) and `default` feature must include `"all"`.
4. **Complex cfg() feature gate extraction**: `extract_feature_gate` now handles `#[cfg(all(feature = "x", ...))]` and `#[cfg(any(...))]` by recursing into nested meta lists.

### Iteration 2 fixes
5. **is_broad_reexport false positive on specific re-exports**: Split into `is_broad_reexport` (top-level) and `is_broad_reexport_inner` (inside paths). `pub use foo::Bar;` correctly returns false (specific item). `pub use foo;` correctly returns true (broad). Reduced RS-ARCH-02 from 1277 to 1256.

### Iteration 3
No issues found. Traced all plan examples through fixed code. Converged.

## Final validation results
3491 errors total:
- RS-ARCH-02 (lib.rs facade-only): 1256
- RS-ARCH-04 (mod.rs facade-only): 1563 (was ~1552, slight increase from new mod.rs detection)
- RS-ARCH-06 (shared flag): 383
- RS-ARCH-08 (feature gating): 190
- RS-ARCH-07 (complexity): 53
- RS-ARCH-03 (mod.rs required): 46
- RS-ARCH-05 (boundary crossing): 0 (all path deps in this repo are within workspace boundaries)

## Key Files
- `facts/crate_tree.rs` — boundary_violation() with parent-self exclusion
- `facts/facade_surface.rs` — split broad/inner reexport detection, recursive cfg extraction
- `complexity/rs_arch_08_feature_gated_exports.rs` — all/default feature validation
