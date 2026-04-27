Goal

Clean the next dirty package roots from the full validate sweep without changing rules:
- `packages/rs/deny/g3rs-deny-config-checks`
- `packages/rs/toolchain/g3rs-toolchain-config-checks`

Approach

- Fix `g3rs-code/ast-10-too-many-use-imports` in `deny-config-checks` by reducing top-level import count in `crates/runtime/src/support/unknown_keys.rs` without changing package behavior.
- Fix the two `g3rs-test/runtime-assertions-split` failures in `toolchain-config-checks` by making sidecar helpers call only their owned production module through `super::super::check`.
- Re-run package-local tests and validate for both package roots.
- Stop immediately if a package-local fix runs into a real rule contradiction instead of more debt.

Key decisions

- Do not widen this pass into the larger `garde/hooks/release/test/topology` clusters yet. Start with the two smallest dirty roots to shrink the remaining surface with low risk.
- Keep the current sidecar test architecture. The toolchain failures look like stale helper imports, not evidence that the rules are wrong.
- Avoid refactoring surrounding files beyond what is needed to clear the reported findings.

Files to modify

- `packages/rs/deny/g3rs-deny-config-checks/crates/runtime/src/support/unknown_keys.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_01_channel_and_components/rule_tests/helpers.rs`
- `packages/rs/toolchain/g3rs-toolchain-config-checks/crates/runtime/src/rs_toolchain_config_02_msrv_consistency/rule_tests/helpers.rs`
