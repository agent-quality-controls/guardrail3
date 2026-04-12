# Toolchain Filetree Test Hardening

## Goal

Harden the new toolchain filetree lane with tests that pin the full 01/04
interaction matrix and the parse-free ingestion boundary.

## Approach

1. Add failing tests first.
   - Rule-runtime tests for aggregate outputs:
     - modern only
     - neither
     - both
   - Pipeline tests for exact result sets.
   - Pipeline tests for malformed root content, unreadable root entries, and
     deleted-after-crawl root entries.
2. Fix only package tests and assertions as needed.
   - Do not change old app runtime behavior.
   - Do not widen package implementation unless a real package bug appears.
3. Verify mechanically.
   - `cargo test --workspace -q` in toolchain filetree and ingestion packages
   - `git diff --check`

## Key decisions

- Ignore old app runtime ownership findings.
  - Why: the current model treats the old app only as rule inventory.
- Treat malformed and unreadable root files as valid filetree presence.
  - Why: this lane is parse-free and path-based by design.
- Prefer exact full-result assertions over `any(...)`.
  - Why: the attack found that the current suite can miss extra findings and
    wrong severities.

## Files to modify

- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_01_exists_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/rs_toolchain_filetree_04_legacy_file_tests/mod.rs`
- `packages/rs/toolchain/g3rs-toolchain-filetree-checks/crates/runtime/src/test_support.rs`
- `packages/rs/toolchain/g3rs-toolchain-ingestion/crates/runtime/src/ingest_tests/filetree.rs`
