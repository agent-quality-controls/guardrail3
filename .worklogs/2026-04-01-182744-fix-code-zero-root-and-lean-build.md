# Fix Code Zero-Root And Lean Build

**Date:** 2026-04-01 18:27
**Scope:** `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs`, `apps/guardrail3/crates/app/rs/runtime/src/runners.rs`, `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs`

## Summary
Fixed two regressions left after the production tree-ingress cut. The `code` family no longer fails open when a repo has Rust files but zero discovered Cargo roots, and the lean `family-code` build no longer trips a dead-code error in the runtime runner.

## Context & Problem
After removing raw full-tree ingress from production runtime entrypoints, the remaining attack pass found two concrete `code` problems. First, `CodeFacts::collect()` returned immediately when the routed root set was empty, which let `code` silently skip stray Rust files in zero-root repos. Second, the runtime runner still compiled a helper behind `feature = "family-code"` even though only `garde` used it, so the lean `family-code` build failed on dead code.

## Decisions Made

### Keep `code` global over owned Rust files even when no Cargo roots exist
- **Chose:** Removed the early return in `CodeFacts::collect()` when `active_root_dirs` is empty.
- **Why:** `code` is a global family. Cargo roots provide extra context for structural-cap and policy-derived checks, but they must not gate basic Rust-file rule execution. If there is a routed Rust file, `code` must still inspect it.
- **Alternatives considered:**
  - Reintroduce a fake repo root just to satisfy the existing root-based flow — rejected because it hides the real contract problem and keeps root presence as an accidental gate.
  - Change runtime routing to synthesize Cargo roots for zero-root repos — rejected because `code` should not depend on fake structure to notice owned `.rs` files.

### Narrow helper compilation to the family that actually uses it
- **Chose:** Changed `scoped_route_root_cargo_files()` in the runtime runner to compile only for `family-garde`.
- **Why:** `code` uses routed root directories, not routed root Cargo paths. Leaving the helper compiled under `family-code` caused the lean build to fail on dead code even though runtime behavior was otherwise correct.
- **Alternatives considered:**
  - Add `#[allow(dead_code)]` — rejected because it masks a real feature-surface mismatch.
  - Make `code` use the helper just to keep it live — rejected because it would reintroduce unused Cargo-root dependence into `code`.

### Prove the zero-root behavior at runtime level
- **Chose:** Added a runtime regression test that creates only `guardrail3.toml` plus `tools/stray.rs`, with no Cargo roots anywhere, and asserts `RS-CODE-13` fires.
- **Why:** The existing regression only covered “Rust outside Cargo roots while another workspace exists,” which did not exercise the true zero-root fail-open.
- **Alternatives considered:**
  - Cover this only in family-local unit tests — rejected because the bug lived at the runtime-to-family routing boundary.
  - Rely on the end-to-end validator run alone — rejected because the repo’s normal output is noisy and does not isolate the zero-root contract.

## Architectural Notes
This keeps the current architecture boundary intact: runtime still routes an owned global surface into `code`, and `code` still computes root-derived context only when roots exist. The fix does not solve the larger remaining issue that `code` still rediscovers `.rs` files from the routed surface instead of receiving a precomputed owned file slice. It only removes the worst fail-open and the lean-feature compile break.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs` — zero-root early return and root-derived fact collection.
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — feature-gated helper compilation and `code` runtime surface construction.
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — existing runtime regression coverage for global `code`.
- `.worklogs/2026-03-31-201531-cut-production-tree-ingress.md` — previous boundary cut that exposed these follow-up issues.
- `.worklogs/2026-03-31-191822-runtime-scope-attack-hardening.md` — earlier routing attack pass and family runtime coverage shape.

## Open Questions / Future Considerations
- `code` still rediscovers Rust files from the routed surface; the stricter long-term fix is to route owned Rust files explicitly and stop family-side discovery.
- Family-local test helpers still use `RsProjectSurface::from_tree(...)`, which can still hide routed-surface regressions in tests even though production ingress is fixed.
- The end-to-end `family-code` validator run currently scans agent worktrees such as `.claude/worktrees/...`; that may be correct or may need explicit ignore/ownership policy elsewhere.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/code/crates/runtime/src/facts.rs` — current `code` fact collection and remaining root-dependent logic.
- `apps/guardrail3/crates/app/rs/runtime/src/runners.rs` — family runtime ingress and per-family routed surface construction.
- `apps/guardrail3/crates/app/rs/runtime/src/lib_tests/mod.rs` — runtime contract tests for global vs workspace-local routing.
- `.worklogs/2026-03-31-201531-cut-production-tree-ingress.md` — context for the prior production ingress removal.
- `.worklogs/2026-03-31-191822-runtime-scope-attack-hardening.md` — earlier attack coverage that led into this fix.

## Next Steps / Continuation Plan
1. Remove the remaining family-side rediscovery in `code` by routing explicit owned Rust-file slices from shared pre-family logic instead of scanning the routed surface.
2. Audit family-local test helpers for `RsProjectSurface::from_tree(...)` and replace them with routed surfaces so test coverage matches production boundaries.
3. Add a second zero-root regression for malformed `guardrail3.toml` if `code` should still fail closed in repos with no Cargo roots and only stray Rust files.
