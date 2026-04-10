## Summary

Fixed three real hook-source bugs found by the adversarial pass.
`HOOK-RS-10` now knows when the repo is actually a Cargo workspace, `HOOK-RS-16` now tracks `guardrail3-rs.toml`, and `hooks-shared` no longer ingests nested files under `.githooks/pre-commit.d/`.

## Decisions made

- Moved workspace-project classification into `hooks-rs` source ingestion by parsing root `Cargo.toml`.
- Kept `HOOK-RS-10` in the source lane, but made it inventory-only when workspace scope is not required.
- Switched `HOOK-RS-16` from the stale `guardrail3.toml` trigger to `guardrail3-rs.toml`.
- Tightened `hooks-shared` modular selection to direct children only so ingestion matches the plan and old app facts.
- Added exact-result pipeline assertions instead of loose `any(rule_id)` checks for the touched cases.

## Key files for context

- `.plans/todo/checks/hooks/rs.md`
- `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks/crates/runtime/src/hook_rs_10_test_uses_workspace/mod.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/mod.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks-rs/g3rs-hooks-rs-ingestion/crates/runtime/src/ingest_tests`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/crates/runtime/src/run.rs`
- `packages/rs/hooks-shared/g3rs-hooks-shared-ingestion/crates/runtime/src/ingest_tests`

## Next steps

- Build the remaining hook config and structural lanes.
- Then move on to the remaining mixed Rust structure families.
