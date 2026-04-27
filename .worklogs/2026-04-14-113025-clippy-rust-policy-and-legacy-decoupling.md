## Summary

Removed the active `clippy` family's dependency on archived app modules and switched its optional policy surface from dead `guardrail3.toml` to Rust-only `guardrail3-rs.toml`. The family now compiles again after the old app quarantine and preserves the same rule behavior through package-owned constants and Rust-policy state.

## Decisions made

- Moved clippy baseline constants into the package runtime.
  - Rejected: pointing active crates at `legacy/` or keeping the deleted app module dependency.
- Reused `guardrail3-rs.toml` for profile and `checks.garde`.
  - Rejected: inventing a new clippy-only policy schema.
- Kept `guardrail3-rs.toml` optional for clippy.
  - Missing policy still falls back to the previous defaults.
  - Parse and read failures still stand down the affected rules and surface through `g3rs-clippy/policy-context-parseable`.
- Left the app untouched.
  - The bug was inside the family package, not the thin app.

## Key files for context

- `.plans/2026-04-14-111806-clippy-rust-policy-and-legacy-decoupling.md`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/select.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/baseline.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/test_support.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/README.md`
- `packages/rs/clippy/g3rs-clippy-ingestion/README.md`

## Verification

- `cargo test --workspace -q` in `packages/rs/clippy/g3rs-clippy-config-checks`
- `cargo test --workspace -q` in `packages/rs/clippy/g3rs-clippy-ingestion`
- `cargo test --workspace -q` in `packages/rs/clippy/g3rs-clippy-filetree-checks`
- `cargo test -q` in `packages/rs/clippy/g3rs-clippy-types`
- `git diff --check`

## Remaining blockers

- `cargo test -q` in `apps/guardrail3-rs` still fails, but not because of clippy.
- The next surfaced blocker is `packages/rs/deny/g3rs-deny-config-checks`, which still depends on the deleted old app domain modules.
