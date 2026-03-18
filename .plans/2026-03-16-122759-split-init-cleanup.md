# Split init into rs/ts, remove monorepo profile, move ValidateDomains

**Date:** 2026-03-16 12:27
**Task:** Clean separation of RS and TS stacks

## Changes

### 1. Move ValidateDomains out of rs/validate/mod.rs
- Move to src/validate.rs or src/report/types.rs (it's a shared check-category filter)
- Update all imports (main.rs, commands/validate.rs, ts/validate/mod.rs, hooks/validate.rs)

### 2. Split init command
- `guardrail3 rs init --profile service|library` — scaffolds [rust] config + local/ Rust files + release configs
- `guardrail3 ts init` — scaffolds [typescript] config (no profile needed, TS config is always the same)
- Keep `guardrail3 init` as a convenience that runs both? Or remove it entirely?
- Remove monorepo profile template from init

### 3. Remove monorepo profile
- Replace `profile == "monorepo"` with `profile == "service"` everywhere (they were identical)
- Remove monorepo from CLI --profile options
- Remove monorepo template from init
- Update CLAUDE.md profiles table

### 4. Update CLI
- Add RsCommands::Init and TsCommands::Init subcommands
- Remove or deprecate top-level init (or keep as alias for rs init)

## Files to modify
- src/cli.rs — add init subcommands to Rs/Ts
- src/main.rs — wire new init commands, move ValidateDomains import
- src/commands/init.rs — split into rs_init + ts_init functions
- src/commands/validate.rs — update ValidateDomains import
- src/rs/validate/mod.rs — remove ValidateDomains definition
- src/ts/validate/mod.rs — update import
- src/hooks/validate.rs — update import
- src/commands/generate.rs — remove monorepo profile checks
- CLAUDE.md — update profiles table
