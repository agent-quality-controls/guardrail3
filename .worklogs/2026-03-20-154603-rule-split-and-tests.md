# Split RS-ARCH-01 into per-rule files + exhaustive rule_01 tests

**Date:** 2026-03-20 15:46

## Summary
Split RS-ARCH-01 check into per-rule files under arch/rs_arch_01/. Split tests to mirror. Wrote 41 exhaustive tests for rule 01 covering all 54 identified scenarios. Also wrote initial tests for rules 02-06 and 12 (from parallel agents).

## Check code structure
- `arch/rs_arch_01/mod.rs` — orchestrator
- `arch/rs_arch_01/check_01_crates_exists.rs` through `check_12_src_banned.rs`
- `arch/rs_arch_01/helpers.rs` — shared utilities

## Test structure
- `tests/unit/rs_arch_01/` — per-rule test files
- `tests/unit/rs_arch_01/rule_01.rs` — 41 tests (symlinks, filesystem edge cases, hex-in-hex detection, cascading failures, error quality, wrong placement, app detection)
- Old test files moved to `tests/unit/legacy/`
- Old check code copied to `crates/app/rs/validate/legacy/`

## Gaps found
- rule_06: .gitkeep-only leaf subdirs not supported (test fails)
- rule_12: empty src/ dir not detected (test fails)
- Rule 01 tests all pass — may indicate tests aren't adversarial enough

## Next steps
- Adversarial rule_02 tests that mutate ALL apps simultaneously
- Fix gaps exposed by failing tests
- Implement rules 7-11 (workspace enforcement)
