# Replace #[garde(skip)] with real validators on CLI String fields

**Date:** 2026-03-16 21:53
**Task:** Replace garde(skip) on String fields with real validators where appropriate

## Goal
CLI String fields that currently skip garde validation should have real validators. Only bool, Option<usize>, and enum subcommand fields should keep skip.

## Approach

### cli.rs changes:
1. `ValidateArgs.files` — change skip to `#[garde(inner(length(min = 1)))]`
2. `ValidateArgs.path` — change skip to `#[garde(length(min = 1))]`
3. `GenerateArgs.path` — change skip to `#[garde(length(min = 1))]`
4. `PathArg.path` — change skip to `#[garde(length(min = 1))]`
5. `ShowModuleArgs.name` — change skip to `#[garde(length(min = 1))]`
6. `Cli.command` — keep skip (enum subcommand)

### config/types.rs — keep all skips as-is. These are deserialized config structs with Option<String> fields that legitimately have different validation strategies (TOML schema, use-site validation). Adding length(min=1) would break empty-string configs.

### RsCommands::Init inline fields — not garde::Validate derived, so no action needed.

## Files to Modify
- `apps/guardrail3/src/cli.rs` — replace skip on 5 String fields
- Golden test JSON may need updating if self-validate output changes
