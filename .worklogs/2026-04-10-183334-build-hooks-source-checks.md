## Summary

Built the hook-source extraction path under `packages/`.
This added a shared shell parser package, extracted source checks for `hooks-shared` and `hooks-rs`, and wired source ingestion with real selection and pipeline tests.

## Decisions made

- Extracted shell parsing into `packages/parsers/hook-shell-parser` so both hook families share one parser boundary.
- Kept hook script body checks in the source lane, not config or file-tree.
- Kept `hooks-rs` source ingestion scoped to the effective pre-commit hook only, matching app behavior.
- Kept `hooks-shared` source ingestion scoped to the effective pre-commit hook plus modular `.githooks/pre-commit.d/*` scripts.
- Failed closed on unreadable selected hook scripts instead of silently skipping them.
- Added real ingestion tests instead of keeping unused scaffold dev-dependencies.

## Key files for context

- `.plans/2026-04-10-180638-build-hooks-source-checks.md`
- `packages/parsers/hook-shell-parser/crates/parser/runtime/src/lib.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks/crates/runtime/src/run.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/src/ingest_tests`

## Next steps

- Build `hooks-shared` config checks for the non-source hook rules.
- Build `hooks-rs` config checks for tool-availability rules if they stay out of source.
- Then return to the remaining mixed structural families.
