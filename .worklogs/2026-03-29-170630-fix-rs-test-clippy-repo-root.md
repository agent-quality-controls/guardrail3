# Fix RS-TEST Clippy Repo Root

**Date:** 2026-03-29 17:06
**Scope:** `apps/guardrail3/crates/app/rs/families/clippy/**`

## Summary
Removed the remaining repo-root `RS-TEST-03` debt in the `clippy` family by stopping parity and scenario sidecars from importing local crates directly. The family still validates clean under its own `test` rules, its library suite passes, and an app-root temp-copy attack still fires when a forbidden `guardrail3_domain_modules` import is reintroduced.

## Context & Problem
After the earlier `RS-TEST` family sweep, repo-root `RS-TEST` was still reporting a `clippy` bucket even though the family-local validator was clean. The root cause was that app-root validation sees sidecar boundary violations that the family-root run does not surface the same way: several `clippy` parity and scenario tests still imported `guardrail3_domain_modules` and `guardrail3_domain_report` directly.

I briefly tried to move canonical baseline knowledge into the sibling assertions crate, but that was the wrong boundary. `RS-TEST-03` also forbids assertions modules from importing local component crates, so pushing `domain_modules` into assertions only moved the violation rather than fixing it.

## Decisions Made

### Keep parity baselines local and literal instead of routing them through local crates
- **Chose:** replace sidecar imports of `guardrail3_domain_modules` with literal expected sets/values in the parity files.
- **Why:** these tests prove generated baseline exactness, and literal expected data in the sidecar is acceptable. What the rule forbids is reaching into local component crates for that data.
- **Alternatives considered:**
  - move the constants into the assertions crate — rejected because assertions modules are also subject to `RS-TEST-03`
  - move the constants into `test_support` — rejected because `RS-TEST-18` is meant to keep `test_support` generic and non-semantic

### Keep builder access through existing generic test_support helpers
- **Chose:** use `build_fixture_clippy_toml(...)` and related generic helpers where the sidecars needed generated TOML.
- **Why:** those helpers already exist in `test_support`, do not depend on runtime internals, and avoid direct sidecar imports of `guardrail3_domain_modules`.
- **Alternatives considered:**
  - call `guardrail3_domain_modules::clippy::build_clippy_toml(...)` directly from sidecars — rejected because that is the exact repo-root `RS-TEST-03` violation

### Treat app-root validation as the authoritative attack surface
- **Chose:** validate the full `apps/guardrail3` app root after the cleanup and use a temp app-root copy for the adversarial reintroduction.
- **Why:** this exact debt existed at repo root even while the family-local validator was clean, so the app root is the real source of truth here.
- **Alternatives considered:**
  - rely only on the family-local `clippy --family test` run — rejected because it had already failed to expose the repo-root boundary problem

## Architectural Notes
This checkpoint clarifies a useful rule boundary:
- sidecars may own literal expected parity data
- sidecars may use generic fixture/build helpers from `test_support`
- sidecars may not import local component crates directly
- sibling assertions crates may own semantic result proof, but they also may not become a tunnel to local crates

So the correct fix for `clippy` was narrower than a new helper layer: keep the assertions surface where it already belongs, and remove the illegal local-crate edges from the sidecars themselves.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/parity.rs` — representative repo-root `RS-TEST-03` offender
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — existing generic TOML builder helpers
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — boundary rule governing sidecar/assertions imports
- `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md` — earlier family-local cleanup checkpoint
- repo-root validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3 --family test --inventory --format json`
- adversarial reintroduction:
  - temp app copy with a direct `guardrail3_domain_modules::clippy::BASE_TYPE_PATHS` import restored in `rs_clippy_05_missing_type_ban_tests/parity.rs`

## Open Questions / Future Considerations
- The remaining repo-root `RS-TEST` debt is now dominated by `hooks-shared`, then `hooks-rs`, with smaller tails in `garde`, `hexarch`, `code`, and some non-family app-local files.
- The unrelated dirty `release`, `deps`, `Cargo.lock`, and `project-tree` edits remain outside this checkpoint and should stay isolated.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_05_missing_type_ban_tests/parity.rs` — representative parity test after removing direct local-crate imports
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/rs_clippy_17_test_relaxations_tests/multiple_relaxations.rs` — representative sidecar now using the sibling assertions severity surface instead of importing `guardrail3_domain_report`
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — generic fixture and TOML builder helpers that remain legal for sidecars to use
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split.rs` — the rule this cleanup is satisfying at repo root
- `.worklogs/2026-03-29-155903-finish-rs-test-clippy-family.md` — earlier family-local context for the same family

## Next Steps / Continuation Plan
1. Commit only the `clippy` files in this checkpoint plus this worklog.
2. Move directly to the hooks families:
   - `hooks-shared` first, because it is now the largest remaining bucket and has a bounded structural migration path
   - `hooks-rs` next, using the same `RS-TEST-02` sidecar-shape migration pattern
3. After the hooks families, re-run app-root `RS-TEST` and then clean the remaining smaller tails in `garde`, `hexarch`, `code`, `generate`, `project-tree`, and the `test` family itself.
