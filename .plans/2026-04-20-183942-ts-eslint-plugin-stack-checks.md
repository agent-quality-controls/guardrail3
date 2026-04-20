Goal

Extend `ts/eslint` with explicit effective-config plugin-stack enforcement for the TS source probe, without adding any `package.json` dependency checks.

Approach

- Add one grouped config rule in `g3ts-eslint-config-checks` for the non-TS plugin stack required by the current baseline:
  - `unicorn`
  - `regexp`
  - `sonarjs`
- Keep the check on effective ESLint config only.
  - Do not read `package.json`.
  - Do not add manifest dependency presence checks.
- Extend runtime test fixtures with a `missing_plugin_stack()` case.
- Update the golden and missing-config expectations for the new rule id.
- Re-run:
  - `cargo test`
  - `cargo fmt --check`
  - `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`

Key decisions

- Use one grouped rule id for the non-TS plugin stack.
  - Reason: this is one baseline slice, not three unrelated policy concepts.
- Do not add `react` or `react-hooks` yet.
  - Reason: current parser inputs do not distinguish real TSX/UI applicability from fallback probe selection well enough.

Files to modify

- `.plans/2026-04-20-183942-ts-eslint-plugin-stack-checks.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_16_plugin_stack.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `.worklogs/2026-04-20-183942-ts-eslint-plugin-stack-checks.md`
