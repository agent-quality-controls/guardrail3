Goal
- Fix `g3rs-test/real-proof-site` so it stops blaming early setup helpers like `git_init()` and only treats the actual final proof step as the proof decision.

Approach
- Add a direct rule test that proves this bug: a test may call a local setup helper first and then call the shared assertions crate later, and that must pass.
- Read the current parser and rule flow to find the narrowest correct place to fix it.
- Change `g3rs-test/real-proof-site` so local helper detection prefers the last local proof-like call instead of the first local helper-looking call.
- Re-run the rule package tests and validate `packages/rs/code/g3rs-code-ingestion --family test` to confirm the false positive is gone.

Key decisions
- Fix the rule, not the package. The package is correct to use local setup helpers before the final shared proof.
- Keep the known shared-assertions detection as-is unless the failing test proves it is also broken here.
- Do not try to model real control flow. The needed fix is simpler: use syntactic call order inside one test function.

Files to modify
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/rule.rs`
- `packages/rs/test/g3rs-test-source-checks/crates/runtime/src/rs_test_07_real_proof_site/tests/mod.rs`
