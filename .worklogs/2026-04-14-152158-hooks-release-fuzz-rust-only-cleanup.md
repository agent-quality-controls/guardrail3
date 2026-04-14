Summary

- Removed the remaining dead CLI/config residue from hooks and release, and deleted the active fuzz crate that still targeted deleted `apps/guardrail3` internals.
- The active Rust-only tree no longer preserves fake `--config guardrail3.toml` hook compatibility or an old-app fuzz dependency.

Decisions made

- Rejected dead global-flag compatibility in the hooks validate-step detector.
  - Reason: the live `guardrail3-rs` CLI has no global flags before `validate`.
- Kept the release TODO file but removed the stale dead-config statement.
  - Reason: only the residue was wrong; the file itself is still active project context.
- Deleted `fuzz` from the active tree instead of repointing it.
  - Reason: its targets fuzzed deleted old-app config, source-scan, and discover internals, not current package boundaries.

Key files for context

- `.plans/2026-04-14-152108-hooks-release-fuzz-rust-only-cleanup.md`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/tests/golden.rs`
- `packages/rs/release/g3rs-release-source-checks/TODO.md`

Next steps

- Re-audit the tree for any remaining live references to deleted `apps/guardrail3` or dead `guardrail3.toml` semantics outside `hexarch`.
- Keep `legacy/` archive-only and continue deleting, not repointing, active crates that only target the deleted app model.
