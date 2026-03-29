# Continue RS-TEST Release Sweep

**Date:** 2026-03-29 13:49
**Scope:** `apps/guardrail3/crates/app/rs/families/release/*`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/{lib.rs,parse.rs,rs_test_07_real_proof_site_tests/assertions_calls.rs}`

## Summary
Continued the repo-wide `RS-TEST` cleanup by pushing the `release` family farther into the owned-assertions pattern and tightening `RS-TEST-07` so macro-defined proof helpers are recognized. This batch converted a large slice of `release` sidecars from direct semantic result assertions to sibling assertions-crate helpers and cut `release` family `RS-TEST` errors substantially, but did not finish the family.

## Context & Problem
The repo-root `RS-TEST` debt was still dominated by `RS-TEST-16` semantic assertions living directly in sidecars, with `release` as the most concentrated family specimen. Earlier work had already split `release` into `runtime`/`assertions`/`test_support`, but many sidecars still asserted on `CheckResult` fields directly. While moving those assertions out, an additional checker gap surfaced: `RS-TEST-07` did not reliably recognize owned proof sites when the assertions crate defined its helpers through the local `define_rule_assertions!` macro pattern.

## Decisions Made

### Harden `RS-TEST-07` for macro-defined assertions
- **Chose:** Extend the `test` family parser/catalog logic so assertions modules that invoke `define_rule_assertions!` are treated as exporting proof-bearing helpers.
- **Why:** `release` assertions modules often consist only of a macro invocation. Without this, sidecars that correctly called into the owned assertions crate could still be flagged as lacking a real proof site.
- **Alternatives considered:**
  - Leave the checker unchanged and force every sidecar to keep extra direct semantic assertions — rejected because it preserves the exact `RS-TEST-16` ownership leak the sweep is trying to eliminate.
  - Add brittle local `results.len()` assertions everywhere — rejected because family-level runs can contain multiple unrelated findings, so whole-result-count checks are not a stable proof-site pattern.

### Use rule-scoped proof sites instead of whole-family result counts
- **Chose:** In converted sidecars, keep one local assertion on `assertions::findings(&results)` and move semantic result-shape checks into `assertions::assert_rule_results(...)` / `assertions::assert_rule_quiet(...)`.
- **Why:** This satisfies `RS-TEST-07` locally without assuming the entire family output has size 0 or 1. It also preserves the `RS-TEST-16` split between sidecar setup and owned semantic assertions.
- **Alternatives considered:**
  - Assert `results.len() == 1` or `results.is_empty()` directly — rejected because `run_family(...)` cases often include unrelated family findings.
  - Remove the local proof site entirely and rely only on owned assertions calls — rejected because the current checker still requires a concrete proof site in the test body.

### Batch-convert the highest-yield `release` rule files first
- **Chose:** Convert the biggest remaining `release` files in descending error count rather than trying to finish smaller scattered files first.
- **Why:** This drops the family count fastest and validates the migration pattern against multiple rule styles before repeating it across the long tail.
- **Alternatives considered:**
  - Finish one entire subtheme (all `pub` rules, all `release` rules) before checking counts — rejected because the live family counts were needed to keep targeting the largest remaining buckets.
  - Stop after a single rule file specimen — rejected because the user asked to keep driving `RS-TEST` down, not to pause at a pattern demo.

## Architectural Notes
This work reinforces the intended `RS-TEST` contract:
- sidecars own scenario setup and one local proof site
- sibling assertions crates own semantic result matching
- assertions crates may export helpers through source macros without being invisible to the checker

The `release` family is now a stronger specimen for the same migration that still needs to happen in other high-debt families. The work also clarified that local proof sites should usually be rule-scoped (`assertions::findings`) rather than family-scoped (`results.len()`), because a family orchestrator may emit unrelated rule results in integration-style tree runs.

## Information Sources
- Live repo-root and family-level validation runs from `guardrail3 rs validate ... --family test --inventory --format json`
- Existing release assertions macro in `apps/guardrail3/crates/app/rs/families/release/assertions/src/common.rs`
- `RS-TEST` runtime logic in:
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs`
  - `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site.rs`
- Prior worklogs:
  - `.worklogs/2026-03-29-125544-harden-rs-test-proof-detection.md`
  - `.worklogs/2026-03-29-125951-start-rs-release-test-split.md`
  - `.worklogs/2026-03-29-130401-refresh-lockfile-and-family-cleanups.md`

## Open Questions / Future Considerations
- `release` is still not `RS-TEST`-clean. The remaining debt is now concentrated in the smaller `pub`/`release` sidecars and golden/bypass files that still assert directly on result fields.
- After `release` is clean, the same migration pattern still needs to be applied to other repo-root `RS-TEST` buckets (`garde`, `deps`, `code`, and others).
- If `RS-TEST-07` continues to be noisy after the family sweep, it may be worth revisiting whether owned assertions calls alone should count as a sufficient local proof site without the extra `assertions::findings(...)` check.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/parse.rs` — parser support for imports, calls, and macro-defined proof helpers
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — proof catalog construction and rule fan-out
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_07_real_proof_site_tests/assertions_calls.rs` — regression covering macro-defined owned assertions
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/common.rs` — shared release-family assertion helper macro and expected-result matcher
- `apps/guardrail3/crates/app/rs/families/release/src/rs_bin_01_binary_release_workflow_tests/golden.rs` — first fully migrated proof-site pattern inside `release`
- `apps/guardrail3/crates/app/rs/families/release/src/rs_bin_02_linux_target_tests/bypasses.rs` — multi-case bypass variant of the same migration
- `.worklogs/2026-03-29-125544-harden-rs-test-proof-detection.md` — prior checker hardening context for `RS-TEST`
- `.worklogs/2026-03-29-125951-start-rs-release-test-split.md` — initial `release` family split and migration rationale

## Next Steps / Continuation Plan
1. Finish the remaining `release` family `RS-TEST-16` files by continuing the same pattern: import the owned assertions module, keep one local `assertions::findings(...)` proof site, and move semantic checks into `assert_rule_results` / `assert_rule_quiet`.
2. Rerun `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-release --lib` after each logical batch and keep `guardrail3 rs validate .../families/release --family test` as the truth source for the remaining file ranking.
3. Once `release` reaches `0/0/0` for the `test` family, run the required adversarial attack pass against `release`, then move to the next repo-root `RS-TEST` concentration family using the same migration approach.
