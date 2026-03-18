# Split init command into rs init and ts init subcommands

**Date:** 2026-03-16 12:29
**Task:** Split the init command into separate `rs init` and `ts init` subcommands

## Goal
Add `Init` variants to both `RsCommands` and `TsCommands` enums, split the init logic into `run_rs` and `run_ts` functions, and wire them up in main.rs.

## Approach

### Step-by-step plan
1. `src/cli.rs` — Add `Init` variant to `RsCommands` (with profile, path, force args) and `TsCommands` (with path, force args)
2. `src/commands/init.rs` — Split into `run_rs` (creates guardrail3.toml with [rust]+[local] only, creates local/ dir, scaffolds release files for service profile) and `run_ts` (appends/creates [typescript] section). Keep `run` calling both.
3. `src/main.rs` — Add match arms for `RsCommands::Init` and `TsCommands::Init`

## Files to Modify
- `src/cli.rs` — Add Init variants to RsCommands and TsCommands
- `src/commands/init.rs` — Split run into run_rs and run_ts
- `src/main.rs` — Wire new match arms
