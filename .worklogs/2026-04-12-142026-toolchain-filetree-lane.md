# Toolchain Filetree Lane

## Summary

Built the missing toolchain filetree lane in packages and migrated the two remaining live toolchain filetree rules into it. The new package now owns root `rust-toolchain.toml` presence and legacy `rust-toolchain` conflict checks, with ingestion and pipeline coverage wired end to end.

## Decisions made

- Kept the filetree input root-only.
  - Why: the package model validates one pointed workspace root, so descendant toolchain files are out of scope for this lane.
- Kept filetree ingestion parse-free.
  - Why: these rules only care about file presence and conflict, so malformed file contents should not suppress them.
- Preserved the old rule messages and legacy-only behavior.
  - Why: this was a migration of live policy, not a policy rewrite.
- Left source ingestion as a stub.
  - Why: there are no live toolchain source rules to migrate.

## Key files for context

- `.plans/2026-04-12-141503-toolchain-filetree-lane.md`
- `packages/rs/toolchain/g3rs-toolchain-types/src/lib.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_01_exists.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_04_legacy_file.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/run.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/filetree.rs`

## Next steps

- Re-audit `toolchain` with the `test-attack` skill if we want an explicit adversarial migration verdict in-session.
- After `toolchain`, the next small remaining family slice is `fmt` filetree.
