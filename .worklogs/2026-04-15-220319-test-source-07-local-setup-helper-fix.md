Summary
- Fixed `RS-TEST-SOURCE-07` so it no longer treats same-file setup helpers like `git_init()` as proof helpers just because they contain an assertion. The rule now keeps blaming real local proof helpers like `assert_results()`, but setup helpers fall back to the correct "no shared proof step" result.

Decisions made
- Fixed the rule, not `g3rs-code-ingestion`. The package was correct to use a local setup helper before its final proof.
- Narrowed local proof-helper detection by helper shape, not by ad hoc package exceptions. A same-file local helper now counts only if it is clearly a proof helper by name, or if it wraps the shared assertions crate.
- Added direct regression tests for both sides:
  - local same-file `git_init()` with an assertion inside must not count as proof
  - local same-file `assert_results()` must still count as a forbidden local proof helper

Key files for context
- `.plans/2026-04-15-220003-test-source-07-final-proof-step.md`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`

Next steps
- Commit this rule fix as a stand-alone bug fix.
- Return to `packages/rs/code/g3rs-code-ingestion`.
- Keep cleaning that package until the next real contradictory or outdated rule appears.
