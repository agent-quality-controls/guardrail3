# Adversarial round 2 fixes

**Date:** 2026-04-06 16:04
**Scope:** g3rs-cargo-config-checks check 02, g3rs-workspace-crawl crawl.rs

## Summary
Round 2 adversarial attack found that the round 1 required-allow fix was a no-op, plus a missing banned-dir filter in the crawl's Phase 1.

## Decisions Made

### g3rs-cargo/lint-levels: catch any level deviation, not just weakening
- **Chose:** Changed `check_expected` predicate from `actual != expected && is_weaker(expected, actual)` to just `actual != expected`. The error title changed from "weakens policy" to "deviates from policy" with a direction indicator ("weaker" or "different").
- **Why:** `is_weaker("allow", X)` is always false because `level_rank("allow") = 0` and nothing ranks below 0. The round 1 fix added iteration over required-allow lints but the predicate never triggered. Any deviation from expected level is a policy violation regardless of direction.

### Crawl Phase 1: add banned-dir filter to WalkBuilder
- **Chose:** Added `filter_entry` to the WalkBuilder that skips banned directories (target, node_modules, .claude/worktrees).
- **Why:** If these directories are not gitignored (e.g., new project, missing .gitignore), Phase 1 would walk into them and include thousands of build artifacts. The old app had the same filter.

## Key Files
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs` — bidirectional level check
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule_tests/required_allow.rs` — new test proving the fix works
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl.rs` — Phase 1 filter_entry
