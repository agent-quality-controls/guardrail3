Summary

Added explicit TS-source plugin-stack enforcement to `g3ts-eslint-config-checks`. The ESLint family now errors if the effective TS source config is missing the `unicorn`, `regexp`, or `sonarjs` plugins, without adding any `package.json` dependency checks.

Decisions made

- Kept the check on effective ESLint config only.
  - Reason: plugin declaration in `package.json` is not needed to prove ESLint correctness.
  - The parser already fails closed if ESLint cannot load the config and its plugin imports.
- Used one grouped rule id for the non-TS plugin stack.
  - Reason: this is one baseline slice, not three unrelated policy surfaces.
- Did not add React or React Hooks checks yet.
  - Reason: current probe selection does not distinguish real TSX/UI applicability from fallback TSX probe selection strongly enough.

Key files for context

- `.plans/2026-04-20-183942-ts-eslint-plugin-stack-checks.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_16_plugin_stack.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/cases.rs`

Verification

- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`

Next steps

- If we continue `ts/eslint`, the next high-value slice is probe applicability and parity:
  - TSX-specific policy only when TSX is real, not just a fallback probe
  - optional inventory/error surfacing for non-test relaxations and override drift
- After that, start the next TS family scaffold rather than bloating `ts/eslint` with `package.json` ownership.
