# Style Policy Prefix And Pattern Bans

## Goal

Make the style policy rule able to ban class families such as arbitrary Tailwind typography classes without forcing apps to enumerate every exact class token.

## Approach

- Update `g3ts-eslint-plugin-style-policy` rule options:
  - keep `denyList` for exact class token bans
  - add `denyPrefixes` for simple class token prefix bans
  - add `denyPatterns` for regular expression class token bans
- Keep source scanning inside ESLint. G3TS must not scan source files for this rule.
- Update rule reporting so the reported policy value is specific:
  - exact token
  - prefix
  - regex pattern
- Update plugin tests to prove:
  - exact token bans still work
  - prefix bans catch arbitrary Tailwind classes
  - regex bans catch matching tokens
  - partial dynamic template extraction still catches visible denied parts
- Update G3TS style ingestion so `style-policy/no-denied-class-tokens` is effective when the first options object has at least one non-empty `denyList`, `denyPrefixes`, or `denyPatterns`.
- Update G3TS style messages and tests so they no longer imply exact-token-only policy.

## Key Decisions

- Prefixes are first-class because they are easier to configure safely than regex for Tailwind arbitrary values.
- Regex is still supported for cases prefixes cannot express, but the plugin treats invalid regex as configuration errors.
- G3TS checks only wiring and non-empty policy mechanism. It does not own the policy values.

## Files

- `packages/ts/g3ts-eslint-plugin-style-policy/src/utils/options.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/src/rules/no-denied-class-tokens.ts`
- `packages/ts/g3ts-eslint-plugin-style-policy/tests/no-denied-class-tokens.test.ts`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint.rs`
- `packages/ts/style/g3ts-style-ingestion/crates/runtime/src/eslint_tests/cases.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run.rs`
- `packages/ts/style/g3ts-style-config-checks/crates/runtime/src/run_tests/cases.rs`
