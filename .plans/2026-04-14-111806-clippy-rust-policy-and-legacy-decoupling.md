## Goal

Make the active `clippy` family package self-contained after the old app quarantine.

Desired end state:
- no active `clippy` crate depends on archived `apps/guardrail3` modules
- `clippy` no longer reads `guardrail3.toml`
- `clippy` uses Rust-only workspace policy from `guardrail3-rs.toml`
- compile-time and rule-time behavior are proved by tests first

## Approach

1. Add failing tests in `g3rs-clippy-ingestion` proving:
   - `guardrail3-rs.toml` drives profile and garde policy facts
   - `guardrail3.toml` is ignored
   - parse errors point at `guardrail3-rs.toml`
2. Add failing tests in `g3rs-clippy-config-checks` proving:
   - library and garde-disabled behavior comes from the Rust policy state
   - baseline test helpers no longer require archived app modules
3. Replace the archived app dependency in `g3rs-clippy-config-checks-runtime` with package-local baseline constants and a local baseline renderer used only by tests.
4. Replace `G3RsClippyPolicyContextState` with a Rust-only policy state backed by `guardrail3-rs-toml-parser`.
5. Update `g3rs-clippy-ingestion` to select and parse `guardrail3-rs.toml` instead of `guardrail3.toml`.
6. Run family tests and then the new app tests to prove the package still integrates cleanly.

## Key decisions

- Keep the existing rule IDs.
  - The bug is the policy source and the archived import, not the rule inventory.
- Keep policy optional when the file is absent.
  - Existing `clippy` behavior already stands down to defaults when no policy file exists.
  - Rejected: making `guardrail3-rs.toml` mandatory for `clippy` in this bug-fix pass.
- Use `guardrail3-rs.toml` directly instead of inventing a new `clippy`-specific schema.
  - The parser already has `profile` and `checks.garde`.

## Files to modify

- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/select.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_09_missing_method_ban_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_10_missing_type_ban_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_14_library_global_state_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_15_avoid_breaking_exported_api_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/rs_clippy_config_19_policy_context_parseable_tests/mod.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/README.md`
- `packages/rs/clippy/g3rs-clippy-ingestion/README.md`
