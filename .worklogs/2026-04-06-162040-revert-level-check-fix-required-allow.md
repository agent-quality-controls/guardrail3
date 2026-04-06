# Revert level check to weakening-only, fix required-allow separately

**Date:** 2026-04-06 16:20
**Scope:** g3rs-cargo-config-checks check 02

## Summary
Reverted the round 2 change that made check 02 flag ANY level deviation. The original behavior was correct: guardrails enforce minimums, so making a lint stricter than expected is fine. The required-allow case (redundant_pub_crate must be exactly "allow") is a different kind of check and now uses exact-match logic instead of routing through the minimum-enforcement predicate.

## What was wrong
Round 2 changed check_expected from `is_weaker(expected, actual)` to `actual != expected`. This made every package that used `missing_debug_implementations = "deny"` (all of them) get flagged, since the expected minimum was `"warn"`. To cover that up, the expected level was changed to `"deny"` and the golden fixture was changed — compounding the error.

## What's correct now
- `check_expected` uses `is_weaker` — only fires when a lint is set to a WEAKER level than the minimum. Stronger is always fine.
- Required-allow lints use exact-match — `redundant_pub_crate` must be exactly `"allow"`, any other level fires with "must be allow" and includes the reason why.
- `missing_debug_implementations` expected level is `"warn"` (the minimum). Packages setting it to `"deny"` pass cleanly.
- Golden fixture is back to `"warn"`.

## Key Files
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_02_lint_levels/rule.rs`
- `packages/g3rs-cargo-config-checks/crates/runtime/src/support.rs`
- `packages/g3rs-cargo-config-checks/crates/runtime/src/rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml`
