Goal
- Fix `RS-HOOKS-SOURCE-15` so config-trigger coverage detection follows helper-function dispatch through the full parsed hook script instead of reparsing isolated branch strings.
- Remove the production-path branch reparsing in `hook_rs_16_config_changes_trigger_validation/support.rs`.

Approach
- Add red-first tests proving the current support misses config-triggered validation when a matching branch or direct trigger line calls a helper function defined elsewhere in the script.
- Replace string-level `block_contains_validation` / direct-line reparsing with line-aware checks over the original `ParsedShellScript` using `any_resolved_command_on_line`.
- Keep the existing textual branch parsing only for identifying which lines belong to which branch and whether those lines mention exact config trigger names.
- Re-run hooks source-check tests and validators, then commit as a stand-alone bug fix.

Key decisions
- Do not redesign the parser around conditional branches in this fix.
  - Why: the concrete bug is helper-resolution loss caused by reparsing branch text in isolation.
  - Rejected: a wider parser branch model change, because it is not needed to fix the proven bug.
- Use the existing full-script parsed context for command resolution.
  - Why: it already owns function definitions and executable-line semantics.

Files to modify
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/support.rs
- packages/rs/hooks/g3rs-hooks-source-checks/crates/runtime/src/hook_rs_16_config_changes_trigger_validation/rule_tests/golden.rs
