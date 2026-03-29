# Finish RS-TEST Release Family

**Date:** 2026-03-29 14:23
**Scope:** `apps/guardrail3/crates/app/rs/families/release/**`

## Summary
Finished the `RS-TEST` migration for the `release` family by moving the remaining sidecar-owned semantic assertions into the family’s owned assertions modules. The family now validates cleanly against `RS-TEST`, its library tests are green, and an adversarial temp-copy pass still catches reintroduced `RS-TEST-16` and `RS-TEST-03` regressions.

## Context & Problem
After the earlier `release` split and initial `RS-TEST` sweep, the family still concentrated the repo’s remaining `RS-TEST-16` debt. The remaining failures were mostly direct `CheckResult` field assertions in golden and bypass sidecars, plus a final pair of `RS-TEST-03` cross-rule helper imports after the semantic migration was nearly complete.

The goal here was not only to make `release` pass, but to make it pass honestly:
- sidecars keep local scenario setup and one local proof site
- owned sibling assertions modules own semantic result matching
- cross-rule checks must still route through the sidecar’s owned assertions module, not through direct sibling assertions imports

## Decisions Made

### Finish the family instead of stopping at the first working specimen
- **Chose:** convert the remaining long tail of `release` sidecars instead of leaving the family half-migrated.
- **Why:** `release` was still one of the largest repo-root `RS-TEST` buckets, and partial migration would keep the family as a recurring blocker.
- **Alternatives considered:**
  - Stop after the biggest files were converted — rejected because the remaining files were still enough to keep family-level validation red.
  - Move on to another family and come back later — rejected because that would multiply contexts while leaving one family internally inconsistent.

### Use owned assertions helpers even for cross-rule expectations
- **Chose:** route cross-rule checks through helpers in the owned assertions module for the sidecar’s own rule instead of importing sibling assertions modules directly.
- **Why:** `RS-TEST-03` is explicit that a sidecar may only import its owned assertions module. Cross-rule expectations are allowed, but they must be mediated by the owned assertions surface.
- **Alternatives considered:**
  - Import sibling assertions modules directly — rejected because that is exactly what `RS-TEST-03` forbids.
  - Leave direct result scans in the sidecar for cross-rule cases — rejected because that recreates `RS-TEST-16` semantic ownership leaks.

### Keep rule-scoped proof sites local and move all result semantics into assertions crates
- **Chose:** use local `assertions::findings(...)` checks as proof sites and use `assert_rule_results` / `assert_rule_quiet` for semantic matching.
- **Why:** the checker still wants a concrete local proof site, but semantic result ownership belongs in the sibling assertions module.
- **Alternatives considered:**
  - Assert family-global `results.len()` counts — rejected because family runs may include unrelated rule output.
  - Remove local proof sites entirely — rejected because it would fall foul of the current `RS-TEST-07` proof expectations.

## Architectural Notes
This commit makes `release` a complete specimen for the current `RS-TEST` contract:
- sidecars do setup
- sibling assertions modules own result semantics
- cross-rule checks stay inside the owned assertions surface
- the family remains self-hosted and clean under `RS-TEST`

The two last `RS-TEST-03` failures were useful because they clarified the intended architecture more sharply: “owned assertions only” is not merely about where helper code lives, but about which sibling crate boundary a sidecar is allowed to depend on.

## Information Sources
- Live family validation:
  - `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/release --family test --inventory --format json`
- Family tests:
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-release --lib`
  - focused runs for `rs_release_02_release_plz_exists` and `rs_release_12_input_failures`
- Owned assertions modules:
  - `apps/guardrail3/crates/app/rs/families/release/assertions/src/rs_release_02_release_plz_exists.rs`
  - `apps/guardrail3/crates/app/rs/families/release/assertions/src/rs_release_12_input_failures.rs`
- Prior worklogs:
  - `.worklogs/2026-03-29-125951-start-rs-release-test-split.md`
  - `.worklogs/2026-03-29-134926-continue-rs-test-release-sweep.md`

## Open Questions / Future Considerations
- `release` is now family-clean for `RS-TEST`, but repo-root `RS-TEST` still has large remaining buckets in other families.
- The same owned-assertions migration pattern should be applied next in the next largest remaining family rather than re-invented.
- If future cross-rule cases become common, it may be worth formalizing a more obvious helper naming convention for “owned rule asserting related rule” helpers.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/rs_release_02_release_plz_exists.rs` — owned assertions surface for `RS-RELEASE-02`, including related-rule helpers
- `apps/guardrail3/crates/app/rs/families/release/assertions/src/rs_release_12_input_failures.rs` — owned assertions surface for `RS-RELEASE-12`, including related-rule helpers and file-absence checks
- `apps/guardrail3/crates/app/rs/families/release/src/rs_release_02_release_plz_exists_tests/bypasses.rs` — final cross-rule `RS-TEST-03` specimen after migration
- `apps/guardrail3/crates/app/rs/families/release/src/rs_release_12_input_failures_tests/readme_failures.rs` — final cross-rule `RS-TEST-03` specimen after migration
- `apps/guardrail3/crates/app/rs/families/release/src/rs_pub_14_include_exclude_inventory_tests/golden.rs` — representative converted run-family golden using owned assertions
- `.worklogs/2026-03-29-134926-continue-rs-test-release-sweep.md` — prior release-family sweep context

## Next Steps / Continuation Plan
1. Stage and commit the `release` family sweep as its own checkpoint so the repo history has one clean family-level `RS-TEST` completion point.
2. Re-run repo-root `RS-TEST` validation and rank the next remaining family buckets by count.
3. Apply the same migration pattern to the next family in descending order:
   - convert direct semantic sidecar assertions into owned assertions helpers
   - remove sibling assertions imports by adding owned related-rule helpers where needed
   - re-run family-level `RS-TEST`, family library tests, and an adversarial temp-copy regression before committing
