# Rename local/ → .guardrail3/overrides/ + smart generate

**Date:** 2026-03-17 23:20

## Part 1: Rename local/ → .guardrail3/overrides/

Files to change:
- domain/config/types.rs — LocalConfig field paths
- commands/init.rs — scaffold path, remove local/ creation from init entirely (generate does it)
- commands/generate.rs — reads override files from new path
- guardrail3.toml — self-config paths
- help_gen.rs — docs
- domain/modules/guide.rs — docs
- CLAUDE.md — docs
- tests (cli_tests.rs, any test referencing local/)

Config shape change:
```toml
[overrides]
clippy_methods = ".guardrail3/overrides/clippy-methods.toml"
clippy_types = ".guardrail3/overrides/clippy-types.toml"
deny_bans = ".guardrail3/overrides/deny-bans.toml"
deny_skip = ".guardrail3/overrides/deny-skip.toml"
deny_feature_bans = ".guardrail3/overrides/deny-feature-bans.toml"
```

Actually — wait. If the path is always .guardrail3/overrides/, why does the user need to specify it in config? Convention over configuration: guardrail3 KNOWS overrides live in .guardrail3/overrides/. No config needed. Drop the [local] / [overrides] section entirely.

## Part 2: Smart generate dry-run

The dry-run should:
1. For each managed file, check if it exists
2. If exists, diff against what generate would produce
3. If custom entries found, show what would move to .guardrail3/overrides/
4. Show file status: create / update / no changes

## Part 3: Generate extracts custom entries before regenerating

When generate runs:
1. Read existing managed file
2. Diff against generated base
3. Extract custom entries to .guardrail3/overrides/
4. Regenerate managed file = base + overrides merged
5. Report what was moved
