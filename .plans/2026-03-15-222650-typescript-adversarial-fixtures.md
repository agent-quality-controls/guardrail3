# Create 10 TypeScript adversarial test fixtures for grep-attacks

**Date:** 2026-03-15 22:26
**Task:** Create TypeScript fixtures that expose grep-based false positives in guardrail3's TS source scan

## Goal
10 TypeScript files in `tests/fixtures/grep-attacks/typescript/` where TS-specific patterns (eslint-disable, ts-ignore, process.env, any, file length) appear in non-code contexts (strings, templates, comments) to expose grep false positives.

## Approach
Create each file as valid TypeScript. For line-count fixtures, generate exactly N effective lines using `const _varN = N;` patterns.

## Files to Create
1. `string_eslint_disable.ts` — eslint-disable in string literal
2. `template_eslint_disable.ts` — eslint-disable in template literal
3. `comment_about_eslint.ts` — eslint-disable mentioned in comment (not a directive)
4. `string_ts_ignore.ts` — @ts-ignore in string literal
5. `string_process_env.ts` — process.env in string literal
6. `comment_process_env.ts` — process.env in comment
7. `type_any_in_string.ts` — ": any" in string literal
8. `generic_any.ts` — real `any` usage in generic default (SHOULD be flagged)
9. `exactly_300_lines.ts` — boundary test, 300 lines (T32 should NOT fire)
10. `exactly_301_lines.ts` — boundary test, 301 lines (T32 SHOULD fire)
