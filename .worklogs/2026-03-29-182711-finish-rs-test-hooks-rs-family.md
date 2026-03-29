# Finish RS-TEST Hooks-RS Family

**Date:** 2026-03-29 18:27
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/hooks-rs/**`

## Summary
Migrated the legacy `hooks-rs` family onto the owned `RS-TEST` shape, added sibling `assertions` and `test_support` crates, rewired every runtime sidecar to use owner-local `run_case(...)` helpers, and moved semantic result proof into the sibling assertions surface. The family now validates clean under `--family test`, and temp-copy adversarial regressions still trip `RS-TEST-03` and `RS-TEST-16` exactly where they should.

## Context & Problem
After `hooks-shared` was cleaned, `hooks-rs` became the next large `RS-TEST` bucket. It still had the old flat `*_tests.rs` sidecars, a runtime-local `test_support.rs`, no sibling assertions crate, and many sidecars directly constructing parsed hook inputs or asserting on result shape fields. That meant the family was violating all three important `RS-TEST` boundaries:
- `RS-TEST-02` for flat sidecar shape
- `RS-TEST-03` for escaping owned module boundaries through runtime-local helpers and local crate imports
- `RS-TEST-16` for sidecar-owned semantic result proof

## Decisions Made

### Convert the family to owned sidecar directories plus sibling crates
- **Chose:** move every flat sidecar into `<module>_tests/mod.rs` + `golden.rs`, add sibling `assertions` and `test_support` crates, and wire them through workspace membership plus `dev-dependencies`.
- **Why:** `RS-TEST` is enforcing structural ownership, not just test style. The correct fix was to give `hooks-rs` the same owned runtime/assertions/test-support surface that already worked in the other migrated families.
- **Alternatives considered:**
  - Leave the family single-crate and carve out hook-family exceptions in `RS-TEST` — rejected because the user explicitly asked not to relax the rules.
  - Keep flat `*_tests.rs` files and only rewrite imports — rejected because `RS-TEST-02` still correctly rejects the flat sidecar shape.

### Keep scenario setup in the owner runtime module
- **Chose:** add `#[cfg(test)] pub(super) fn run_case(...)` helpers inside each owning rule module and remove direct sidecar construction of parsed hook inputs, tool checkers, and orchestrator scaffolding.
- **Why:** sidecars are allowed to reach their owned production module subtree, but not sibling runtime modules or crate-root helpers by default. Owner-local `run_case(...)` keeps setup local while preserving the owned-boundary constraint.
- **Alternatives considered:**
  - Rebuild inputs in sidecars using `parsed_hook`, `RustHookCommandInput`, or `StubToolChecker` directly — rejected because those imports are the exact `RS-TEST-03` violations this sweep is removing.
  - Move scenario setup into assertions — rejected because assertions should prove results, not become a generic setup tunnel.

### Make helper assertions explicitly proof-bearing
- **Chose:** keep the macro-defined rule helpers, but make every exported helper call `self::assert_rule_results(...)` rather than bare `assert_rule_results(...)`.
- **Why:** `RS-TEST-16` computes proof-bearing assertion functions statically from call paths. Bare calls to macro-generated local functions were not recognized as proof-bearing exports; `self::...` makes the ownership explicit and lets the rule prove that `assert_present` / `assert_missing` really delegate to owned proof helpers.
- **Alternatives considered:**
  - Replace every helper with fully manual `assert!(...)` logic — rejected because the macro-defined helpers already encode the common result-matching surface and only needed explicit self-qualified delegation.
  - Relax `RS-TEST-16` to infer unqualified calls to macro-defined helpers — rejected because that would weaken the rule instead of fixing the family shape.

### Treat the crate-root orchestrator tests as a root-owned sidecar
- **Chose:** keep `src/lib_tests/` as the sidecar for `src/lib.rs`, but change its helper access to `super::super::run_case`.
- **Why:** the validator was correctly flagging `use crate::run_case;` as an owned-boundary escape. `lib_tests/golden.rs` sits two modules below the crate root, so `super::super::run_case` stays within the allowed local boundary.
- **Alternatives considered:**
  - Leave `use crate::run_case;` in place — rejected because `RS-TEST-03` correctly reports it.
  - Recreate orchestrator setup inside the sidecar — rejected because it would duplicate logic and reopen ownership leaks.

## Architectural Notes
`hooks-rs` now follows the same pattern as the cleaned Rust families:
- runtime modules own small `run_case(...)` test hooks
- sibling assertions modules own semantic guardrail proof
- sibling `test_support` owns parsed-hook and tool-checker scaffolding
- sidecars no longer import sibling runtime modules or directly assert on `CheckResult` fields
- the crate-root orchestrator tests are treated like any other owned sidecar, not as a special exemption

This family also exposed an important nuance in the `RS-TEST` proof model: macro-defined helper functions only count as proof-bearing for exported wrappers when the wrapper calls them through an explicit owned path that the checker can resolve. That is now documented here because other families using `define_rule_assertions!` can hit the same trap.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_02_owned_sidecar_shape.rs` — sidecar directory/declaration ownership rules
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — boundary rules for sidecars, assertions crates, and local helper paths
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_16_assertions_modules_prove.rs` — proof-bearing assertions export requirements
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — how proof-bearing assertion functions are inferred across an assertions package
- `.worklogs/2026-03-29-175039-finish-rs-test-hooks-shared-family.md` — prior hook-family checkpoint used as the immediate migration specimen
- Verification:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/hooks-rs --family test --inventory --format json`
  - temp-copy attack reintroducing direct `results[0].inventory` in `hook_rs_09_clippy_denies_warnings_tests/golden.rs`
  - temp-copy attack reintroducing `use crate::run_case;` in `src/lib_tests/golden.rs`
  - repo-root refresh: `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family test --inventory --format json`

## Open Questions / Future Considerations
- The fresh repo-root `RS-TEST` backlog is now down to `101` errors and is no longer dominated by the hook families.
- Remaining buckets are mostly:
  - flat sidecar leftovers in `crates/app/rs/ast`, `crates/app/rs/generate`, and `crates/domain/project-tree`
  - direct local-crate imports in `clippy`, `code`, `garde`, and `hexarch`
  - small `RS-TEST-16` tails in `hexarch` and the `test` family’s own fixtures
- A separate `cargo test -p guardrail3-app-rs-family-hooks-rs --lib` compile was still in progress while this checkpoint was cut because the crate is large and hits a long `rustc` pass. The structural validator and adversarial regressions were already green, so this checkpoint records the architecture state even if the cargo run finishes later in the background.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/lib.rs` — root orchestrator plus root-owned `run_case(...)` helper and `lib_tests/` wiring
- `apps/guardrail3/crates/app/rs/families/hooks-rs/assertions/src/common.rs` — shared macro-defined proof surface that the module helpers now call through `self::...`
- `apps/guardrail3/crates/app/rs/families/hooks-rs/assertions/src/hook_rs_09_clippy_denies_warnings.rs` — representative proof-bearing helper fix for macro-defined assertions
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/hook_rs_09_clippy_denies_warnings_tests/golden.rs` — representative migrated sidecar using only owner `run_case(...)` plus sibling assertions helpers
- `apps/guardrail3/crates/app/rs/families/hooks-rs/src/lib_tests/golden.rs` — representative root-sidecar boundary fix (`super::super::run_case`)
- `apps/guardrail3/crates/app/rs/families/hooks-rs/test_support/src/lib.rs` — generic parsed-hook / tool-checker setup surface
- `.worklogs/2026-03-29-175039-finish-rs-test-hooks-shared-family.md` — previous hook-family checkpoint that established the migration pattern

## Next Steps / Continuation Plan
1. Commit only the `hooks-rs` family changes plus the workspace member additions in `apps/guardrail3/Cargo.toml`; keep unrelated dirty `release`, `deps`, `Cargo.lock`, and `project-tree` work out of this commit.
2. Refresh repo-root `RS-TEST` from the current tree and take the remaining errors in this order:
   1. `crates/app/rs/ast` flat sidecars
   2. `crates/app/rs/generate` flat sidecars
   3. `crates/domain/project-tree` flat sidecars
   4. direct local-crate import cleanup in `clippy`
   5. direct local-crate import cleanup in `code`
   6. direct local-crate import cleanup in `garde`
   7. `hexarch` import and semantic-proof tails
   8. the `test` family’s own remaining `RS-TEST-16` fixtures
3. After those errors are gone, handle the remaining repo-root `RS-TEST` warning/info surfaces (`mutation hook step missing`, mutants inventory) so the whole app reaches `0/0/0`.
