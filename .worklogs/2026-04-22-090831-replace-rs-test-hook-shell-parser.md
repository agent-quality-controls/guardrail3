Summary

- Replaced the duplicate local shell parser in `g3rs-test-ingestion-runtime` with the shared `hook-shell-parser`.
- Added a regression test proving the old local parser missed function-dispatched `cargo mutants` hook commands.

Decisions made

- Treated the duplicate parser as a real bug, not just cleanup.
  - Why: a hook script that dispatches `cargo mutants` through a shell function is valid, and the local parser failed to detect it.
- Replaced `hook_shell.rs` entirely instead of wrapping it.
  - Why: the shared parser already owns richer shell semantics and is now the single source of truth for shell command resolution.
- Kept the command-shape logic local to `cargo mutants`.
  - Why: parser ownership is about shell semantics. The `cargo mutants` subcommand policy still belongs to the `rs/test` package.

Key files for context

- `.plans/2026-04-22-090456-replace-rs-test-hook-shell-parser.md`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/hooks.rs`
- `packages/rs/test/g3rs-test-ingestion/crates/runtime/src/ingest/run_tests/basic.rs`
- `packages/rs/test/g3rs-test-ingestion/guardrail3-rs.toml`

Next steps

- Continue the package-boundary repair in the `rs/test` family by moving analysis/normalization out of `g3rs-test-source-checks/crates/runtime/src/support.rs`.
- After that, take the same bag-to-atomic-input cleanup into `topology`, `apparch`, and `release`.
