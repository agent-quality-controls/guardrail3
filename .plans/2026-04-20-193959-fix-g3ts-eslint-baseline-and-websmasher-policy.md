Goal

- Fix the `g3ts-eslint-config-checks` rule-id baseline bug so the checker matches real ESLint effective config.
- Bring `/Users/tartakovsky/Projects/websmasher/websmasher/apps/web` and `/Users/tartakovsky/Projects/websmasher/websmasher/apps/landing` into compliance with the current `g3ts` ESLint config family.

Approach

- In `packages/ts/eslint/g3ts-eslint-config-checks`, keep the checker boundary unchanged and correct the baseline inventories to use real effective rule IDs with plugin prefixes.
- Verify the checker package with `cargo test`, `cargo fmt --check`, and `g3rs validate`.
- In `/Users/tartakovsky/Projects/websmasher/websmasher/eslint.config.mjs`, change only the rules that the current `g3ts` findings require:
  - raise `max-lines` for TS sources from `300` to `400`
  - enable the required unicorn rules
  - enable the required regexp rules
  - enable the required sonarjs rules
  - add the TS test carve-out for `@typescript-eslint/no-explicit-any`
- Re-run `g3ts validate` against `apps/web` and `apps/landing`.

Key decisions

- Do not add any `package.json` dependency presence checks.
  - Reason: the parser already fails closed through real ESLint evaluation if plugins cannot load.
- Fix the checker bug in `guardrail3` before trusting the app findings.
  - Reason: the earlier false negatives on `@typescript-eslint/*` and `import-x/*` proved the checker was wrong.
- Change the monorepo root ESLint config, not only the app-local entrypoints.
  - Reason: the remaining findings are all in the shared effective config surface.

Files to modify

- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/cases.rs`
- `/Users/tartakovsky/Projects/websmasher/websmasher/eslint.config.mjs`
