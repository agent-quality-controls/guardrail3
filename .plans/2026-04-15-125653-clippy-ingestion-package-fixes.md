Goal
- Clean the obvious package-local failures in `packages/rs/clippy/g3rs-clippy-ingestion`.
- Stop only if the remaining failures point to a doubtful rule or a rule-vs-rule contradiction.

Approach
- Fix config files first:
  - add the missing clippy bans
  - add the missing deny bans and tokio full feature ban
- Re-run package validation to shrink the output
- Fix the test package shape:
  - add the missing runtime dependency to assertions
  - move result assertions into the shared assertions crate
  - stop sidecars from calling local runtime functions directly
- Re-run full validation
- If `arch` remains, inspect whether it is a real package bug or a rule contradiction

Key decisions
- Keep release untouched in this slice. It is already fixed and green.
- Do not change `arch` unless the package side is clearly wrong.
- Prefer the already-proven `runtime + assertions + test_support` pattern if the test family needs shared helpers again.

Files to modify
- `packages/rs/clippy/g3rs-clippy-ingestion/clippy.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/deny.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/assertions/src/common.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/*`
- any new shared assertions files needed under `crates/assertions/src/`
