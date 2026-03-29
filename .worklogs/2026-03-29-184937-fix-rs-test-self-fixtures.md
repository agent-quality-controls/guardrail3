# Fix RS-TEST Self Fixtures

**Date:** 2026-03-29 18:49
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_10_input_failures_tests/golden.rs`

## Summary
Removed the last two `RS-TEST-16` violations from the `test` family’s own runtime sidecars by routing them through the already-existing assertions helpers instead of letting the sidecars assert directly on rule absence.

## Context & Problem
After the app-local cleanup commit, repo-root `RS-TEST` still had four `RS-TEST-16` errors. Two of them were inside the `test` family itself, which is unacceptable because the family is supposed to model the exact boundary it enforces elsewhere. Both failures were simple sidecar-owned “no result with this id” assertions that duplicated existing helper semantics.

## Decisions Made

### Reuse Existing Assertions Helpers Instead Of Adding New Ones
- **Chose:** Replace the direct sidecar assertions with `assert_rule_quiet`.
- **Why:** Both tests were already proving “no findings for this rule,” and the assertions crate already owns that proof shape.
- **Alternatives considered:**
  - Add a new helper for “no result with this id” — rejected because it would duplicate `assert_rule_quiet`.
  - Leave the direct assertions in place — rejected because that is the exact `RS-TEST-16` violation the family bans.

## Architectural Notes
This commit matters less for behavior than for self-consistency: the `test` family now follows its own proof-ownership rule for these scenarios instead of cheating in its own fixtures. That keeps the remaining repo-root backlog focused on real downstream families rather than the checker family violating itself.

## Information Sources
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_03_runtime_assertions_split.rs`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_10_input_failures.rs`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-test --lib`
- `apps/guardrail3/target/debug/guardrail3 rs validate apps/guardrail3/crates/app/rs/families/test --family test --inventory --format json`
- repo-root `RS-TEST` snapshot in `/tmp/rs-test-root.json`

## Open Questions / Future Considerations
- Repo-root `RS-TEST` still has `87` errors after this change, now concentrated in `hexarch`, `code`, and `garde`.
- The remaining `RS-TEST-16` findings are both inside `hexarch`.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/boundaries.rs` — self-fixture boundary case now using owned assertions
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_10_input_failures_tests/golden.rs` — quiet-path golden case now using owned assertions only
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_03_runtime_assertions_split.rs` — owned proof helpers for `RS-TEST-03`
- `apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/rs_test_10_input_failures.rs` — owned proof helpers for `RS-TEST-10`
- `.worklogs/2026-03-29-184634-clean-rs-test-app-local-assertions-and-clippy.md` — prior chunk that left only these self-fixture and family buckets

## Next Steps / Continuation Plan
1. Commit this self-fixture cleanup without staging unrelated `release`, `deps`, `Cargo.lock`, or the already-committed app-local changes.
2. Sweep `apps/guardrail3/crates/app/rs/families/hexarch` for the remaining `RS-TEST-03` direct imports of `guardrail3_domain_report` and `guardrail3_domain_project_tree`.
3. Move the last two `hexarch` `RS-TEST-16` assertions in `rs_hexarch_01_crates_exists_tests/{core,ownership}.rs` into owned assertions helpers.
4. After `hexarch`, repeat the same import-first, proof-second sweep in `garde` and then `code`.
