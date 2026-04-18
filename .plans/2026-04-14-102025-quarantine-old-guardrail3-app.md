## Goal

Move the current old `apps/guardrail3` workspace and its root-local legacy config out of the active app tree and under `legacy/`, while keeping the active package families and `guardrail3-rs` buildable.

## Approach

1. Preserve the existing historical `legacy/apps/guardrail3` tree.
   - Move the current app to a distinct legacy path instead of clobbering the older legacy subtree.
2. Move the old app workspace and root `guardrail3.toml` under `legacy/`.
   - Old app path target: `legacy/apps/guardrail3-current`
   - Old universal config target: `legacy/guardrail3.toml`
3. Repoint live build references.
   - Update active package path dependencies that still import old app crates.
   - Update test fixture includes that currently read files from `apps/guardrail3/tests/...`.
   - Update any live Cargo path dependency such as `fuzz/Cargo.toml`.
4. Update the active handoff/instructions so the old app no longer presents as the current code path.
5. Verify affected workspaces still compile and tests still pass.

## Key Decisions

- Use `legacy/apps/guardrail3-current` instead of `legacy/apps/guardrail3`.
  - Reason: `legacy/apps/guardrail3` already exists and contains older validate-era code.
- Move `guardrail3.toml` under `legacy/` too.
  - Reason: the user explicitly killed the old universal config; leaving it at repo root invites accidental reuse.
- Keep the old app code buildable through updated path dependencies rather than breaking it in place.
  - Reason: quarantine should remove it from active paths, not corrupt the archived tree.

## Files To Modify

- `fuzz/Cargo.toml`
- `packages/rs/garde/g3rs-garde-source-checks/crates/runtime/Cargo.toml`
- `packages/rs/garde/g3rs-garde-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/clippy/g3rs-clippy-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/Cargo.toml`
- `packages/rs/code/g3rs-code-ingestion/crates/runtime/src/ingest_tests/pipeline.rs`
- `AGENTS.md`
- `README.md`

## Files To Move

- `apps/guardrail3` -> `legacy/apps/guardrail3-current`
- `guardrail3.toml` -> `legacy/guardrail3.toml`
