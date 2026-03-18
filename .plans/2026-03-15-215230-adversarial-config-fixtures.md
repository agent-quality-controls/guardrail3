# Adversarial config test fixtures for R1-R29

**Date:** 2026-03-15 21:52
**Task:** Create adversarial test fixtures that trigger specific config violations, run guardrail3 against them, report bugs.

## Goal
12 fixture projects in `tests/fixtures/adversarial-configs/`, each triggering specific R-checks. An integration test file validates each one by running the CLI and parsing JSON output.

## Approach

### Fixtures (12)
1. `no-clippy-toml/` - triggers R1 (missing clippy.toml)
2. `no-deny-toml/` - triggers R8 (missing deny.toml)
3. `no-rustfmt-toml/` - triggers R21 (missing rustfmt.toml)
4. `no-toolchain/` - triggers R24 (missing rust-toolchain.toml)
5. `no-claude-md/` - triggers R49 (missing CLAUDE.md)
6. `incomplete-clippy/` - triggers R4/R5 (missing method/type bans)
7. `incomplete-deny-bans/` - triggers R12 (missing crate bans)
8. `missing-deny-licenses/` - triggers R14 (no licenses section)
9. `missing-deny-sources/` - triggers R16 (no sources section)
10. `missing-cargo-lints/` - triggers R26/R27 (no workspace lint config)
11. `relaxed-clippy-lint/` - triggers R27 (unwrap_used = "warn" instead of "deny")
12. `no-lint-inheritance/` - triggers R29 (workspace lints not inherited)

### Test file
`tests/adversarial_config_tests.rs` - runs `guardrail3 rs validate {path} --format json`, parses JSON, asserts specific check IDs fire with expected severity.

## Files to Modify
- `tests/fixtures/adversarial-configs/*/` - 12 fixture directories (new)
- `tests/adversarial_config_tests.rs` - integration test (new)
