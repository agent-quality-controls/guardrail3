# Implement generate, init, check, diff commands in guardrail3

**Date:** 2026-03-15 13:01
**Task:** Add init, generate, check, diff, list-modules, show-module commands with embedded module content from the ts-rust-railway template.

## Goal
guardrail3 can read a `guardrail3.toml` config and generate clippy.toml, deny.toml, rustfmt.toml, rust-toolchain.toml, and pre-commit hooks. Users can init a new config, generate files, check if they're current, and diff changes.

## Approach

### Step-by-step plan
1. Update `src/cli.rs` — add Init, Generate, Check, Diff, ListModules, ShowModule commands + sub-generate under Rs/Ts/Hooks
2. Create `src/config/` — types.rs (config structs), mod.rs (loader)
3. Create `src/modules/` — clippy.rs, deny.rs, canonical.rs, pre_commit.rs, mod.rs with embedded content from template
4. Create command files: `src/commands/init.rs`, `generate.rs`, `check.rs`, `diff.rs`, `modules_cmd.rs`
5. Update `src/commands/mod.rs` and `src/main.rs` to wire everything
6. Verify `cargo build` succeeds

## Files to Modify
- `src/cli.rs` — add new command enums and arg structs
- `src/main.rs` — add mod declarations and command routing
- `src/commands/mod.rs` — add new module declarations
- New: `src/config/mod.rs`, `src/config/types.rs`
- New: `src/modules/mod.rs`, `src/modules/clippy.rs`, `src/modules/deny.rs`, `src/modules/canonical.rs`, `src/modules/pre_commit.rs`
- New: `src/commands/init.rs`, `src/commands/generate.rs`, `src/commands/check.rs`, `src/commands/diff.rs`, `src/commands/modules_cmd.rs`
