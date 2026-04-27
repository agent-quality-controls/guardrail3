## Summary

Completed the clippy package migration under the pointed-workspace model. Added the missing filetree lane, migrated the remaining config rules, fixed the published-library fallback bug in ingestion, and hardened the package tests until the adversarial agents stopped finding concrete package-model bugs.

## Decisions made

- Kept the package boundary pointed-workspace-only.
  - Rejected reviving the old app's repo-wide routed-workspace behavior inside packages because the user explicitly set the package model to one workspace per run.
- Migrated the remaining live app config rules into `g3rs-clippy/missing-method-ban..21`.
  - Kept `g3rs-clippy/test-relaxations` as package-native policy rather than deleting it just because it has no live old-app ID.
- Built `g3rs-clippy-filetree-checks` only for root coverage and same-root conflict.
  - Rejected treating old repo-scoped descendant workspace coverage as a required package behavior.
- Made published-library detection best-effort.
  - Rejected aborting clippy ingestion on malformed or unreadable root `Cargo.toml` because only the library exemption depends on that fact.
- Documented that old app `g3rs-clippy/local-policy-root` and `g3rs-clippy/no-op-placeholder` do not survive as standalone package rules.
  - `13` depended on repo-routed local policy roots.
  - `15` was a no-op placeholder.

## Key files for context

- `.plans/2026-04-13-122834-clippy-family-migration.md`
- `packages/rs/clippy/g3rs-clippy-types/src/lib.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/src/support.rs`
- `packages/rs/clippy/g3rs-clippy-filetree-checks/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/run.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/parse.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/basic.rs`
- `packages/rs/clippy/g3rs-clippy-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`

## Next steps

- Audit the next remaining partial family with the same pointed-workspace package standard.
- If clippy policy changes later, update both the package README/TODO and the direct rule tests together.
