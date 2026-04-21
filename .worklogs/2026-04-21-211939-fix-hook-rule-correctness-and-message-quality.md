## Summary

Fixed the Rust hook source-check messages so they state the file, the concrete command or change to make, and the reason. Also fixed two false positives in the hooks family before rewriting the messages: `RS-HOOKS-SOURCE-10` now recognizes deny-warnings clippy inside wrapped `cd && cargo clippy ...` commands, and `RS-HOOKS-SOURCE-18` now only flags unconditional `exit 0` paths instead of standard guarded early-success branches.

## Decisions Made

- Fixed rule correctness before touching wording.
  - `RS-HOOKS-SOURCE-10` was a real false positive because the rule evaluated `line.raw` instead of the parser's extracted `command_text`.
  - `RS-HOOKS-SOURCE-18` was a real false positive because it treated any executable `exit 0` as unconditional after parser control context had already been erased.
- Split `hook_rs_16_config_changes_trigger_validation` support logic into a sidecar `support.rs`.
  - The message rewrite pushed the rule file over the `RS-CODE-SOURCE-09` size threshold.
  - Moving the trigger-analysis helpers out kept the rule file small without weakening the rule.
- Made `RS-HOOKS-SOURCE-15` report the exact missing config filenames.
  - The old boolean-only message was too vague to fix from output alone.
- Synced the installed `g3rs` binary after the code changes.
  - Earlier in this session, stale local binaries already caused confusion.

## Key Files For Context

- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_09_clippy_denies_warnings/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/shell_safety/hook_shared_13_no_unconditional_exit_zero/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_17_shared_target_dir_present/rule.rs`
- `packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/workflow/hook_shared_15_merge_conflict_step_present/rule.rs`

## Verification

- `cargo test -q --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/rs/hooks/g3rs-hooks-source-checks/Cargo.toml`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/rs/hooks/g3rs-hooks-source-checks`
- `g3rs validate --path /Users/tartakovsky/Projects/websmasher/websmasher --family hooks`

## Next Steps

- If the user wants hook output fully green on `websmasher`, the next repo edits are now explicit from the messages:
  - add `cargo dupes --exclude-tests`
  - add `g3rs validate --path ...`
  - add shared `CARGO_TARGET_DIR`
  - add fail-closed shell options
  - add merge-marker scan
  - add concrete frozen-lockfile verification
