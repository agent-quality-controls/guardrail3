## Summary

Completed the deny family package migration. Added the missing deny filetree lane, migrated the remaining app-only deny config rules into package-owned checks, and hardened deny ingestion so coverage, shadowing, policy-context failures, unreadable files, and malformed nested deny entries fail closed with exact tests.

## Decisions made

- Split deny ownership into package lanes only:
  - `RS-DENY-FILETREE-01` and `RS-DENY-FILETREE-03` in `g3rs-deny-filetree-checks`
  - remaining deny config semantics in `g3rs-deny-config-checks`
  - no deny source lane
- Treated profile-sensitive deny rules as gated by trusted policy context.
  - `RS-DENY-CONFIG-23` and `RS-DENY-CONFIG-25` now stand down completely when `policy_context_valid == false`
  - rejected partial pre-gate enforcement because the deny family contract explicitly says these rules must stand down without trusted profile context
- Kept deny filetree coverage independent from selected-file parse or read failures.
  - selected coverage remains visible
  - failures surface alongside it
  - rejected fail-open behavior and rejected suppressing coverage inventory on failure
- Fixed malformed deny-entry handling at the shared name-normalization boundary.
  - blank or whitespace `name` / `crate` values are no longer treated as real crate identities
  - unnamed or blank-named `[[bans.deny]]` and `[bans].allow` entries now fail closed in the rules that depend on those names
  - rejected silently skipping malformed typed entries because that weakens inventory semantics

## Key files for context

- `.plans/2026-04-13-111415-deny-family-migration.md`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_23_ban_baseline_complete.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_25_allow_override_channel.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_26_extra_deny_bans_inventory.rs`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/rs_deny_config_27_wrappers.rs`
- `packages/rs/deny/g3rs-deny-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/run.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/ingest.rs`
- `packages/rs/deny/g3rs-deny-ingestion/crates/runtime/src/ingest_tests/filetree.rs`

## Next steps

- Audit `clippy` next with the same process:
  - read all live app rule code
  - compare against package code
  - classify remaining rules into config/filetree/source
  - add failing tests before each fix
