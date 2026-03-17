# Rename local/ → .guardrail3/overrides/ + smart generate dry-run

**Date:** 2026-03-17 23:34
**Scope:** types.rs, generate.rs, diff.rs, init.rs, help_gen.rs, guide.rs, CLAUDE.md, guardrail3.toml, cli_tests.rs

## Summary
1. Renamed local/ → .guardrail3/overrides/ (convention path, no [local] config needed)
2. Removed LocalConfig struct and [local] section from config
3. Smart generate dry-run shows per-file status: create/update/no-changes
4. Custom entry detection in clippy.toml/deny.toml — shows what would move to overrides
5. 6 adversarial integration tests for the new behavior
