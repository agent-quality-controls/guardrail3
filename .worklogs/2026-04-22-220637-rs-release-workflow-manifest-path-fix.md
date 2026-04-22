## Summary

Fixed the release-config workflow helper so `--manifest-path` credit stays on the intended binary crate instead of falling through on filename-only `Cargo.toml` matches. Added a regression that builds two binary crates and proves the workflow only satisfies the target crate.

## Decisions made

- Fixed the matching logic in `support/workflow.rs`.
  - Why: the wrong-result bug lives in the shared workflow helper used by both binary workflow checks.
- Kept the regression at the runtime test boundary in `run_tests/cases.rs`.
  - Why: it proves the package-level result shape that callers actually consume.
- Rejected filename-only manifest-path fallback.
  - Why: it misattributes workflow coverage once more than one binary crate exists.

## Key files for context

- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`

## Next steps

- None for this fix.
