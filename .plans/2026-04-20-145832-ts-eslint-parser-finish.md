## Goal

Finish the `eslint-config-parser` package and correct the `ts/eslint` family boundary so the parser package and the three ESLint family roots are clean under tests, formatting, and `g3rs validate`.

## Approach

1. Check the current parser package and `ts/eslint` roots for architectural and validation issues.
2. Fix the active-root selection order in `g3ts-eslint-ingestion` to match official ESLint precedence.
3. Run package-local tests and formatting for:
   - `packages/parsers/eslint-config-parser`
   - `packages/ts/eslint/g3ts-eslint-types`
   - `packages/ts/eslint/g3ts-eslint-ingestion`
   - `packages/ts/eslint/g3ts-eslint-config-checks`
4. Run `g3rs validate` on those four roots and fix any findings without widening scope into unrelated in-flight work.

## Key Decisions

- Keep ESLint evaluation Node-backed through the official `eslint` API.
  - Reason: `eslint.config.*` is executable module code, not a declarative format that a pure Rust parser can faithfully evaluate.
- Keep multi-config discovery out of `config-checks`.
  - Reason: `config-checks` should consume parsed file content, not discovery summaries.
- Limit this pass to parser and ESLint-family roots only.
  - Reason: the worktree already contains large unrelated in-flight changes.

## Files To Modify

- `packages/parsers/eslint-config-parser/**`
- `packages/ts/eslint/g3ts-eslint-types/**`
- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`
