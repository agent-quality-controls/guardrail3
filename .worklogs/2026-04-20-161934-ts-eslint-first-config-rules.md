## Summary

Implemented the first real `ts/eslint` ingestion and config-check behavior on top of the new ESLint parser package. The family now has live config findings, shared assertions, and tests proving both root-config selection and parsed document handoff.

## Decisions made

- Kept the first ESLint rule cut narrow.
  - Implemented only existence, parseability, TS plugin presence, `projectService`, `no-explicit-any`, and `no-console`.
  - Rejected porting the full legacy inventory in one pass because that would mix stable generic baseline work with unresolved package/app/profile-specific policy.
- Kept config checks on the parsed effective-config document.
  - Rejected raw config string checks because the parser package now exists specifically to avoid that boundary leak.
- Added shared assertions crates for both `g3ts-eslint-config-checks` and `g3ts-eslint-ingestion`.
  - Reason: internal tests need the same proof surface the validator expects, and grouped ad hoc test assertions immediately failed guardrails.
- Added ingestion tests only for precedence and parsed handoff.
  - Reason: enough to lock the ingestion boundary without bloating the package before more lanes exist.

## Key files for context

- `.plans/2026-04-20-160416-ts-eslint-first-config-rules.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_01_exists.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_02_parseable.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_03_ts_plugin_present.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_04_project_service_enabled.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_05_no_explicit_any_error.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/ts_eslint_config_06_no_console_error.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/assertions/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/eslint/g3ts-eslint-ingestion/crates/runtime/src/select_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/parsers/eslint-config-parser/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-types/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-ingestion/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml`
- `g3rs validate --path packages/parsers/eslint-config-parser`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-types`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-ingestion`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`

## Next steps

- Expand `ts/eslint` from the first six generic checks into the next wave of typed-lint baseline rules.
- Decide when plugin/package-presence checks move into `ts/package` instead of remaining in `ts/eslint`.
- Start `ts/tsconfig` once the ESLint baseline cut is stable enough not to churn its shared assumptions immediately.
