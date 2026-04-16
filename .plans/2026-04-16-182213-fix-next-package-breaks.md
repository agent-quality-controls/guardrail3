Goal
- Fix the current broken packages after the doc cleanup so the next package-by-package pass can continue.

Approach
- Fix `packages/rs/code/g3rs-code-ingestion` by switching stale imports from `g3rs_code_ingestion_types` to the current `g3rs_code_types` surface.
- Fix `packages/rs/clippy/g3rs-clippy-ingestion` by wiring its shared run assertions to the actual exported helpers in `g3rs-clippy-filetree-checks-assertions`.
- Fix `packages/rs/toolchain/g3rs-toolchain-ingestion` by updating the real-workspace test to match the current toolchain policy enforced by live package fixtures.
- Verify each package with the smallest mechanical command that proves the fix.

Key decisions
- Treat the `code-ingestion` imports as package debt, not a rule problem.
- Treat the `clippy-ingestion` assertion calls as package debt, not a rule problem.
- Re-check the real `rust-toolchain.toml` files before changing the toolchain test so the test follows current repo policy instead of stale expectations.

Files to modify
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_files.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/real_workspaces.rs`
- one worklog for the commit
