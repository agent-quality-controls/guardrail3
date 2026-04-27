Goal

Move the TS ESLint config-content rule files back to the runtime crate root and waive `g3rs-arch/structural-split` for the runtime crate, matching the established rule-package pattern used elsewhere in the repo.

Approach

- Flatten `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/full_config/*` back into `crates/runtime/src/`.
- Update module wiring in `crates/runtime/src/lib.rs` and `crates/runtime/src/run.rs`.
- Rewrite the moved files' imports from `super::...` to crate-root module paths where needed.
- Add a `g3rs-arch/structural-split` waiver for `crates/runtime/Cargo.toml` in `packages/ts/eslint/g3ts-eslint-config-checks/guardrail3-rs.toml`.
- Run:
  - `cargo test`
  - `cargo fmt --check`
  - `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`
  - `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks --family arch --inventory`

Key decisions

- Follow the existing waiver pattern instead of hiding the rule in an arbitrary nested module folder.
  - Other mature rule packages already waive `g3rs-arch/structural-split` for intentionally one-rule-per-file runtime crates.
- Keep the regression commit for the recursive arch rule.
  - That fix was real; the current request is about package organization and waiver policy, not about reverting the rule behavior.

Files to modify

- `.plans/2026-04-20-175201-flatten-ts-eslint-config-runtime-and-waive-structural-split.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/guardrail3-rs.toml`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/*.rs`
- `.worklogs/2026-04-20-175201-flatten-ts-eslint-config-runtime-and-waive-structural-split.md`
