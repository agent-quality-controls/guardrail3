## Goal

Implement the first real `ts/eslint` ingestion and config-check behavior on top of the corrected parser boundary.

End state:

- `g3ts-eslint-ingestion` still selects and parses one active root ESLint config, with tests that prove precedence and parsed handoff.
- `g3ts-eslint-config-checks` runs a first wave of real rules instead of returning an empty vector.
- The implemented rule cut stays generic and avoids package-manager, hexarch, CSS, and app-specific policy.
- Parser and ESLint family roots remain clean under tests, fmt, and `g3rs validate`.

## Approach

1. Add a small config-check runtime support layer for finding construction and document access.
2. Implement the first rule cut:
   - config exists
   - config parseable
   - `@typescript-eslint` plugin active on TS source
   - `projectService: true` on TS source
   - `@typescript-eslint/no-explicit-any` set to error on TS source
   - `no-console` set to error on TS source
3. Add a config-check assertions crate and rule tests that use shared proof helpers.
4. Add ingestion tests for:
   - root config precedence
   - parsed document handoff from a fake workspace through ingestion
5. Run tests, fmt, and `g3rs validate` on:
   - `packages/ts/eslint/g3ts-eslint-types`
   - `packages/ts/eslint/g3ts-eslint-ingestion`
   - `packages/ts/eslint/g3ts-eslint-config-checks`

## Key Decisions

- Keep the first rule cut narrow.
  - Reason: prove the architecture and typed document flow before porting the full legacy ESLint inventory.
- Check effective config state, not raw source strings.
  - Reason: the parser package now exists specifically to avoid string-matching policy checks.
- Keep plugin and rule checks TS-source scoped for wave 1.
  - Reason: JS/script/test carve-outs need deliberate policy decisions and should not be guessed.

## Files To Modify

- `packages/ts/eslint/g3ts-eslint-ingestion/**`
- `packages/ts/eslint/g3ts-eslint-config-checks/**`
