Goal
- Make `packages/rs/clippy/g3rs-clippy-ingestion` clean under the active rules.

Approach
- Add the missing root files so filetree families stop reporting missing workspace policy.
- Mark the package unpublished and add the missing release metadata so release noise becomes package-correct.
- Fix the direct `std::fs` use by routing it through the existing `fs` module.
- Clean the test package shape:
  - shared assertions crate for final proof
  - package-local or sibling test support only for setup
  - no direct `CheckResult` shape checks in sidecars
- Re-run package tests and the full validator.

Key decisions
- Fix package issues first.
- Only change a rule if the package cannot satisfy what the rule asks for.
- Reuse the same `runtime + assertions + test_support` split if this package needs it.

Files to modify
- `packages/rs/clippy/g3rs-clippy-ingestion/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/*/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/*`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/*`
