Summary
- Fixed a real `RS-TEST-SOURCE-07` false positive. The rule knew only part of the `define_result_assertions!` helper surface, so tests calling shared helpers like `assert_has_info` were misreported as having no shared proof step.

Decisions made
- Kept the known-macro contract approach. Rejected cargo-package rewrites because the bug was in the stale rule-side helper surface.
- Moved the `define_result_assertions!` proof helper contract into one parser constant and extended it to the full active helper set:
  - `assert_findings`
  - `assert_no_findings`
  - `assert_contains`
  - `assert_has_info`
  - `assert_has_warn`
  - `assert_has_error`
  - `assert_title_count`
  - `assert_message_contains`
  - `assert_title_absent`
- Added one regression that proves every helper name from that macro counts as shared proof.

Key files for context
- `.plans/2026-04-15-211048-test-source-07-define-result-assertions-surface.md`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/parse/mod.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`

Next steps
- Keep package cleanup separate from this bug fix commit.
- Re-run full validation on any package that uses `define_result_assertions!` helper names beyond `assert_findings` and `assert_no_findings`.
