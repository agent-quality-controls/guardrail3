## Summary

Restored `rs/release` config architecture to the intended shape: ingestion now passes parsed config surfaces through to config checks, and config checks interpret those surfaces directly. Fixed the workflow matcher regression exposed by the red ingestion test and cleaned the release package boundaries until the release types, ingestion, and config-check packages all validate cleanly.

## Decisions Made

- Reverted the `fmt`-style mistake for `rs/release` config:
  - removed ingestion-owned rule-shaped booleans and counters from the public config contract
  - kept parsed `Cargo.toml`, `release-plz.toml`, `cliff.toml`, and workflow surfaces intact for config checks
- Kept relation-shaped config inputs where they are the family subject:
  - `edge_checks` still model dependency relations
  - rule interpretation moved out of ingestion for version satisfaction and publishability
- Fixed the workflow-rule bug at the support boundary:
  - the binary release workflow matcher now accepts `softprops/action-gh-release@...` in addition to the rust binary upload action
  - this repair fixed both `g3rs-release/binary-release-workflow` and `g3rs-release/linux-release-target`
- Split `support` into a real facade plus sibling modules:
  - `support/mod.rs` is now facade-only
  - `support/basic.rs` holds parsed-config interpretation helpers
  - `support/workflow.rs` holds workflow graph matching
- Preserved the required owned test shapes:
  - `lib_tests/mod.rs` remains the file-owned sidecar entrypoint for internal test helpers
  - sidecar tests no longer escape through `lib_tests`
  - semantic result assertions moved into the assertions crate
- Reverted formatter spillover outside the release repair before commit.

## Key Files For Context

- `.plans/2026-04-22-140116-rs-release-config-surface-restore.md`
- `packages/rs/release/g3rs-release-types/src/types.rs`
- `packages/rs/release/g3rs-release-types/src/lib.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/crate_base.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/repo.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/mod.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/basic.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/support/workflow.rs`
- `packages/rs/release/g3rs-release-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/rs/release/g3rs-release-ingestion/crates/runtime/src/ingest/collect_tests/deps.rs`

## Next Steps

- Move to the next Rust family/package that still has a confirmed boundary defect.
- Do not apply config-doc slicing to `rs/cargo`, `rs/clippy`, or `rs/deny`; those families should keep parsed config surfaces intact unless the family subject is a derived relation rather than the config document itself.
