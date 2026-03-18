# CLI Integration Tests to Kill Surviving Mutants

**Date:** 2026-03-15 21:44
**Task:** Write integration tests for CLI layer to kill known surviving mutants

## Goal
Create `tests/cli_tests.rs` with ~15 integration tests that invoke the guardrail3 binary and check output/exit codes, killing mutants in main.rs, discover.rs, check.rs, diff.rs, generate.rs, init.rs, validate.rs, and modules_cmd.rs.

## Approach
- Use `std::process::Command` with `env!("CARGO_BIN_EXE_guardrail3")`
- Use `tempfile` for init/generate tests
- Each test targets specific mutant lines
- No `assert_cmd` needed -- raw Command is sufficient and avoids new deps

## Files to Modify
- `tests/cli_tests.rs` -- new file, all CLI integration tests
