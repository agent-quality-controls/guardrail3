Goal

- Finish the remaining Rust-only cleanup outside `hexarch` by removing dead CLI compatibility from hooks, stale dead-config text from release, and the broken old-app fuzz crate.

Approach

- Change the stale hook golden tests first so `g3rs --config ... validate` is no longer accepted, then fix the hook rule to match the real `guardrail3-rs` CLI shape.
- Remove the stale release TODO text that still describes dead `guardrail3.toml` behavior.
- Remove the active `fuzz` crate, which only targets deleted `apps/guardrail3` internals and no longer has a valid dependency graph.
- Re-run hooks tests and app tests, plus a grep/metadata pass proving the deleted old-app dependency is gone.

Key decisions

- Reject dead global-flag compatibility in hooks instead of preserving it.
  - Reason: the live `guardrail3-rs` CLI has no such global flags.
- Delete `fuzz` from the active tree instead of repointing it.
  - Reason: its targets fuzz deleted old-app internals, not current package boundaries.

Files to modify

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/mod.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_08_guardrail_validate_staged_present/tests/golden.rs`
- `packages/rs/release/g3rs-release-source-checks/TODO.md`
- `fuzz/Cargo.toml`
- `fuzz/fuzz_targets/fuzz_config_parse.rs`
- `fuzz/fuzz_targets/fuzz_source_scan.rs`
- `fuzz/fuzz_targets/fuzz_discover.rs`
