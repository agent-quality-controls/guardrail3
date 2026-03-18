# Fix 4 TS validation bugs

**Date:** 2026-03-17 10:40
**Task:** Fix 4 bugs in TS validation: any detection misses type aliases, any in nested as expressions, ESLint value check flags stricter values, T34/T35 false positive on strings.

## Goal
Fix all 4 bugs so that:
1. `type X = Record<string, any>` is detected as `any` usage
2. `foo as Record<string, any>` is detected (nested `as` expressions)
3. ESLint rule values that are stricter (lower number) than expected pass instead of fail
4. T34/T35 patterns only match inside actual comments, not string literals

## Approach

### Bug 1: `any` detection misses type aliases (#6)
In `ts_code_analysis.rs`, `collect_any_types` only matches `type_annotation` and `as_expression`. A `type X = Record<string, any>` produces a `predefined_type` node with text "any" that is NOT inside a `type_annotation` or `as_expression` — it's inside a `type_alias_declaration`. Add a match arm for `predefined_type` where text is `"any"`.

### Bug 2: `any` in nested `as` expressions (#7)
In same file, the `as_expression` arm only checks direct children. `foo as Record<string, any>` has `any` nested deeper. Use `has_predefined_any_child` (recursive) instead of the shallow loop.

### Bug 3: ESLint value check flags stricter values (#8)
In `eslint_check.rs`, `check_eslint_rule` does `line.contains(val)` string matching. If expected is "300" and actual is "200", it doesn't find "300" and flags mismatch. Fix: extract numeric values from lines near the rule name, compare numerically — actual <= expected means PASS.

### Bug 4: T34/T35 false positive on strings (#10)
In `source_scan.rs`, `check_comment_pattern` uses `line.contains(pattern)`. This matches inside string literals. Fix: use tree-sitter to parse the file, extract comment nodes, only search comment text for the patterns.

## Files to Modify
- `apps/guardrail3/src/app/ts/validate/ts_code_analysis.rs` — bugs 1 & 2
- `apps/guardrail3/src/app/ts/validate/eslint_check.rs` — bug 3
- `apps/guardrail3/src/app/ts/validate/source_scan.rs` — bug 4
