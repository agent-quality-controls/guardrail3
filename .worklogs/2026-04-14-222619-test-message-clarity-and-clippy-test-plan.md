# Summary
Rewrote the weakest live `test` family messages so they say the exact bad thing, the exact fix, and the reason in plain words. Also wrote the explicit plan for cleaning `packages/rs/clippy/g3rs-clippy-config-checks` under the `test` family.

# Decisions Made
- Kept `RS-TEST-SOURCE-07` broad. Rejected narrowing it to only local helper misuse because the rule must also catch tests with no proof step at all.
- Split `RS-TEST-SOURCE-07` into 2 concrete failure cases. Rejected the old single vague message because it hid which problem actually happened.
- Tightened `RS-TEST-FILETREE-02` and `RS-TEST-FILETREE-03` messages only where they were still vague. Rejected a broad rewrite of every test-family message because `RS-TEST-SOURCE-16` was already specific enough.
- Wrote the clippy test cleanup plan before package edits. Rejected jumping into file moves while the live messages were still unclear.

# Key Files For Context
- `.plans/2026-04-14-220517-rule-error-message-format.md`
- `.plans/2026-04-14-221853-clippy-test-family-fixes.md`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_02_owned_sidecar_shape/rule.rs`
- `packages/rs/test/g3rs-test-file-tree-checks/crates/runtime/src/rs_test_03_runtime_assertions_split/rule.rs`

# Next Steps
- Commit this message-fix slice as its own bug-fix commit.
- Then fix `packages/rs/clippy/g3rs-clippy-config-checks` so the `test` family is clean:
  - move shared test helpers out of runtime-local `test_support.rs`
  - wire sibling dependencies correctly
  - move proof logic into the shared assertions crate
  - keep sidecars as setup-and-run only
