# Harden RS-TEST Proof Detection

**Date:** 2026-03-29 12:55
**Scope:** `.plans/todo/checks/rs/test.md`, `apps/guardrail3/crates/app/rs/families/test/README.md`, `apps/guardrail3/crates/app/rs/families/test/crates/assertions/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/*`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/*`, `apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

## Summary
Hardened the `RS-TEST` family so semantic result-shape assertions hidden inside assertion-macro arguments are now visible to `RS-TEST-16`, while also tightening family-root discovery and clarifying the shared-report-model exception in the plan/README. The library test suite for the `test` family is green, and the new regression proves that `assert_eq!(results[0].id, "RS-...")` no longer slips through.

## Context & Problem
The immediate trigger was an adversarial pass on the `release` family. After migrating `release` sidecars to sibling assertions crates, a temp attack that replaced an owned assertions call with:

```rust
assert_eq!(results[0].id, "RS-PUB-09");
```

still passed `RS-TEST`. That meant `RS-TEST-16` was not actually seeing result-shape semantics embedded inside assertion-macro arguments. The existing parser tracked string literals, field accesses, and path uses from normal expressions, but `visit_macro` did not recurse into the expressions passed to `assert_eq!`, `assert!`, `debug_assert_eq!`, etc.

At the same time, the current `RS-TEST` implementation had ongoing work around:

- root-local component discovery for family-shaped roots without a parent package manifest
- allowing assertions crates to prove over the shared report/result model when that is the runtime public surface
- expanding attack coverage around sidecar/owned-assertions boundary leaks

This commit captures that checker/runtime hardening as one coherent checkpoint rather than letting the parser bug fix stay buried inside a later family migration.

## Decisions Made

### Recurse into assertion-macro arguments during test-function analysis
- **Chose:** Extend the `TestBodyVisitor::visit_macro` path to parse comma-separated expression arguments for assertion/panic macros and visit those expressions normally.
- **Why:** `RS-TEST-16` depends on field accesses, string literals, and path uses. If macro arguments are opaque, direct result-shape assertions inside macros become invisible.
- **Alternatives considered:**
  - Add name-based heuristics for suspicious `assert_eq!` bodies — rejected because the family explicitly avoids heuristic matching where AST proof is available.
  - Only special-case `assert_eq!` — rejected because the same blind spot exists for the broader assertion-macro set.

### Keep the shared report/result model exception narrow and explicit
- **Chose:** Document and test that assertions crates may import the shared report model only when they are proving over that runtime public surface.
- **Why:** The repo needs a principled exception for owned assertions helpers that operate on `CheckResult` without reopening arbitrary local-crate imports from assertions.
- **Alternatives considered:**
  - Forbid all report-model imports from assertions — rejected because some families legitimately prove over runtime results.
  - Treat the report model as fully exempt — rejected because that would weaken `RS-TEST-03` boundary enforcement too broadly.

### Tighten discovery/fixtures around family-local component roots
- **Chose:** Keep the current discovery/runtime changes that make root-local family component shapes visible to `RS-TEST-03` instead of silently skipping them.
- **Why:** Silent skips are the failure mode the user is explicitly trying to eliminate. If a family-shaped root has harnesses, `RS-TEST` must judge it, not ignore it.
- **Alternatives considered:**
  - Leave the existing root-local blind spot and rely on app-root validation — rejected because family-level migrations need deterministic local feedback too.

## Architectural Notes
This commit is about the `RS-TEST` checker itself, not one product family. The important architectural outcome is that `RS-TEST-16` is now based on actual semantic evidence from the parsed AST, including assertions wrapped inside macros, instead of only direct non-macro expressions.

The commit also keeps the checker aligned with the current repo direction:

- sidecars own setup
- sibling owned assertions crates own reusable semantic proof
- `test_support` stays generic
- shared report/result imports are allowed only in the narrow case where they represent the runtime public surface being proved

## Information Sources
- `.plans/todo/checks/rs/test.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove_tests/proof_bearing.rs`
- adversarial `release` temp-copy mutation replacing an owned assertions call with `assert_eq!(results[0].id, "RS-PUB-09")`
- verification commands:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib rs_test_16_assertions_modules_prove_tests::proof_bearing::sidecar_indexed_result_id_assertion_is_reported`

## Open Questions / Future Considerations
- Family-root self-validation for `apps/guardrail3/crates/app/rs/families/test` still has two live `RS-TEST-16` findings in the family’s own sidecars. I left those for the broader `RS-TEST` migration sweep because this commit is about checker correctness, not self-hosting cleanup.
- The stricter parser surfaced much larger `RS-TEST-16` debt in other families, especially `release`, which is exactly the point but means downstream migration work is still substantial.
- There is still a separate large repo cleanup around deleted legacy top-level app tests and family-local migrations that should not be conflated with this checker hardening commit.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — AST visitor that now traverses assertion-macro arguments
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — semantic sidecar proof rule
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — structural runtime/assertions boundary rule
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — owned-root/component discovery shaping the rule inputs
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove_tests/proof_bearing.rs` — regression proving indexed result-id assertions are caught
- `.plans/todo/checks/rs/test.md` — authoritative rule reminders updated for the report-model exception

## Next Steps / Continuation Plan
1. Commit this `RS-TEST` checker hardening group before touching more family-specific migration work.
2. Resume the `release` family migration, now under the stricter parser, and move remaining direct sidecar result-shape assertions into the sibling assertions crate.
3. After `release`, apply the same `RS-TEST-03`/`16` migration pattern to the next highest-count families: `garde`, `deps`, `code`, `hooks-shared`, and `deny`.
