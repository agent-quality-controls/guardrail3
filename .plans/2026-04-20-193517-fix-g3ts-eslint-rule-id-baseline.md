Goal

Fix false negatives in `g3ts-eslint-config-checks` by aligning the baseline rule inventory with real ESLint effective rule IDs.

Approach

- Update the baseline lists in `g3ts-eslint-config-checks` to use the same rule IDs that `eslint --print-config` returns:
  - `@typescript-eslint/...`
  - `import-x/...`
- Update the config-check fixtures and assertions so the golden path uses those real rule IDs.
- Re-run the package tests and then re-run `g3ts validate` on the `websmasher` app roots to see the remaining real config drift.

Key decisions

- Fix the checker, not the apps, for the false-negative block.
  - Reason: the apps already enforce many of those rules in real ESLint output.
- Use the current effective-config shape as the source of truth for rule IDs.
  - Reason: `g3ts` checks effective config, not source-text aliases.

Files to modify

- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
