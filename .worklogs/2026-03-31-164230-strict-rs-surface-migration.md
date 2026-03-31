# Strict RS Surface Migration (No Raw Family Tree Ingress)

**Date:** 2026-03-31 16:42
**Scope:** `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`, `apps/guardrail3/crates/app/hooks/Cargo.toml`, Rust family runtime/test-support crates under `apps/guardrail3/crates/app/rs/families/*`, plus lockfile updates

## Summary
Completed the strict migration pass so Rust families and family test-support flows run through `RsProjectSurface` instead of raw `ProjectTree` ingress. Fixed the fallout from the earlier bulk alias rewrite, restored compilation for the migrated family matrix, and validated the lean `family-clippy` path end to end.

## Context & Problem
Previous migration work changed many family internals to `RsProjectSurface` aliases, but left the test surface inconsistent:
- many tests still built domain `ProjectTree` fixtures and passed them into helper functions now expecting `RsProjectSurface`
- some helper code still called shared structure/mapper APIs that expected `ProjectTree`
- `app/hooks` started using surface types without declaring the mapper dependency

The result was a partially migrated state: some commands passed, but full Rust family test compilation failed with type mismatches and unresolved dependency errors.

## Decisions Made

### Keep strict family ingress on `RsProjectSurface`
- **Chose:** preserve the strict direction (families consume `RsProjectSurface`) and fix the surrounding support code.
- **Why:** matches the architecture target and avoids sliding back to direct tree ingress in family entrypoints.
- **Alternatives considered:**
  - Revert family signatures back to `ProjectTree` broadly — rejected because it undoes strict isolation.
  - Keep mixed dual signatures permanently — rejected because it preserves compatibility drift.

### Make `RsProjectSurface` interoperate safely with shared collectors
- **Chose:** implement `Deref<Target = ProjectTree>` for `RsProjectSurface`.
- **Why:** allows existing shared helpers (`structure::collect`, mapper construction, etc.) to accept surfaced inputs without reopening family APIs to raw tree types.
- **Alternatives considered:**
  - Rewrite all shared collectors to surface-native signatures in this pass — rejected as too broad for this checkpoint.
  - Add one-off conversions everywhere — rejected due duplicated glue and error-prone churn.

### Move family test-support builders/walkers to surface outputs
- **Chose:** update family `test_support` crates to construct/return `RsProjectSurface`, and wrap walker outputs with `RsProjectSurface::from_tree`.
- **Why:** aligns test callsites with strict family helper signatures and removes frequent `ProjectTree`↔`RsProjectSurface` mismatch failures.
- **Alternatives considered:**
  - Patch every test callsite to convert manually — rejected as noisier and less durable.
  - Reintroduce raw-tree-only test helpers — rejected because it keeps split models alive.

### Fix dependency boundaries explicitly
- **Chose:** add mapper deps where new surface imports were introduced (`app/hooks`, family test-support crates), and remove now-unused `domain-project-tree` deps in migrated test-support crates.
- **Why:** compile under `-D unused-crate-dependencies` required explicit dependency hygiene.
- **Alternatives considered:**
  - Silence unused dependency lint — rejected because it weakens enforced hygiene.

## Architectural Notes
- Family runtime `check(...)` entrypoints stay surface-based.
- Test-support now mostly emits surface objects, so helper/test paths follow the same type contract as runtime.
- Shared structure/mapper helper calls continue to work through deref coercion, keeping migration incremental while preserving strict family ingress.
- One remaining literal reference to `guardrail3_domain_project_tree` is inside a test fixture string payload (`rs_test_03_runtime_assertions_split_tests/boundaries.rs`), not runtime ingress.

## Information Sources
- `AGENTS.md`
- Recent checkpoint worklogs:
  - `.worklogs/2026-03-31-160904-rs-project-surface-family-routing.md`
  - `.worklogs/2026-03-31-153735-enforce-arch-owned-placement-and-local-family-routing.md`
  - `.worklogs/2026-03-31-140740-rs-structure-legality-routing.md`
- Updated code paths:
  - `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs`
  - `apps/guardrail3/crates/app/hooks/Cargo.toml`
  - `apps/guardrail3/crates/app/rs/families/*/test_support/{Cargo.toml,src/*.rs}`
  - `apps/guardrail3/crates/app/rs/families/release/src/test_fixtures.rs`
  - `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/*`

## Open Questions / Future Considerations
- Full workspace `cargo test --no-run` still fails on an unrelated TS module path (`crates/app/ts/validate/eslint/eslint_plugin_checks_tests` missing); Rust family matrix is green.
- If we want absolute type purity in shared helpers, a future pass can replace deref-based compatibility with explicit surface-native shared APIs.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/family_mapper/src/views.rs` — `RsProjectSurface` capabilities and new deref bridge
- `apps/guardrail3/crates/app/hooks/Cargo.toml` — mapper dependency needed by surface-based hooks wiring
- `apps/guardrail3/crates/app/rs/families/*/test_support/src/*.rs` — surface-first fixture/walker outputs
- `apps/guardrail3/crates/app/rs/families/release/src/test_fixtures.rs` — release family test entry alignment
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/dependency_integrity/` — strict test type alignment fixes
- `.worklogs/2026-03-31-160904-rs-project-surface-family-routing.md` — previous step this pass stabilizes

## Next Steps / Continuation Plan
1. Resolve the unrelated TS missing test module so full workspace `cargo test --no-run` can be green again.
2. Audit remaining non-test-support crates for any accidental raw-tree-only assumptions hidden behind fixture strings or generated snippets.
3. Decide whether to remove deref bridging once all shared helpers are explicitly surface-native.
