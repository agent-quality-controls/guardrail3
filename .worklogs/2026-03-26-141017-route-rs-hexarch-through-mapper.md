# Route Rs Hexarch Through Mapper

**Date:** 2026-03-26 14:10
**Scope:** `apps/guardrail3/crates/app/rs/runtime.rs`, `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/dependency_facts.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/test_support.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/tests/collectors/structural_roots.rs`, `apps/guardrail3/crates/app/rs/families/hexarch/src/rs_hexarch_20_dev_dependency_direction_tests/broad_attacks.rs`, `apps/guardrail3/Cargo.lock`

## Summary
Moved the `rs/hexarch` family off family-local root collection and onto the shared Rust `FamilyMapper` route. The family now takes routed app roots as input, while still deriving dependency context from repo paths so cross-app dependency rules keep working. I also tightened the mapper so `hexarch` only routes roots whose `Cargo.toml` content is actually present in `ProjectTree`, which preserves malformed manifests but excludes broken symlink roots.

## Context & Problem
The current architecture plan in `apps/guardrail3/crates/app/rs/README.md` says Rust root scope should be collected once and mapped into typed family inputs before any family-specific discovery runs. `rs/test` and `rs/arch` had already moved to that shape, but `rs/hexarch` still crawled `ProjectTree` directly to decide which app roots it owned. That duplicated scope logic inside a family and kept `runtime.rs` from actually being the single place where crawling is turned into family input.

`hexarch` also has tests that distinguish between three kinds of package roots:
- empty but present `Cargo.toml` files should still count as owned roots
- malformed but present `Cargo.toml` files should still count as owned roots
- broken `Cargo.toml` symlinks should not count as owned roots

The first mapper cut routed too broadly and broke that third expectation.

## Decisions Made

### Route `hexarch` through `RsHexarchRoute`
- **Chose:** Change `hexarch::check` and its fact collectors to accept `&RsHexarchRoute` instead of collecting app roots from `ProjectTree`.
- **Why:** This removes family-local live-root discovery and makes `runtime.rs -> placement -> FamilyMapper -> hexarch` the only scope path for this family.
- **Alternatives considered:**
  - Keep `hexarch` on raw `ProjectTree` until all families migrate — rejected because it preserves the exact duplication the refactor is meant to remove.
  - Pass raw `RustRootPlacementRootFacts` into rules directly — rejected because the family should still own its own normalization and dependency fan-out.

### Keep dependency target inference path-based, but ownership route-based
- **Chose:** Use routed app roots to decide which workspaces and members are owned, while still inferring dependency target context from repo paths.
- **Why:** `hexarch` rules care about dependency direction across nested crates inside an owned app root. Routing should decide which app roots are in scope, but path structure is still the right local signal for mapping source files and dependency targets within those roots.
- **Alternatives considered:**
  - Restrict all dependency reasoning to only routed root directories themselves — rejected because it would lose the inner crate structure that `hexarch` rules are supposed to validate.
  - Continue deriving owned roots from path heuristics inside `dependency_facts.rs` — rejected because that leaves family-local scope reconstruction in place.

### Treat `hexarch` roots as live only when manifest content is present
- **Chose:** Add a mapper-side liveness predicate for `map_rs_hexarch()` that requires `ProjectTree::file_content(cargo_rel_path).is_some()`.
- **Why:** `placement` still exposes roots with parse/input failures. `hexarch` tests intentionally want malformed manifests to remain owned, but broken symlink manifests should not be routed. Content presence is the simplest boundary that preserves that distinction.
- **Alternatives considered:**
  - Route every app-classified placement root, including input-failure roots — rejected because it made broken symlink roots look live.
  - Drop malformed manifests too — rejected because that weakens `hexarch`'s fail-closed structural coverage and breaks existing tests.

## Architectural Notes
This slice makes `hexarch` match the same architecture already used by `arch` and `test`:

`ProjectTree -> placement -> FamilyMapper -> typed family route -> family facts -> rule inputs`

The family still owns:
- workspace/member normalization
- dependency edge collection
- rule-specific fan-out

But it no longer owns:
- deciding which app roots are live
- walking the tree to discover app roots for itself

This is still not the end state for the Rust orchestrator. `runtime.rs` retains applicability/filtering logic, and other families (`cargo`, `code`, `garde`, `release`) still do family-local cargo-root discovery. This commit is a step toward that cleanup, not the whole migration.

## Information Sources
- `apps/guardrail3/crates/app/rs/README.md` — target architecture for placement, family selection, and family mapper
- `apps/guardrail3/crates/app/rs/runtime.rs` — current Rust orchestration entrypoint
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — existing routed entrypoints for `arch` and `test`
- `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs` — family entrypoint
- `apps/guardrail3/crates/app/rs/families/hexarch/src/facts.rs` and `dependency_facts.rs` — existing family-local discovery and normalization
- `.worklogs/2026-03-26-135559-rs-routing-cleanup-and-repo-sync.md` — prior checkpoint reviewing the remaining decoupling work

## Open Questions / Future Considerations
- `FamilyMapper` still owns some policy that the README says should live before mapping.
- `runtime.rs` still post-filters results and keeps a separate applicability layer.
- `code`, `garde`, `cargo`, and `release` still reconstruct cargo-root scope locally.
- The shared placement layer still exposes placement-shaped facts rather than the narrower route view types described in the README.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/README.md` — current architecture contract for shared scope, family selection, and typed mapping
- `apps/guardrail3/crates/app/rs/runtime.rs` — current Rust runtime orchestration and family dispatch
- `apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs` — mapper logic and routed family inputs
- `apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs` — routed `hexarch` family entrypoint
- `apps/guardrail3/crates/app/rs/families/hexarch/src/facts.rs` — routed structural fact collection
- `apps/guardrail3/crates/app/rs/families/hexarch/src/dependency_facts.rs` — route-aware dependency normalization
- `apps/guardrail3/crates/app/rs/families/hexarch/src/test_support.rs` — test harness path for constructing routed family inputs
- `.worklogs/2026-03-26-135559-rs-routing-cleanup-and-repo-sync.md` — latest architectural review before this migration

## Next Steps / Continuation Plan
1. Migrate the remaining families that still crawl Rust roots locally, starting with `code` and `garde`, onto typed mapper routes.
2. Move any remaining family applicability policy out of `FamilyMapper` so the mapper becomes a pure projection layer.
3. Thin `runtime.rs` by removing post-hoc applicability filtering once all routed families consume the external selection and mapping flow.
4. Add regression tests that prove mapper-owned file scoping and routed-root ownership for the migrated families.
