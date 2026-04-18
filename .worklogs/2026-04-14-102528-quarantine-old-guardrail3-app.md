## Summary

Quarantined the old `apps/guardrail3` workspace under `legacy/apps/guardrail3-current` and moved the old root `guardrail3.toml` to `legacy/guardrail3.toml`. I initially repointed active packages into `legacy/`, then reversed that because it hid the architecture debt instead of surfacing it.

## Decisions made

- Archived the current old app at `legacy/apps/guardrail3-current`.
  - Rejected: overwriting the pre-existing `legacy/apps/guardrail3` tree.
- Moved the old universal config to `legacy/guardrail3.toml`.
  - Reason: leaving it at repo root kept a dead config surface looking active.
- Kept active package references broken instead of repointing them to `legacy/`.
  - Reason: the point of the move is to expose remaining coupling to old app crates and fixtures, not hide it behind a new path.
- Updated the active handoff docs to make `apps/guardrail3-rs` the current app and the old app explicitly archived.

## Key files for context

- `.plans/2026-04-14-102025-quarantine-old-guardrail3-app.md`
- `AGENTS.md`
- `README.md`
- `legacy/apps/guardrail3-current/Cargo.toml`
- `legacy/guardrail3.toml`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `fuzz/Cargo.toml`

## Surfaced breakages

- `packages/rs/garde/g3rs-garde-source-checks`
  - still depends on `guardrail3-domain-config` at `apps/guardrail3/crates/domain/config`
- `packages/rs/garde/g3rs-garde-config-checks`
  - still depends on `guardrail3-domain-modules` at `apps/guardrail3/crates/domain/modules`
- `packages/rs/clippy/g3rs-clippy-config-checks`
  - still depends on `guardrail3-domain-modules` at `apps/guardrail3/crates/domain/modules`
- `packages/rs/deny/g3rs-deny-config-checks`
  - still depends on `guardrail3-domain-modules` at `apps/guardrail3/crates/domain/modules`
- `packages/rs/code/g3rs-code-ingestion`
  - still includes test fixtures from `apps/guardrail3/tests/...`
- `fuzz`
  - still depends on `../apps/guardrail3`

## Verification

- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-source-checks`
  - failed on missing `apps/guardrail3/crates/domain/config/Cargo.toml`
- `cargo test --workspace -q` in `packages/rs/garde/g3rs-garde-config-checks`
  - failed on missing `apps/guardrail3/crates/domain/modules/Cargo.toml`
- `cargo test --workspace -q` in `packages/rs/clippy/g3rs-clippy-config-checks`
  - failed on missing `apps/guardrail3/crates/domain/modules/Cargo.toml`
- `cargo test --workspace -q` in `packages/rs/deny/g3rs-deny-config-checks`
  - failed on missing `apps/guardrail3/crates/domain/modules/Cargo.toml`
- `cargo test --workspace -q` in `packages/rs/code/g3rs-code-ingestion`
  - failed on missing `apps/guardrail3/tests/fixtures/...`
- `cargo test -q` in `fuzz`
  - failed on missing `apps/guardrail3/Cargo.toml`
- `git diff --check`
  - passed

## Next steps

- Remove `guardrail3-domain-config` usage from `garde` source checks.
- Remove `guardrail3-domain-modules` usage from `garde`, `clippy`, and `deny` config checks.
- Move `code` ingestion test fixtures out of the archived app.
- Decide whether `fuzz` should target `guardrail3-rs`, be archived too, or be deleted.
