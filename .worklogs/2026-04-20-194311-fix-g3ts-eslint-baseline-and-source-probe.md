Summary

Fixed two real `g3ts` ESLint checker bugs while bringing external TS apps under the new validator. First, the config-check baseline inventory used stripped rule IDs instead of the real effective ESLint IDs for `@typescript-eslint/*` and `import-x/*`. Second, ingestion could bind the `TsSource` probe to `scripts/*.ts`, which caused false failures when script-only carve-outs were present.

Decisions made

- Fixed the rule-id mismatch in the checker baseline rather than weakening the rules.
  - Reason: `eslint --print-config` proved the apps were already enforcing several prefixed rules that `g3ts` incorrectly reported missing.
- Fixed the `TsSource` bug in ingestion probe selection, not in config checks.
  - Reason: the wrong file was being evaluated. `TsSource` must prefer real application source over build scripts.
- Kept the app-facing ESLint changes in the external repo separate from the checker bug fix here.
  - Reason: the checker must be trustworthy before app findings are acted on.

Key files for context

- `.plans/2026-04-20-193959-fix-g3ts-eslint-baseline-and-websmasher-policy.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select_tests/cases.rs`

Verification

- `cargo test --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-config-checks`
- `cargo test --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml`
- `cargo run -q --manifest-path apps/guardrail3-rs/Cargo.toml -p g3rs -- validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/web --family eslint`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family eslint`

Next steps

- Add the next TS families only after keeping `g3ts` findings aligned with real effective config semantics.
- If another TS app has only script `.ts` files and `.tsx` source, the new `TsSource` precedence should cover it. Keep that regression test.
