# Implement help_gen.rs — runtime CLI help injection

**Date:** 2026-03-16 14:15
**Task:** Create help_gen.rs with comprehensive CLI help, wire into main.rs and lib.rs

## Goal
Every `--help` flag shows complete documentation: commands, check IDs, profiles, workflow, examples.

## Approach

### Step-by-step plan
1. Create `src/help_gen.rs` with `inject_help(cmd) -> Command` function
2. Add top-level help via `after_help()` with getting started, profiles, workflow, commands, output, scope
3. Add `rs validate` help with all Rust check IDs grouped by category
4. Add `ts validate` help with all TypeScript check IDs grouped by category
5. Add `rs init` help with profile details, files created, examples
6. Add `ts init` help with what it creates, examples
7. Wire into main.rs: replace `Cli::parse()` with `inject_help(Cli::command())` + `from_arg_matches`
8. Add `pub mod help_gen;` to lib.rs

### Key decisions
- **Static strings over registry pattern:** The plan mentioned const arrays in check modules, but that's invasive. Instead, hardcode the check list in help_gen.rs — it's a documentation file, not a code contract.
- **Single file:** Keep all help text in one file for easy maintenance.

## Files to Modify
- `src/help_gen.rs` — new file with all help content
- `src/main.rs` — change CLI parsing to use inject_help
- `src/lib.rs` — add pub mod help_gen
