## Summary

Fixed the `ts/tsconfig` family so explicit target-root validation accepts `tsconfig.base.json` when a root `tsconfig.json` is absent. This closes the live `websmasher` repo-root false positive without widening the family into repo-global dual-surface traversal.

## Decisions made

- Fixed the root-surface selection in ingestion.
  - Why: the current `g3ts validate --path <root>` runner validates one explicit root at a time, so the right correction is to select the root config file for that target, not to force a fake repo-root `tsconfig.json`.
  - Chosen precedence:
    - `tsconfig.json`
    - `tsconfig.base.json`

- Kept the public family input unchanged.
  - Why: the bug was in root config selection and root-file-specific messaging, not in the family boundary itself.
  - Rejected: adding a broader dual-root family state just to fix this explicit-target-root case.

- Proved the bug with unit tests first.
  - Added ingestion coverage for:
    - neither root surface exists
    - fallback to `tsconfig.base.json`
    - precedence of `tsconfig.json` over `tsconfig.base.json`
  - Added config-check coverage that `tsconfig.base.json` is accepted as the selected root surface.

## Key files for context

- `.plans/2026-04-21-130931-ts-foundation-attack-and-tightening.md`
- `packages/ts/tsconfig/g3ts-tsconfig-ingestion/crates/runtime/src/run.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-ingestion/crates/runtime/src/run_tests/cases.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/ts_tsconfig_config_01_exists.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/ts_tsconfig_config_04_extends_or_inline.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/run_tests/helpers.rs`
- `packages/ts/tsconfig/g3ts-tsconfig-config-checks/crates/runtime/src/run_tests/cases.rs`

## Verification

- `cargo test -q --manifest-path packages/ts/tsconfig/g3ts-tsconfig-ingestion/Cargo.toml --workspace`
- `cargo test -q --manifest-path packages/ts/tsconfig/g3ts-tsconfig-config-checks/Cargo.toml --workspace`
- `cargo test -q --manifest-path apps/guardrail3-ts/Cargo.toml --workspace`
- `cargo fmt --all --check --manifest-path packages/ts/tsconfig/g3ts-tsconfig-ingestion/Cargo.toml`
- `cargo fmt --all --check --manifest-path packages/ts/tsconfig/g3ts-tsconfig-config-checks/Cargo.toml`
- `g3rs validate --path packages/ts/tsconfig/g3ts-tsconfig-ingestion`
- `g3rs validate --path packages/ts/tsconfig/g3ts-tsconfig-config-checks`
- `g3rs validate --path apps/guardrail3-ts`
- `cargo run -q --manifest-path apps/guardrail3-ts/Cargo.toml -p g3ts -- validate --path /Users/tartakovsky/Projects/websmasher/websmasher`

## Next steps

- The `websmasher` repo-root target now fails only on `ts/eslint` root policy.
- If continuing target cleanup, fix the repo-root ESLint config to use `projectService: true` and satisfy the type-safety baseline for TS and TSX probes.
