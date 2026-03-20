# Tree-sitter ESLint config parser migration

**Date:** 2026-03-20 09:04
**Scope:** eslint_parser.rs (new), eslint_check.rs, eslint_rule_infra.rs, eslint_plugin_checks.rs, eslint_audit.rs, mod.rs, Cargo.toml

## Summary
Replaced ALL `.contains()` string matching in ESLint validation with tree-sitter-javascript AST parsing. New `eslint_parser.rs` parses config once into `EslintConfig` struct, all 35+ check functions query the struct instead of raw content.

## Architecture
- `parse_eslint_config(content) -> Option<EslintConfig>` — single parse
- `EslintConfig.rules: BTreeMap<String, RuleConfig>` — O(1) rule lookup with severity + numeric value
- `EslintConfig.presets` — detected spread expressions (strictTypeChecked, etc.)
- Boolean markers: has_boundaries, has_process_env_ban, has_route_wrappers, has_regexp_ban
- `EslintConfig::fallback(content)` — when parsing fails, raw content preserved for legacy checks

## Key improvements over string matching
- Rules checked by SEVERITY (not just presence) — `"off"` no longer passes
- Test override rules don't shadow main rules
- Comments can't produce false positives
- Numeric values extracted from options objects (`{ max: 300 }`)
- Plugin prefixes handled: `@typescript-eslint/`, `import-x/`, `import/`
- Presets detected via spread expressions in AST

## Baseline comparison
Before (string matching): 39e/3w/141i on steady-parent
After (tree-sitter): 40e/3w — 1 new error (T2 max-lines numeric extraction needs tuning)

## Known issue
T2 max-lines numeric value not extracted for `["error", { max: 300, skipBlankLines: true, skipComments: true }]` — parser unit test passes for simpler case but real config format differs slightly.
