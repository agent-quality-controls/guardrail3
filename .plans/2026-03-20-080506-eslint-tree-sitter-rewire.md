# Rewire ESLint checks to use tree-sitter parser

**Date:** 2026-03-20 08:05
**Task:** Convert all ESLint check functions from `content.contains()` to use parsed `EslintConfig` struct from tree-sitter parser.

## Goal
All ESLint check functions use the structured `EslintConfig` from `eslint_parser::parse_eslint_config()` instead of raw string matching. Parse once, pass struct to all checks.

## Approach

### Files to modify (5 files):

1. **eslint_check.rs** — Main conversion:
   - `check_eslint_config`: parse once with `eslint_parser::parse_eslint_config`, pass `&EslintConfig` to sub-checks
   - Convert all sub-functions to accept `&EslintConfig` instead of `&str`
   - T2-T5: Use `config.rules.get()` for value rules
   - T6: Use `config.has_boundaries`
   - T-ESLP-13/14: Use `config.presets`
   - T-ESLP-15: Use `config.has_regexp_ban`
   - T7: Count rules with off/warn severity (excluding test overrides)
   - T8/T49: These scan raw lines for file overrides/test relaxations — keep using `config.raw_content`
   - T40-T48, T60-T83: Use `config.rules.contains_key()` + severity check
   - T50: Use `config.has_route_wrappers`
   - T51: Use `config.has_process_env_ban`

2. **eslint_rule_infra.rs** — Rewrite `check_eslint_rule` and `check_eslint_rule_presence` to accept `&EslintConfig`
   - `check_eslint_rule`: look up rule in `config.rules`, check severity and numeric_value
   - `check_eslint_rule_presence`: look up rule in `config.rules`, verify severity is "error" and not test_override

3. **eslint_plugin_checks.rs** — Convert `check_core_plugins` and `check_content_plugins` to accept `&EslintConfig`
   - `find_missing_rules`: check against `config.rules` keys
   - `check_config_import`: keep using `config.raw_content` for import pattern detection
   - Other checks: use `config.rules` or `config.raw_content` as appropriate

4. **eslint_audit.rs** — Convert `check()` to parse config, pass to sub-checks
   - Zone definitions: use `config.rules` for `boundaries/element-types` etc.
   - Keep `config.raw_content` for pattern matching where rule names alone aren't sufficient

5. **mod.rs** — Update orchestrator calls to pass `&EslintConfig` instead of raw content

### Key decisions
- Parse once per file in `check_eslint_config`, fallback to raw content if parse fails
- For rule presence checks, a rule must exist in `config.rules`, NOT be a test override, and have severity "error"
- `find_missing_rules` in plugin checks: check both `config.rules` keys AND `config.raw_content` for patterns that aren't rule names (import markers)
