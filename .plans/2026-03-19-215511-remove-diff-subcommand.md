# Remove redundant `diff` subcommand

**Date:** 2026-03-19 21:55
**Task:** Remove `diff` subcommand from CLI, keep `generate --dry-run` which calls the same code. Move `--dump-dir` to GenerateArgs.

## Goal
Eliminate the redundant `diff` subcommand. `generate --dry-run` already calls the same diff code. Move `--dump-dir` from DiffArgs to GenerateArgs.

## Approach
1. cli.rs: Remove `Diff(DiffArgs)` from both enums, remove `DiffArgs` struct, add `dump_dir` to `GenerateArgs`
2. main.rs: Remove `Diff` match arms, update `generate --dry-run` to pass `args.dump_dir.as_deref()`
3. CLAUDE.md: Remove `diff` row from command table, update `generate` row
4. Verify with `cargo check`

## Files to Modify
- `apps/guardrail3/src/cli.rs`
- `apps/guardrail3/src/main.rs`
- `CLAUDE.md`
