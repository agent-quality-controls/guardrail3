# Code config lane hardening

## Goal

Fix the `code` config lane so it stops scanning foreign nested repos, proves the
owned-config boundary, and closes the missing negative and branch coverage
called out by the 4-agent `test-attack`.

## Approach

1. Add failing tests first
   - add ingestion and pipeline tests for:
     - nested foreign repo config files staying quiet
     - harmless comments and quoted `EXCEPTION` markers staying quiet
     - duplicate `EXCEPTION` hits producing exact counts
     - `RS-CODE-12` `deny`, `warn`, missing, and non-workspace manifest paths
     - fail-closed behavior through the pipeline for owned unreadable/malformed inputs

2. Add explicit config ownership selection in `g3rs-code-ingestion`
   - derive owned package/config roots from the root `Cargo.toml`
   - if `[workspace]` exists, use `[workspace].members` plus root package when present
   - otherwise treat the root package only as owned
   - scan config files only at those owned roots

3. Keep the checks package unchanged unless tests prove a rule bug
   - the current problem is in ingestion selection and coverage

4. Re-run package tests
   - `packages/rs/code/g3rs-code-config-checks`
   - `packages/rs/code/g3rs-code-ingestion`

5. Re-run the 4-agent `test-attack`
   - completeness
   - missing scenarios
   - pattern parity
   - false positives

## Key decisions

- Fix ownership at selection time instead of teaching the rules about nested
  repos.
  - Why: the bug is that foreign config files enter the lane at all.

- Reuse the root-workspace/member selection pattern already used in `deps`
  ingestion instead of adding ad hoc path globs.
  - Why: member selection is already solved there and fits this lane.

- Keep the config-comment filename table broad, but require every scanned file
  to live under an owned root.
  - Why: preserves live rule scope while removing foreign-repo bleed-through.

## Files to modify

- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/config_comments.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/unsafe_code_lints.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/run.rs`
- new shared config-root helper under
  `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- optional rule-local config-check tests if exact-count coverage belongs there

## Done means

- foreign nested repos no longer enter the `code` config lane
- negative and boundary scenarios are proved end to end
- package tests pass
- 4-agent `test-attack` finds no blocker
