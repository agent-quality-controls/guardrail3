Summary

Flattened `g3ts-eslint-config-checks` runtime rules back to the crate root and added the standard `g3rs-arch/structural-split` structural-split waiver for the runtime crate. The package now matches the existing rule-package pattern used elsewhere in guardrail3 and validates clean again.

Decisions made

- Moved the grouped ESLint config rule files back to `crates/runtime/src/`.
  - Reason: the nested `full_config` folder was only there to dodge the structural-split rule.
  - The established repo pattern for intentional one-rule-per-file runtime crates is a waiver, not hiding files in a subtree.
- Added only the runtime-crate waiver.
  - Reason: that is the actual intentionally dense rule crate.
  - The assertions crate did not need a new waiver here.
- Kept the recursive arch rule and the mixed-workspace regression intact.
  - Reason: those fixes were real.
  - This change is package policy, not a rollback of arch behavior.

Key files for context

- `.plans/2026-04-20-175201-flatten-ts-eslint-config-runtime-and-waive-structural-split.md`
- `packages/ts/eslint/g3ts-eslint-config-checks/guardrail3-rs.toml`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/lib.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/run.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/support.rs`
- `packages/ts/eslint/g3ts-eslint-config-checks/crates/runtime/src/baseline.rs`

Verification

- `cargo fmt --all --check --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml`
- `cargo test -q --manifest-path packages/ts/eslint/g3ts-eslint-config-checks/Cargo.toml --workspace`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks`
- `g3rs validate --path packages/ts/eslint/g3ts-eslint-config-checks --family arch --inventory`

Next steps

- If we keep building TS families, use the same rule-package policy consistently: root-level one-rule-per-file runtimes with explicit structural waivers when the density is intentional.
- Only introduce a deeper module subtree when it reflects a real semantic split, not as a shape workaround.
