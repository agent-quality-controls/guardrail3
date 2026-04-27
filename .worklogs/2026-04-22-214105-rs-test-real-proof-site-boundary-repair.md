## Summary

Fixed `g3rs-test/real-proof-site` so local proof-path detection no longer guesses from arbitrary `assertions` path segments inside the rule. The rule now consumes ingestion-owned same-file proof-helper facts, and the slice has red-first proof for both the missed local proof helper case and the false local `assertions` import case.

## Decisions made

- Moved same-file local proof-helper binding into `rs/test` ingestion.
  - Why: the rule was rebuilding local proof-helper state and mixing local path classification with loose text heuristics.
  - Rejected: widening the old rule-local prefix matcher in place.
- Kept a proof-helper name heuristic, but narrowed it to proof/result-oriented tokens instead of treating any helper with an assertion macro as proof.
  - Why: the existing `git_init()` negative case proves that raw assertion presence is too broad, while the new `check_results()` case proves the old prefix-only matcher was too narrow.
  - Rejected: seeding helpers from every function with an assertion macro, which created a false positive.
- Split `source_analysis` into a facade module plus sibling implementation files.
  - Why: the first fix pushed the file over the code-size guardrail and then tripped the facade-only `mod.rs` rule.
  - Rejected: leaving the bug fix in an oversized file or working around the arch rule.

## Key files for context

- `.plans/2026-04-22-212830-rs-test-real-proof-site-boundary-repair.md`
- `packages/rs/test/g3rs-test-types/src/types.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/mod.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/pipeline.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/source_analysis/proof_helpers.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule_tests/cases.rs`

## Next steps

- Fix the confirmed `rs/code` production bug in `packages/rs/code/g3rs-code-source-checks/crates/runtime/src/parse/fs_visitors.rs`, where alias bindings in `std_aliases` leak across sibling modules.
- Then take the remaining queued parallel-audit bugs in `rs/code` and `rs/hooks` without going back to serial rule-by-rule audits.
